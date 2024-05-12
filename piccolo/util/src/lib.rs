use gc_arena::Mutation;
use piccolo::{
    Callback, CallbackReturn, Context, Error, Execution, FromMultiValue, IntoMultiValue,
};

pub mod freeze;
pub mod user_methods;

#[cfg(feature = "serde")]
pub mod serde;

pub fn callback_fn<
    'gc,
    A: FromMultiValue<'gc>,
    R: IntoMultiValue<'gc>,
    F: Fn(Context<'gc>, Execution<'gc, '_>, A) -> Result<R, Error<'gc>> + 'static,
>(
    mc: &Mutation<'gc>,
    cb: F,
) -> Callback<'gc> {
    Callback::from_fn(&mc, move |ctx, exec, mut stack| {
        let args = stack.consume(ctx)?;
        stack.replace(ctx, cb(ctx, exec, args)?);
        Ok(CallbackReturn::Return)
    })
}
