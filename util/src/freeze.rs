use std::fmt::Debug;
use std::ops::Deref;
use std::sync::Arc;
use std::{cell::RefCell, marker::PhantomData, mem, rc::Rc};

use parking_lot::RwLock;
use thiserror::Error;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Error)]
pub enum AccessError {
    #[error("frozen value accessed outside of enclosing scope")]
    Expired,
    #[error("already borrowed incompatibly")]
    BadBorrow,
}

/// Trait for accessing shared data, mutably or otherwise.
pub trait Access<T> {
    /// Attempt to access the wrapped value immutably.
    /// Should return `None` if it is already being accessed mutably.
    fn try_with<F: FnOnce(&T) -> R, R>(&self, f: F) -> Option<R>;
    /// Attempt to access the wrapped value mutably.
    /// Should return `None` if it is already being accessed mutably or
    /// immutably.
    fn try_with_mut<F: FnOnce(&mut T) -> R, R>(&self, f: F) -> Option<R>;

    /// Convenience method for calling `.try_with(f).unwrap()`
    fn with<F: FnOnce(&T) -> R, R>(&self, f: F) -> R {
        self.try_with(f).unwrap()
    }
    /// Convenience method for calling `.try_with_mut(f).unwrap()`
    fn with_mut<F: FnOnce(&mut T) -> R, R>(&self, f: F) -> R {
        self.try_with_mut(f).unwrap()
    }
}

impl<T> Access<T> for RefCell<T> {
    fn try_with<F: FnOnce(&T) -> R, R>(&self, f: F) -> Option<R> {
        self.try_borrow().ok().as_deref().map(f)
    }
    fn try_with_mut<F: FnOnce(&mut T) -> R, R>(&self, f: F) -> Option<R> {
        self.try_borrow_mut().ok().as_deref_mut().map(f)
    }
}

impl<T> Access<T> for RwLock<T> {
    fn try_with<F: FnOnce(&T) -> R, R>(&self, f: F) -> Option<R> {
        Some(f(&*self.read()))
    }
    fn try_with_mut<F: FnOnce(&mut T) -> R, R>(&self, f: F) -> Option<R> {
        Some(f(&mut *self.write()))
    }
}

/// Trait for producing clonable "handles" to data with interior mutability.
pub trait Handle {
    type Ptr<T>: Deref<Target = Self::Access<T>>;
    type Access<T>: Access<T>;

    fn clone<T>(ptr: &Self::Ptr<T>) -> Self::Ptr<T>;
    fn new<T>(val: T) -> Self::Ptr<T>;
}

/// Handle implementation for local-only data.
/// `!Send + !Sync`, but lower overhead than `SendHandle`.
pub struct LocalHandle;

impl Handle for LocalHandle {
    type Ptr<T> = Rc<RefCell<T>>;
    type Access<T> = RefCell<T>;

    fn clone<T>(ptr: &Self::Ptr<T>) -> Self::Ptr<T> {
        Rc::clone(ptr)
    }

    fn new<T>(val: T) -> Self::Ptr<T> {
        Rc::new(RefCell::new(val))
    }
}

/// Handle implementation for data that needs to be `Send + Sync`.
pub struct SendHandle;

impl Handle for SendHandle {
    type Ptr<T> = Arc<RwLock<T>>;
    type Access<T> = RwLock<T>;

    fn clone<T>(ptr: &Self::Ptr<T>) -> Self::Ptr<T> {
        Arc::clone(ptr)
    }

    fn new<T>(val: T) -> Self::Ptr<T> {
        Arc::new(RwLock::new(val))
    }
}

/// Safely erase a lifetime from a value and temporarily store it in a shared handle.
///
/// Works by providing only limited access to the held value within an enclosing call to
/// `FrozenScope::scope`. All cloned handles will refer to the same underlying value with its
/// lifetime erased.
///
/// Useful for passing non-'static values into things that do not understand the Rust lifetime
/// system and need unrestricted sharing, such as scripting languages.
pub struct Frozen<F: for<'f> Freeze<'f>, M: Handle = LocalHandle> {
    inner: M::Ptr<Option<<F as Freeze<'static>>::Frozen>>,
}

pub trait Freeze<'f>: 'static {
    type Frozen: 'f;
}

pub struct DynFreeze<T: ?Sized>(PhantomData<T>);

impl<'f, T: ?Sized + for<'a> Freeze<'a>> Freeze<'f> for DynFreeze<T> {
    type Frozen = <T as Freeze<'f>>::Frozen;
}

