use std::{
  borrow::Cow,
  sync::Mutex,
};

use bevy::prelude::*;
use bevy_mod_scripting::{
  api::impl_tealr_type,
  lua::{
    api::bevy::LuaEntity,
    prelude::*,
  },
  prelude::*,
};
use bevy_mod_scripting_lua::prelude::mlua::Debug;
use tealr::{
  self,
};

pub struct LogAPIProvider;

#[derive(Clone)]
struct LogModule;

impl_tealr_type!(LogModule);

fn get_entity(lua: &Lua) -> Option<Entity> {
  lua
    .globals()
    .get("entity")
    .and_then(|e: LuaEntity| e.inner().to_lua_err())
    .ok()
}

fn source_info<'a>(debug_info: &'a Debug<'a>) -> (Cow<'a, str>, i32) {
  let source_info = debug_info.source();
  let source = source_info
    .source
    .map(String::from_utf8_lossy)
    .unwrap_or(Cow::Borrowed("<unknown>"));
  (source, debug_info.curr_line())
}

impl TealData for LogModule {
  fn add_methods<'lua, T: tealr::mlu::TealDataMethods<'lua, Self>>(methods: &mut T) {
    methods.document_type("logging api backed by the `tracing` crate");
    methods.document("Log a message at the TRACE level.");
    methods.add_function("trace", |lua: &Lua, message: String| {
      let entity = get_entity(lua);
      let debug_info = lua.inspect_stack(1).unwrap();
      let (file, line) = source_info(&debug_info);
      trace!(?entity, %file, line, message);
      Ok(())
    });
    methods.document("Log a message at the DEBUG level.");
    methods.add_function("debug", |lua: &Lua, message: String| {
      let entity = get_entity(lua);
      let debug_info = lua.inspect_stack(1).unwrap();
      let (file, line) = source_info(&debug_info);
      debug!(?entity, %file, line, message);
      Ok(())
    });
    methods.document("Log a message at the INFO level.");
    methods.add_function("info", |lua: &Lua, message: String| {
      let entity = get_entity(lua);
      let debug_info = lua.inspect_stack(1).unwrap();
      let (file, line) = source_info(&debug_info);
      info!(?entity, %file, line, message);
      Ok(())
    });
    methods.document("Log a message at the WARN level.");
    methods.add_function("warn", |lua: &Lua, message: String| {
      let entity = get_entity(lua);
      let debug_info = lua.inspect_stack(1).unwrap();
      let (file, line) = source_info(&debug_info);
      warn!(?entity, %file, line, message);
      Ok(())
    });
    methods.document("Log a message at the ERROR level.");
    methods.add_function("error", |lua: &Lua, message: String| {
      let entity = get_entity(lua);
      let debug_info = lua.inspect_stack(1).unwrap();
      let (file, line) = source_info(&debug_info);
      error!(?entity, %file, line, message);
      Ok(())
    });
  }
}

#[derive(Clone, Default)]
struct LogExport;

impl tealr::mlu::ExportInstances for LogExport {
  fn add_instances<'lua, T: tealr::mlu::InstanceCollector<'lua>>(
    self,
    instance_collector: &mut T,
  ) -> mlua::Result<()> {
    instance_collector.document_instance("Logging API");
    instance_collector.add_instance("log", |_| Ok(LogModule))?;

    Ok(())
  }
}

impl APIProvider for LogAPIProvider {
  type APITarget = Mutex<Lua>;
  type DocTarget = LuaDocFragment;
  type ScriptContext = Mutex<Lua>;

  fn attach_api(&mut self, api: &mut Self::APITarget) -> Result<(), ScriptError> {
    let api = api.get_mut().unwrap();

    tealr::mlu::set_global_env(LogExport, api).map_err(ScriptError::new_other)?;

    Ok(())
  }

  fn setup_script(
    &mut self,
    _script_data: &ScriptData,
    _ctx: &mut Self::ScriptContext,
  ) -> Result<(), ScriptError> {
    Ok(())
  }

  fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
    Some(LuaDocFragment::new("LogAPI", |tw| {
      tw.process_type::<LogModule>()
        .document_global_instance::<LogExport>()
        .unwrap()
    }))
  }
}
