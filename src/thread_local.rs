#![allow(dead_code)]

//! Per-object `ThreadLocal` abstraction
//!
//! Borrowed from https://crates.io/crates/thread-local-object, without
//! `unsafe_any` and with `Send` and `Sync` impls added.

use std::{
    any::Any,
    cell::RefCell,
    collections::{
        hash_map,
        HashMap,
    },
    marker::PhantomData,
    mem,
    sync::atomic::{
        AtomicU64,
        Ordering,
    },
};

/// A thread local variable wrapper.
pub struct ThreadLocal<T: 'static> {
    id: u64,
    _p: PhantomData<fn() -> T>,
}

thread_local! {
    static VALUES: RefCell<HashMap<u64, Box<dyn Any>>> = RefCell::new(HashMap::new());
}
static NEXT_ID: AtomicU64 = AtomicU64::new(0);

// if IDs ever wrap around we'll run into soundness issues with downcasts, so panic if we're out of
// IDs. On 64 bit platforms this can literally never happen (it'd take 584 years even if you were
// generating a billion IDs per second), but is more realistic a concern on 32 bit platforms.
//
// FIXME use AtomicU64 when it's stable
fn next_id() -> u64 {
    NEXT_ID.fetch_add(1, Ordering::SeqCst)
}

impl<T> Default for ThreadLocal<T> {
    fn default() -> Self {
        ThreadLocal::new()
    }
}

impl<T: 'static> ThreadLocal<T> {
    /// Creates a new `ThreadLocal` with no values for any threads.
    ///
    /// # Panics
    ///
    /// Panics if more than `u64::max_value()` `ThreadLocal` objects have already been created.
    pub fn new() -> ThreadLocal<T> {
        ThreadLocal {
            id: next_id(),
            _p: PhantomData,
        }
    }

    /// Sets this thread's value, returning the previous value if present.
    ///
    /// # Panics
    ///
    /// Panics if called from within the execution of a closure provided to another method on this
    /// value.
    pub fn set(&self, value: T) -> Option<T> {
        self.entry(|e| match e {
            Entry::Occupied(mut e) => Some(e.insert(value)),
            Entry::Vacant(e) => {
                e.insert(value);
                None
            }
        })
    }

    /// Removes this thread's value, returning it if it existed.
    ///
    /// # Panics
    ///
    /// Panics if called from within the execution of a closure provided to another method on this
    /// value.
    pub fn remove(&self) -> Option<T> {
        VALUES.with(|v| {
            v.borrow_mut()
                .remove(&self.id)
                .and_then(|v| v.downcast::<T>().ok())
                .map(|b| *b)
        })
    }

    /// Passes a handle to the current thread's value to a closure for in-place manipulation.
    ///
    /// The closure is required for the same soundness reasons it is required for the standard
    /// library's `thread_local!` values.
    ///
    /// # Panics
    ///
    /// Panics if called from within the execution of a closure provided to another method on this
    /// value.
    pub fn entry<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Entry<T>) -> R,
    {
        VALUES.with(|v| {
            let mut v = v.borrow_mut();
            let entry = match v.entry(self.id) {
                hash_map::Entry::Occupied(e) => Entry::Occupied(OccupiedEntry(e, PhantomData)),
                hash_map::Entry::Vacant(e) => Entry::Vacant(VacantEntry(e, PhantomData)),
            };
            f(entry)
        })
    }

    /// Passes a mutable reference to the current thread's value to a closure.
    ///
    /// The closure is required for the same soundness reasons it is required for the standard
    /// library's `thread_local!` values.
    ///
    /// # Panics
    ///
    /// Panics if called from within the execution of a closure passed to `entry` or `get_mut` on
    /// this value.
    pub fn get<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Option<&T>) -> R,
    {
        VALUES.with(|v| {
            let v = v.borrow();
            let value = v.get(&self.id).and_then(|v| v.downcast_ref());
            f(value)
        })
    }

    /// Passes a mutable reference to the current thread's value to a closure.
    ///
    /// The closure is required for the same soundness reasons it is required for the standard
    /// library's `thread_local!` values.
    ///
    /// # Panics
    ///
    /// Panics if called from within the execution of a closure provided to another method on this
    /// value.
    pub fn get_mut<F, R>(&self, f: F) -> R
    where
        F: FnOnce(Option<&mut T>) -> R,
    {
        VALUES.with(|v| {
            let mut v = v.borrow_mut();
            let value = v.get_mut(&self.id).and_then(|v| v.downcast_mut());
            f(value)
        })
    }
}