#[macro_export]
#[doc(hidden)]
macro_rules! __scripting_Freeze {
    ($f:lifetime => $frozen:ty) => {
        $crate::freeze::DynFreeze::<
            dyn for<$f> $crate::freeze::Freeze<$f, Frozen = $frozen>,
        >
    };
    ($frozen:ty) => {
        $crate::freeze::Freeze!['freeze => $frozen]
    };
}

pub use crate::__scripting_Freeze as Freeze;

impl<F: for<'a> Freeze<'a>, M: Handle> Clone for Frozen<F, M> {
    fn clone(&self) -> Self {
        Self {
            inner: M::clone(&self.inner),
        }
    }
}

impl<F: for<'a> Freeze<'a>, M: Handle> Default for Frozen<F, M> {
    fn default() -> Self {
        Self {
            inner: M::new(None),
        }
    }
}

impl<F: for<'a> Freeze<'a>, M: Handle> Frozen<F, M> {
    /// Creates a new *invalid* `Frozen` handle.
    pub fn new() -> Self {
        Self::default()
    }

    pub fn in_scope<R>(value: <F as Freeze<'_>>::Frozen, cb: impl FnOnce(Self) -> R) -> R {
        let f = Self::new();
        let p = f.clone();
        FrozenScope::new().freeze(&f, value).scope(move || cb(p))
    }

    /// Returns true if this value is currently set by an enclosing `FrozenScope::scope`.
    pub fn is_valid(&self) -> bool {
        if let Some(b) = self.inner.try_with(|inner| inner.is_some()) {
            b
        } else {
            true
        }
    }

    pub fn try_with<R>(
        &self,
        f: impl for<'f> FnOnce(&<F as Freeze<'f>>::Frozen) -> R,
    ) -> Result<R, AccessError> {
        let res = self.inner.try_with(|inner| inner.as_ref().map(f));
        match res {
            None => Err(AccessError::BadBorrow),
            Some(None) => Err(AccessError::Expired),
            Some(Some(v)) => Ok(v),
        }
    }

    /// # Panics
    /// Panics if this handle is not currently valid or if the held value is already borrowed
    /// mutably.
    pub fn with<R>(&self, f: impl for<'f> FnOnce(&<F as Freeze<'f>>::Frozen) -> R) -> R {
        self.try_with(f).unwrap()
    }

    pub fn try_with_mut<R>(
        &self,
        f: impl for<'f> FnOnce(&mut <F as Freeze<'f>>::Frozen) -> R,
    ) -> Result<R, AccessError> {
        let res = self.inner.try_with_mut(|inner| inner.as_mut().map(f));
        match res {
            None => Err(AccessError::BadBorrow),
            Some(None) => Err(AccessError::Expired),
            Some(Some(v)) => Ok(v),
        }
    }

    /// # Panics
    /// Panics if this handle is not currently valid or if the held value is already borrowed.
    pub fn with_mut<R>(&self, f: impl for<'f> FnOnce(&mut <F as Freeze<'f>>::Frozen) -> R) -> R {
        self.try_with_mut(f).unwrap()
    }
}

/// Struct that enables setting the contents of multiple `Frozen<F>` handles for the body of a
/// single callback.
pub struct FrozenScope<D = ()>(D);

impl Default for FrozenScope<()> {
    fn default() -> Self {
        FrozenScope(())
    }
}

impl FrozenScope<()> {
    pub fn new() -> Self {
        Self(())
    }
}

impl<D: DropGuard> FrozenScope<D> {
    /// Sets the given frozen value for the duration of the `FrozenScope::scope` call.
    pub fn freeze<'h, 'f, F: for<'a> Freeze<'a>, M: Handle>(
        self,
        handle: &'h Frozen<F, M>,
        value: <F as Freeze<'f>>::Frozen,
    ) -> FrozenScope<(FreezeGuard<'h, 'f, F, M>, D)> {
        FrozenScope((
            FreezeGuard {
                value: Some(value),
                handle,
            },
            self.0,
        ))
    }

    /// Inside this call, all of the handles set with `FrozenScope::freeze` will be valid and can be
    /// accessed with `Frozen::with` and `Frozen::with_mut`. The provided handles (and all clones of
    /// them) are invalidated before this call to `FrozenScope::scope` returns.
    ///
    /// # Panics
    /// Panics if any of the provided handles are already set inside another, outer
    /// `FrozenScope::scope` call or if any handles were set with `FrozenScope::freeze` more than
    /// once. The given handles must be used with only one `FrozenScope` at a time.
    pub fn scope<R>(mut self, cb: impl FnOnce() -> R) -> R {
        // SAFETY: Safety depends on a few things...
        //
        // 1) We turn non-'static values into a 'static ones, outside code should never be able to
        //    observe the held 'static value, because it lies about the true lifetime.
        //
        // 2) The only way to interact with the held 'static value is through `Frozen::[try_]with`
        //    and `Frozen::[try_]with_mut`, both of which require a callback that works with the
        //    frozen type for *any* lifetime. This interaction is safe because the callbacks must
        //    work for any lifetime, so they must work with the lifetime we have erased.
        //
        // 3) The 'static `Frozen<F>` handles must have their values unset before the body of
        //    this function ends because we only know they live for at least the body of this
        //    function, and we use drop guards for this.
        unsafe {
            self.0.set();
        }
        let r = cb();
        drop(self.0);
        r
    }
}

