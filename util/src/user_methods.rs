use std::marker::PhantomData;

use gc_arena::{barrier, Mutation, Root, Rootable};
use piccolo::{
    Callback, CallbackReturn, Context, Error, Execution, FromMultiValue, IntoMultiValue, IntoValue,
    MetaMethod, Table, UserData, Value,
};

use crate::callback_fn;

pub struct UserMethods<'gc, U: for<'a> Rootable<'a>> {
    table: Table<'gc>,
    _marker: PhantomData<U>,
}

impl<'gc, U: for<'a> Rootable<'a>> Copy for UserMethods<'gc, U> {}

impl<'gc, U: for<'a> Rootable<'a>> Clone for UserMethods<'gc, U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'gc, U: for<'a> Rootable<'a>> UserMethods<'gc, U> {
    pub fn new(mc: &Mutation<'gc>) -> Self {
        Self {
            table: Table::new(mc),
            _marker: PhantomData,
        }
    }

    pub fn add<F, A, R>(self, name: &'static str, ctx: Context<'gc>, method: F) -> bool
    where
        F: Fn(&Root<'gc, U>, Context<'gc>, Execution<'gc, '_>, A) -> Result<R, Error<'gc>>
            + 'static,
        A: FromMultiValue<'gc>,
        R: IntoMultiValue<'gc>,
    {
        let callback = callback_fn(&ctx, move |ctx, exec, (this, args): (UserData, A)| {
            let this = this.downcast::<U>()?;
            method(&this, ctx, exec, args)
        });

        !self.table.set(ctx, name, callback).unwrap().is_nil()
    }

    pub fn add_write<F, A, R>(self, name: &'static str, ctx: Context<'gc>, method: F) -> bool
    where
        F: Fn(
                &barrier::Write<Root<'gc, U>>,
                Context<'gc>,
                Execution<'gc, '_>,
                A,
            ) -> Result<R, Error<'gc>>
            + 'static,
        A: FromMultiValue<'gc>,
        R: IntoMultiValue<'gc>,
    {
        let callback = callback_fn(&ctx, move |ctx, exec, (this, args): (UserData, A)| {
            let this = this.downcast_write::<U>(&ctx)?;
            method(&this, ctx, exec, args)
        });

        !self.table.set(ctx, name, callback).unwrap().is_nil()
    }

    pub fn metatable(self, ctx: Context<'gc>) -> Table<'gc> {
        let metatable = Table::new(&ctx);
        metatable.set(ctx, MetaMethod::Index, self.table).unwrap();
        metatable
    }

    pub fn wrap(self, ctx: Context<'gc>, ud: Root<'gc, U>) -> UserData<'gc> {
        let ud = UserData::new::<U>(&ctx, ud);
        ud.set_metatable(&ctx, Some(self.metatable(ctx)));
        ud
    }
}

impl<'gc, U: for<'a> Rootable<'a>> IntoValue<'gc> for UserMethods<'gc, U> {
    fn into_value(self, _: Context<'gc>) -> Value<'gc> {
        self.table.into()
    }
}

pub struct StaticUserMethods<'gc, U: 'static> {
    table: Table<'gc>,
    _marker: PhantomData<U>,
}

impl<'gc, U: 'static> Copy for StaticUserMethods<'gc, U> {}

impl<'gc, U: 'static> Clone for StaticUserMethods<'gc, U> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<'gc, U: 'static> StaticUserMethods<'gc, U> {
    pub fn new(mc: &Mutation<'gc>) -> Self {
        Self {
            table: Table::new(mc),
            _marker: PhantomData,
        }
    }

    pub fn add<F, A, R>(self, name: &'static str, ctx: Context<'gc>, method: F) -> bool
    where
        F: Fn(&U, Context<'gc>, Execution<'gc, '_>, A) -> Result<R, Error<'gc>> + 'static,
        A: FromMultiValue<'gc>,
        R: IntoMultiValue<'gc>,
    {
        let callback = callback_fn(&ctx, move |ctx, exec, (this, args): (UserData, A)| {
            let this = this.downcast_static::<U>()?;
            method(&this, ctx, exec, args)
        });

        !self.table.set(ctx, name, callback).unwrap().is_nil()
    }

    pub fn metatable(self, ctx: Context<'gc>) -> Table<'gc> {
        let metatable = Table::new(&ctx);
        metatable.set(ctx, MetaMethod::Index, self.table).unwrap();
        metatable
    }

    pub fn wrap(self, ctx: Context<'gc>, ud: U) -> UserData<'gc> {
        let ud = UserData::new_static(&ctx, ud);
        ud.set_metatable(&ctx, Some(self.metatable(ctx)));
        ud
    }
}

impl<'gc, U: 'static> IntoValue<'gc> for StaticUserMethods<'gc, U> {
    fn into_value(self, _: Context<'gc>) -> Value<'gc> {
        self.table.into()
    }
}
