use std::{f64, fmt, i64, io, string::String as StdString};

use gc_arena::{Collect, Gc};

use crate::{
    Callback, Closure, Constant, Context, FromValue, Function, String, Table, Thread, TypeError,
    UserData,
};

#[derive(Debug, Copy, Clone, Collect)]
#[collect(no_drop)]
pub enum Value<'gc> {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    String(String<'gc>),
    Table(Table<'gc>),
    Function(Function<'gc>),
    Thread(Thread<'gc>),
    UserData(UserData<'gc>),
}

impl<'gc> Default for Value<'gc> {
    fn default() -> Self {
        Value::Nil
    }
}

impl<'gc> Value<'gc> {
    pub fn type_name(self) -> &'static str {
        match self {
            Value::Nil => "nil",
            Value::Boolean(_) => "boolean",
            Value::Integer(_) | Value::Number(_) => "number",
            Value::String(_) => "string",
            Value::Table(_) => "table",
            Value::Function(_) => "function",
            Value::Thread(_) => "thread",
            Value::UserData(_) => "userdata",
        }
    }

    pub fn display<W: io::Write>(self, mut w: W) -> Result<(), io::Error> {
        match self {
            Value::Nil => write!(w, "nil"),
            Value::Boolean(b) => write!(w, "{}", b),
            Value::Integer(i) => write!(w, "{}", i),
            Value::Number(f) => write!(w, "{}", f),
            Value::String(s) => w.write_all(s.as_bytes()),
            Value::Table(t) => write!(w, "<table {:p}>", Gc::as_ptr(t.into_inner())),
            Value::Function(Function::Closure(c)) => {
                write!(w, "<function {:p}>", Gc::as_ptr(c.into_inner()))
            }
            Value::Function(Function::Callback(c)) => {
                write!(w, "<function {:p}>", Gc::as_ptr(c.into_inner()))
            }
            Value::Thread(t) => write!(w, "<thread {:p}>", Gc::as_ptr(t.into_inner())),
            Value::UserData(u) => write!(w, "<userdata {:p}>", Gc::as_ptr(u.into_inner())),
        }
    }

    pub fn is_nil(self) -> bool {
        matches!(self, Value::Nil)
    }

    /// Lua `nil` and `false` are false, anything else is true.
    pub fn to_bool(self) -> bool {
        match self {
            Value::Nil => false,
            Value::Boolean(false) => false,
            _ => true,
        }
    }

    pub fn not(self) -> Value<'gc> {
        Value::Boolean(!self.to_bool())
    }

    /// Interprets Numbers, Integers, and Strings as a Number, if possible.
    pub fn to_number(self) -> Option<f64> {
        self.to_constant().and_then(|c| c.to_number())
    }

    /// Interprets Numbers, Integers, and Strings as an Integer, if possible.
    pub fn to_integer(self) -> Option<i64> {
        self.to_constant().and_then(|c| c.to_integer())
    }

    pub fn to_constant(self) -> Option<Constant<String<'gc>>> {
        match self {
            Value::Nil => Some(Constant::Nil),
            Value::Boolean(b) => Some(Constant::Boolean(b)),
            Value::Integer(i) => Some(Constant::Integer(i)),
            Value::Number(n) => Some(Constant::Number(n)),
            Value::String(s) => Some(Constant::String(s)),
            _ => None,
        }
    }

    pub fn to_native<T: FromValue<'gc>>(self, ctx: Context<'gc>) -> Result<T, TypeError> {
        T::from_value(ctx, self)
    }
}

impl<'gc> fmt::Display for Value<'gc> {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> std::fmt::Result {
        let mut buf = Vec::new();
        self.display(&mut buf).unwrap();
        let s = StdString::from_utf8_lossy(&buf);
        write!(fmt, "{}", s)
    }
}

impl<'gc> From<bool> for Value<'gc> {
    fn from(v: bool) -> Value<'gc> {
        Value::Boolean(v)
    }
}

impl<'gc> From<i64> for Value<'gc> {
    fn from(v: i64) -> Value<'gc> {
        Value::Integer(v)
    }
}

impl<'gc> From<f64> for Value<'gc> {
    fn from(v: f64) -> Value<'gc> {
        Value::Number(v)
    }
}

impl<'gc, S> From<Constant<S>> for Value<'gc>
where
    String<'gc>: From<S>,
{
    fn from(constant: Constant<S>) -> Self {
        match constant {
            Constant::Nil => Value::Nil,
            Constant::Boolean(b) => Value::Boolean(b),
            Constant::Integer(i) => Value::Integer(i),
            Constant::Number(n) => Value::Number(n),
            Constant::String(s) => Value::String(s.into()),
        }
    }
}

impl<'gc> From<String<'gc>> for Value<'gc> {
    fn from(v: String<'gc>) -> Value<'gc> {
        Value::String(v)
    }
}

impl<'gc> From<Table<'gc>> for Value<'gc> {
    fn from(v: Table<'gc>) -> Value<'gc> {
        Value::Table(v)
    }
}

impl<'gc> From<Function<'gc>> for Value<'gc> {
    fn from(v: Function<'gc>) -> Value<'gc> {
        Value::Function(v)
    }
}

impl<'gc> From<Closure<'gc>> for Value<'gc> {
    fn from(v: Closure<'gc>) -> Value<'gc> {
        Value::Function(Function::Closure(v))
    }
}

impl<'gc> From<Callback<'gc>> for Value<'gc> {
    fn from(v: Callback<'gc>) -> Value<'gc> {
        Value::Function(Function::Callback(v))
    }
}

impl<'gc> From<Thread<'gc>> for Value<'gc> {
    fn from(v: Thread<'gc>) -> Value<'gc> {
        Value::Thread(v)
    }
}

impl<'gc> From<UserData<'gc>> for Value<'gc> {
    fn from(v: UserData<'gc>) -> Value<'gc> {
        Value::UserData(v)
    }
}