impl<T> ThreadLocal<T>
where
    T: 'static + Clone,
{
    /// Returns a copy of the current thread's value.
    ///
    /// # Panics
    ///
    /// Panics if called from within the execution of a closure passed to `entry` or `get_mut` on
    /// this value.
    pub fn get_cloned(&self) -> Option<T> {
        VALUES.with(|v| {
            v.borrow()
                .get(&self.id)
                .and_then(|v| v.downcast_ref::<T>().cloned())
        })
    }
}

/// A view into a thread's slot in a `ThreadLocal` that may be empty.
pub enum Entry<'a, T: 'static> {
    /// An occupied entry.
    Occupied(OccupiedEntry<'a, T>),
    /// A vacant entry.
    Vacant(VacantEntry<'a, T>),
}

impl<'a, T: 'static> Entry<'a, T> {
    pub fn get_mut(&'a mut self) -> Option<&'a mut T> {
        match self {
            Entry::Occupied(v) => Some(v.get_mut()),
            _ => None,
        }
    }
    pub fn get(&'a self) -> Option<&'a T> {
        match self {
            Entry::Occupied(v) => Some(v.get()),
            _ => None,
        }
    }

    pub fn insert(self, v: T) -> (&'a mut T, Option<T>) {
        match self {
            Entry::Occupied(mut e) => {
                let prev = e.insert(v);
                (e.into_mut(), Some(prev))
            }
            Entry::Vacant(e) => {
                let curr = e.insert(v);
                (curr, None)
            }
        }
    }

    /// Ensures a value is in the entry by inserting the default if it is empty, and returns a
    /// mutable reference to the value in the entry.
    pub fn or_default(self) -> &'a mut T
    where
        T: Default,
    {
        #[allow(clippy::unwrap_or_default)]
        self.or_insert_with(Default::default)
    }

    /// Ensures a value is in the entry by inserting the default if it is empty, and returns a
    /// mutable reference to the value in the entry.
    pub fn or_insert(self, default: T) -> &'a mut T {
        match self {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(default),
        }
    }

    /// Ensures a value is in the entry by inserting the result of the default function if it is
    /// empty, and returns a mutable reference to the value in the entry.
    pub fn or_insert_with<F>(self, default: F) -> &'a mut T
    where
        F: FnOnce() -> T,
    {
        match self {
            Entry::Occupied(e) => e.into_mut(),
            Entry::Vacant(e) => e.insert(default()),
        }
    }
}

/// A view into a thread's slot in a `ThreadLocal` which is occupied.
pub struct OccupiedEntry<'a, T: 'static>(
    hash_map::OccupiedEntry<'a, u64, Box<dyn Any>>,
    PhantomData<&'a mut T>,
);

impl<'a, T: 'static> OccupiedEntry<'a, T> {
    /// Returns a reference to the value in the entry.
    pub fn get(&self) -> &T {
        self.0.get().downcast_ref().unwrap()
    }

    /// Returns a mutable reference to the value in the entry.
    pub fn get_mut(&mut self) -> &mut T {
        self.0.get_mut().downcast_mut().unwrap()
    }

    /// Converts an `OccupiedEntry` into a mutable reference to the value in the entry with a
    /// lifetime bound of the slot itself.
    pub fn into_mut(self) -> &'a mut T {
        self.0.into_mut().downcast_mut().unwrap()
    }

    /// Sets the value of the entry, and returns the entry's old value.
    pub fn insert(&mut self, value: T) -> T {
        mem::replace(self.get_mut(), value)
    }

    /// Takes the value out of the entry, and returns it.
    pub fn remove(self) -> T {
        *self.0.remove().downcast().unwrap()
    }
}

/// A view into a thread's slot in a `ThreadLocal` which is unoccupied.
pub struct VacantEntry<'a, T: 'static>(
    hash_map::VacantEntry<'a, u64, Box<dyn Any>>,
    PhantomData<&'a mut T>,
);

impl<'a, T: 'static> VacantEntry<'a, T> {
    /// Sets the value of the entry, and returns a mutable reference to it.
    pub fn insert(self, value: T) -> &'a mut T {
        self.0.insert(Box::new(value)).downcast_mut().unwrap()
    }
}
