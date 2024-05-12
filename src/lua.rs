use std::ops;

use gc_arena::{metrics::Metrics, Arena, Collect, CollectionPhase, Mutation, Root, Rootable};

use crate::{
    finalizers::Finalizers,
    registry::{Fetchable, Stashable},
    stdlib::{load_base, load_coroutine, load_io, load_math, load_string, load_table},
    string::InternedStringSet,
    Error, FromMultiValue, Fuel, IntoValue, InvalidTableKey, Registry, Singleton, StashedExecutor,
    StaticError, String, Table, Value,
};

#[derive(Copy, Clone)]
pub struct Context<'gc> {
    mutation: &'gc Mutation<'gc>,
    state: &'gc State<'gc>,
}

impl<'gc> Context<'gc> {
    pub fn globals(self) -> Table<'gc> {
        self.state.globals
    }

    pub fn registry(self) -> Registry<'gc> {
        self.state.registry
    }

    pub fn interned_strings(self) -> InternedStringSet<'gc> {
        self.state.strings
    }

    pub fn finalizers(self) -> Finalizers<'gc> {
        self.state.finalizers
    }

    /// Calls `ctx.globals().set(ctx, key, value)`.
    pub fn set_global<K: IntoValue<'gc>, V: IntoValue<'gc>>(
        self,
        key: K,
        value: V,
    ) -> Result<Value<'gc>, InvalidTableKey> {
        self.state.globals.set(self, key, value)
    }

    /// Calls `ctx.globals().get(ctx, key)`.
    pub fn get_global<K: IntoValue<'gc>>(self, key: K) -> Value<'gc> {
        self.state.globals.get(self, key)
    }

    /// Calls `ctx.registry().singleton::<S>(ctx)`.
    pub fn singleton<S>(self) -> &'gc Root<'gc, S>
    where
        S: for<'a> Rootable<'a>,
        Root<'gc, S>: Singleton<'gc>,
    {
        self.state.registry.singleton::<S>(self)
    }

    /// Calls `ctx.registry().stash(ctx, s)`.
    pub fn stash<S: Stashable<'gc>>(self, s: S) -> S::Stashed {
        self.state.registry.stash(&self, s)
    }

    /// Calls `ctx.registry().fetch(f)`.
    pub fn fetch<F: Fetchable<'gc>>(self, f: &F) -> F::Fetched {
        self.state.registry.fetch(f)
    }

    /// Calls `ctx.interned_strings().intern(&ctx, s)`.
    pub fn intern(self, s: &[u8]) -> String<'gc> {
        self.state.strings.intern(&self, s)
    }

    /// Calls `ctx.interned_strings().intern_static(&ctx, s)`.
    pub fn intern_static(self, s: &'static [u8]) -> String<'gc> {
        self.state.strings.intern_static(&self, s)
    }
}

impl<'gc> ops::Deref for Context<'gc> {
    type Target = Mutation<'gc>;

    fn deref(&self) -> &Self::Target {
        self.mutation
    }
}

