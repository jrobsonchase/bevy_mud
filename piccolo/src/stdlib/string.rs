use anyhow::bail;

use crate::{Callback, CallbackReturn, Context, FromValue, IntoValue, Table, Value};

pub fn load_string<'gc>(ctx: Context<'gc>) {
    let string = Table::new(&ctx);

    string
        .set(
            ctx,
            "len",
            Callback::from_fn(&ctx, |ctx, _, mut stack| {
                let v: Option<Value> = stack.consume(ctx)?;
                if let Some(len) = v.and_then(|v| match v {
                    Value::Integer(i) => Some(i.to_string().as_bytes().len().try_into().unwrap()),
                    Value::Number(n) => Some(n.to_string().as_bytes().len().try_into().unwrap()),
                    Value::String(s) => Some(s.len()),
                    _ => None,
                }) {
                    stack.replace(ctx, len);
                    Ok(CallbackReturn::Return)
                } else {
                    Err("Bad argument to len".into_value(ctx).into())
                }
            }),
        )
        .unwrap();

    string
        .set(
            ctx,
            "byte",
            Callback::from_fn(&ctx, |ctx, _, mut stack| {
                let (s, i, j): (crate::String, u32, Option<u32>) = stack.consume(ctx)?;

                let j = j.unwrap_or(i);

                let bytes = s.as_bytes();
                for i in i..=j {
                    if i == 0 {
                        continue;
                    }
                    if let Some(b) = bytes.get(i as usize - 1) {
                        stack.push_back(b.into_value(ctx));
                    }
                }
                Ok(CallbackReturn::Return)
            }),
        )
        .unwrap();

    string
        .set(
            ctx,
            "char",
            Callback::from_fn(&ctx, |ctx, _, mut stack| {
                let mut buf = vec![];
                loop {
                    if stack.is_empty() {
                        break;
                    }
                    let code = <u8>::from_value(ctx, stack.pop_front())?;
                    buf.push(code);
                }

                stack.push_back(crate::String::from_buffer(&ctx, buf.into()).into_value(ctx));

                Ok(CallbackReturn::Return)
            }),
        )
        .unwrap();

    ctx.set_global("string", string).unwrap();
}
