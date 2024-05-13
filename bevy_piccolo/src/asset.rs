use std::borrow::Borrow;
use std::collections::HashSet;
use std::os::unix::ffi::OsStrExt;
use std::sync::Arc;
use std::{
    fmt,
    io,
    time,
};

use bevy::asset::io::Reader;
use bevy::asset::{
    Asset,
    AssetLoader,
    AsyncReadExt,
    LoadContext,
};
use bevy::ecs::system::Resource;
use bevy::log::debug;
use bevy::prelude::Deref;
use bevy::reflect::Reflect;
use bevy::utils::ConditionalSendFuture;
use parking_lot::RwLock;
use piccolo::compiler::{
    compile_chunk,
    parse_chunk,
    CompileError,
    CompiledPrototype,
    ParseError,
    StringInterner,
};
use thiserror::Error;

#[derive(Asset, Reflect, Clone)]
#[reflect(from_reflect = false)]
pub struct LuaProto {
    #[reflect(ignore)]
    pub path: ArcString,
    #[reflect(ignore)]
    pub compiled: CompiledPrototype<ArcString>,
    #[reflect(ignore)]
    pub compile_time: time::Instant,
}

#[derive(Debug, Default)]
pub struct LuaLoader {
    interner: ArcInterner,
}

#[derive(Error, Debug)]
pub enum LoadError {
    #[error("io error")]
    Io(#[from] io::Error),
    #[error("parse error")]
    Parse(#[from] ParseError),
    #[error("compile error")]
    Compile(#[from] CompileError),
}

impl AssetLoader for LuaLoader {
    type Asset = LuaProto;
    type Error = LoadError;
    type Settings = ();

    fn load<'a>(
        &'a self,
        reader: &'a mut Reader,
        _: &'a Self::Settings,
        ctx: &'a mut LoadContext,
    ) -> impl ConditionalSendFuture<Output = Result<Self::Asset, Self::Error>> {
        let path = (&self.interner).intern(ctx.asset_path().path().as_os_str().as_bytes());
        debug!(%path, "loading lua script");
        async move {
            let mut buf = vec![];
            reader.read_to_end(&mut buf).await?;
            let parsed = parse_chunk(buf.as_slice(), &self.interner)?;
            let compiled = compile_chunk(&parsed, &self.interner)?;
            debug!(%path, "script loaded successfully");
            Ok(LuaProto {
                path,
                compiled,
                compile_time: time::Instant::now(),
            })
        }
    }

    fn extensions(&self) -> &[&str] {
        &["lua"]
    }
}

#[derive(Deref, Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub struct ArcString(Arc<str>);

impl fmt::Display for ArcString {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl AsRef<[u8]> for ArcString {
    fn as_ref(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

impl Borrow<[u8]> for ArcString {
    fn borrow(&self) -> &[u8] {
        self.0.as_bytes()
    }
}

#[derive(Resource, Default, Debug)]
pub struct ArcInterner {
    strings: RwLock<HashSet<ArcString>>,
}

impl StringInterner for &'_ ArcInterner {
    type String = ArcString;

    fn intern(&mut self, s: &[u8]) -> Self::String {
        let mut strings = self.strings.upgradable_read();
        if let Some(s) = strings.get(s) {
            s.clone()
        } else {
            let s = ArcString(Arc::<str>::from(String::from_utf8_lossy(s)));
            strings.with_upgraded(|strings| {
                strings.insert(s.clone());
            });
            s
        }
    }
}

impl StringInterner for ArcInterner {
    type String = ArcString;

    fn intern(&mut self, s: &[u8]) -> Self::String {
        <&ArcInterner>::intern(&mut &*self, s)
    }
}

#[cfg(test)]
mod test {}