pub struct Lua {
    arena: Arena<Rootable![State<'_>]>,
    finalized: bool,
}

impl Default for Lua {
    fn default() -> Self {
        Lua::core()
    }
}

impl Lua {
    /// Create a new `Lua` instance with no parts of the stdlib loaded.
    pub fn empty() -> Self {
        Lua {
            arena: Arena::<Rootable![State<'_>]>::new(|mc| State::new(mc)),
            finalized: false,
        }
    }

    /// Create a new `Lua` instance with the core stdlib loaded.
    pub fn core() -> Self {
        let mut lua = Self::empty();
        lua.load_core();
        lua
    }

    /// Create a new `Lua` instance with all of the stdlib loaded.
    pub fn full() -> Self {
        let mut lua = Lua::core();
        lua.load_io();
        lua
    }

    /// Load the core parts of the stdlib that do not allow performing any I/O.
    ///
    /// Calls:
    ///   - `load_base`
    ///   - `load_coroutine`
    ///   - `load_math`
    ///   - `load_string`
    ///   - `load_table`
    pub fn load_core(&mut self) {
        self.enter(|ctx| {
            load_base(ctx);
            load_coroutine(ctx);
            load_math(ctx);
            load_string(ctx);
            load_table(ctx);
        })
    }

    /// Load the parts of the stdlib that allow I/O.
    pub fn load_io(&mut self) {
        self.enter(|ctx| {
            load_io(ctx);
        })
    }

    /// Size of all memory used by this Lua context.
    ///
    /// This is equivalent to `self.gc_metrics().total_allocation()`. This counts all `Gc` allocated
    /// memory and also all data Lua datastructures held inside `Gc`, as they are tracked as
    /// "external allocations" in `gc-arena`.
    pub fn total_memory(&self) -> usize {
        self.gc_metrics().total_allocation()
    }

    /// Finish the current collection cycle completely, calls `gc_arena::Arena::collect_all()`.
    pub fn gc_collect(&mut self) {
        if !self.finalized {
            self.arena.mark_all().unwrap().finalize(|fc, root| {
                root.finalizers.finalize(fc);
            });
        }

        self.arena.collect_all();
        assert!(self.arena.collection_phase() == CollectionPhase::Sleeping);
        self.finalized = false;
    }

    pub fn gc_metrics(&self) -> &Metrics {
        self.arena.metrics()
    }

    /// Enter the garbage collection arena and perform some operation.
    ///
    /// In order to interact with Lua or do any useful work with Lua values, you must do so from
    /// *within* the garbage collection arena. All values branded with the `'gc` branding lifetime
    /// must forever live *inside* the arena, and cannot escape it.
    ///
    /// Garbage collection takes place *in-between* calls to `Lua::enter`, no garbage will be
    /// collected cocurrently with accessing the arena.
    ///
    /// Automatically triggers garbage collection before returning if the allocation debt is larger
    /// than a small constant.
    pub fn enter<F, T>(&mut self, f: F) -> T
    where
        F: for<'gc> FnOnce(Context<'gc>) -> T,
    {
        const COLLECTOR_GRANULARITY: f64 = 1024.0;

        let r = self.arena.mutate(move |mc, state| f(state.ctx(mc)));
        if self.arena.metrics().allocation_debt() > COLLECTOR_GRANULARITY {
            if self.finalized {
                self.arena.collect_debt();

                if self.arena.collection_phase() == CollectionPhase::Sleeping {
                    self.finalized = false;
                }
            } else {
                if let Some(marked) = self.arena.mark_debt() {
                    marked.finalize(|fc, root| {
                        root.finalizers.finalize(fc);
                    });
                    // Immediately transition to `CollectionPhase::Collecting`.
                    self.arena.mark_all().unwrap().start_collecting();
                    self.finalized = true;
                }
            }
        }
        r
    }

    /// A version of `Lua::enter` that expects failure and also automatically converts `Error` types
    /// into `StaticError`, allowing the error type to escape the arena.
    pub fn try_enter<F, R>(&mut self, f: F) -> Result<R, StaticError>
    where
        F: for<'gc> FnOnce(Context<'gc>) -> Result<R, Error<'gc>>,
    {
        self.enter(move |ctx| f(ctx).map_err(Error::into_static))
    }

    /// Run the given executor to completion.
    ///
    /// This will periodically exit the arena in order to collect garbage concurrently with running
    /// Lua code.
    pub fn finish(&mut self, executor: &StashedExecutor) {
        const FUEL_PER_GC: i32 = 4096;

        loop {
            let mut fuel = Fuel::with(FUEL_PER_GC);

            if self.enter(|ctx| ctx.fetch(executor).step(ctx, &mut fuel)) {
                break;
            }
        }
    }

    /// Run the given executor to completion and then take return values from the returning thread.
    ///
    /// This is equivalent to calling `Lua::finish` on an executor and then calling
    /// `Executor::take_result` yourself.
    pub fn execute<R: for<'gc> FromMultiValue<'gc>>(
        &mut self,
        executor: &StashedExecutor,
    ) -> Result<R, StaticError> {
        self.finish(executor);
        self.try_enter(|ctx| ctx.fetch(executor).take_result::<R>(ctx)?)
    }
}

#[derive(Copy, Clone, Collect)]
#[collect(no_drop)]
struct State<'gc> {
    globals: Table<'gc>,
    registry: Registry<'gc>,
    strings: InternedStringSet<'gc>,
    finalizers: Finalizers<'gc>,
}

impl<'gc> State<'gc> {
    fn new(mc: &Mutation<'gc>) -> State<'gc> {
        Self {
            globals: Table::new(mc),
            registry: Registry::new(mc),
            strings: InternedStringSet::new(mc),
            finalizers: Finalizers::new(mc),
        }
    }

    fn ctx(&'gc self, mutation: &'gc Mutation<'gc>) -> Context<'gc> {
        Context {
            mutation,
            state: self,
        }
    }
}