pub trait DropGuard {
    // Sets the held `Frozen` handle to the held value.
    //
    // SAFETY:
    // This is unsafe because the `Frozen` handle can now be used to access the value independent of
    // its lifetime and the borrow checker cannot check this.
    //
    // Implementers of this trait *must* unset the handle's held value when the value is dropped.
    //
    // Users of this trait *must* drop it before the lifetime of the held value ends.
    unsafe fn set(&mut self);
}

impl DropGuard for () {
    unsafe fn set(&mut self) {}
}

impl<A: DropGuard, B: DropGuard> DropGuard for (A, B) {
    unsafe fn set(&mut self) {
        self.0.set();
        self.1.set();
    }
}

pub struct FreezeGuard<'h, 'f, F: for<'a> Freeze<'a>, M: Handle = LocalHandle> {
    value: Option<<F as Freeze<'f>>::Frozen>,
    handle: &'h Frozen<F, M>,
}

impl<'h, 'f, F: for<'a> Freeze<'a>, M: Handle> Drop for FreezeGuard<'h, 'f, F, M> {
    fn drop(&mut self) {
        if self
            .handle
            .inner
            .try_with_mut(|inner| {
                *inner = None;
            })
            .is_none()
        {
            // This should not be possible to trigger safely, because users cannot hold
            // `Ref` or `RefMut` handles from the inner `RefCell` in the first place,
            // and `Frozen` does not implement Send so we can't be in the body of
            // `Frozen::with[_mut]` in another thread. However, if it somehow happens that
            // we cannot drop the held value, this means that there is a live reference to
            // it somewhere so we are forced to abort the process.
            eprintln!("impossible! freeze lock held during drop guard, aborting!");
            std::process::abort()
        }
    }
}

impl<'h, 'f, F: for<'a> Freeze<'a>, M: Handle> DropGuard for FreezeGuard<'h, 'f, F, M> {
    unsafe fn set(&mut self) {
        assert!(
            !self.handle.is_valid(),
            "handle already used in another `FrozenScope::scope` call"
        );
        self.handle.inner.with_mut(|inner| {
            *inner = Some(mem::transmute::<
                <F as Freeze<'f>>::Frozen,
                <F as Freeze<'static>>::Frozen,
            >(self.value.take().unwrap()))
        });
    }
}

#[cfg(test)]
mod tests {
    use std::{
        sync::mpsc::{channel, Sender},
        thread,
    };

    use super::*;

    fn assert_send<T: Send>() {}
    fn assert_sync<T: Sync>() {}

    #[allow(dead_code)]
    fn asserts() {
        assert_send::<Frozen<Freeze![&'freeze ()], SendHandle>>();
        assert_sync::<Frozen<Freeze![&'freeze ()], SendHandle>>();
    }

    #[test]
    fn test_freeze_works() {
        struct F<'a>(&'a i32);

        let i = 4;
        Frozen::<Freeze![F<'freeze>]>::in_scope(F(&i), |f| {
            f.with(|f| {
                assert_eq!(*f.0, 4);
            });
        });
    }

    #[test]
    fn test_multithread_freeze_works() {
        struct F<'a>(&'a i32);
        type FrozenF = Frozen<Freeze![F<'freeze>], SendHandle>;
        let i = 4;

        let (tx, rx) = channel::<(Sender<i32>, FrozenF)>();
        thread::spawn(move || {
            let (tx, msg) = rx.recv().unwrap();
            tx.send(msg.with(|v| v.0 + 1)).unwrap();
        });

        Frozen::<Freeze![F<'freeze>], SendHandle>::in_scope(F(&i), |f| {
            let (resp_tx, resp_rx) = channel::<i32>();
            tx.send((resp_tx, f)).unwrap();
            assert_eq!(resp_rx.recv().unwrap(), 5);
        });
    }

    #[test]
    fn test_freeze_expires() {
        struct F<'a>(&'a i32);

        type FrozenF = Frozen<Freeze![F<'freeze>]>;

        let mut outer: Option<FrozenF> = None;

        let i = 4;
        FrozenF::in_scope(F(&i), |f| {
            outer = Some(f.clone());
        });

        assert_eq!(
            outer.unwrap().try_with(|f| {
                assert_eq!(*f.0, 4);
            }),
            Err(AccessError::Expired)
        );
    }
}
