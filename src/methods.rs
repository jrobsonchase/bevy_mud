use std::fmt::Debug;

use piccolo::{
    Callback,
    CallbackReturn,
    Context,
    FromValue,
    IntoValue,
    Table,
    UserData,
    Value,
};
use piccolo_util::user_methods::StaticUserMethods;

const REGISTRY_KEY: &str = "__bevy_piccolo_method_registry";

pub fn get_registry(ctx: Context) -> Table {
    Table::from_value(ctx, ctx.get_global(REGISTRY_KEY)).unwrap_or_else(|_| {
        let new_registry = Table::new(&ctx);
        ctx.set_global(REGISTRY_KEY, new_registry).unwrap();
        new_registry
    })
}

pub fn register<'gc, T: Debug + 'static>(
    ctx: Context<'gc>,
    name: &str,
    f: impl FnOnce(Context<'gc>, StaticUserMethods<'gc, T>),
) -> Table<'gc> {
    let registry = get_registry(ctx);

    let key = piccolo::String::from_slice(&ctx, name);
    Table::from_value(ctx, registry.get(ctx, key)).unwrap_or_else(move |_| {
        let methods = StaticUserMethods::new(&ctx);
        f(ctx, methods);
        let meta = methods.metatable(ctx);
        meta.set(
            ctx,
            piccolo::MetaMethod::ToString,
            Callback::from_fn(&ctx, |ctx, _, mut stack| {
                let ud: UserData = stack.consume(ctx)?;
                let v = ud.downcast_static::<T>()?;
                stack.replace(ctx, format!("{:?}", v));
                Ok(CallbackReturn::Return)
            }),
        )
        .unwrap();

        registry.set(ctx, key, meta).unwrap();

        meta
    })
}

pub fn wrap_with<'gc, T: Debug + 'static>(
    ctx: Context<'gc>,
    name: &str,
    v: T,
    f: impl FnOnce(Context<'gc>, StaticUserMethods<'gc, T>),
) -> Value<'gc> {
    let meta = register::<T>(ctx, name, f);
    let ud = UserData::new_static(&ctx, v);
    ud.set_metatable(&ctx, Some(meta));
    ud.into_value(ctx)
}
