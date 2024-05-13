use std::cell::RefCell;
use std::rc::Rc;
use std::time;

use anyhow::bail;
use anyhow::Error;
use asset::LuaProto;
use bevy::asset::AssetPath;
use bevy::ecs::component::Component;
use bevy::prelude::*;
use bevy::utils::{
    HashMap,
    HashSet,
};
use piccolo::{
    Closure,
    Executor,
    FromMultiValue,
    FromValue,
    Function,
    FunctionPrototype,
    IntoMultiValue,
    StashedExecutor,
    StashedTable,
    StaticError,
    Table,
};

use crate::thread_local::ThreadLocal;

pub mod asset;
pub mod methods;
pub mod query;
mod thread_local;
pub mod world;

pub trait AppWorldExt {
    fn add_lua_system<'a>(&mut self, script: impl Into<AssetPath<'a>>);
}

impl AppWorldExt for App {
    fn add_lua_system<'a>(&mut self, script: impl Into<AssetPath<'a>>) {
        self.world_mut().add_lua_system(script);
    }
}

impl AppWorldExt for World {
    fn add_lua_system<'a>(&mut self, script: impl Into<AssetPath<'a>>) {
        let loader = self.resource::<AssetServer>();
        let handle = loader.load::<LuaProto>(script);
        let mut systems = self.resource_mut::<LuaSystems>();
        systems.0.insert(LuaScript::new(handle));
    }
}

pub struct LuaPlugin<F = fn() -> piccolo::Lua> {
    init_lua: F,
}

fn default_lua() -> piccolo::Lua {
    piccolo::Lua::full()
}

impl Default for LuaPlugin {
    fn default() -> Self {
        LuaPlugin {
            init_lua: default_lua,
        }
    }
}

impl<F> LuaPlugin<F> {
    pub fn with_init<G>(init_lua: G) -> LuaPlugin<G> {
        LuaPlugin { init_lua }
    }
}

impl<F> Plugin for LuaPlugin<F>
where
    F: Fn() -> piccolo::Lua + Copy + Sync + Send + 'static,
{
    fn build(&self, app: &mut App) {
        app.init_asset::<LuaProto>();
        app.init_asset_loader::<asset::LuaLoader>();
        app.init_resource::<LuaSystems>();
        app.insert_resource(LocalLua {
            init_lua: self.init_lua,
            lua: Default::default(),
        });
        app.add_systems(Update, global_lua_system);
    }
}

pub struct Lua {
    env: piccolo::Lua,
    script_loads: HashMap<AssetId<LuaProto>, ScriptModule>,
    exec: StashedExecutor,
}

impl Lua {
    fn new(init: impl FnOnce() -> piccolo::Lua) -> Lua {
        let mut vm = init();

        let exec = vm.enter(|ctx| ctx.stash(Executor::new(ctx)));
        Lua {
            env: vm,
            script_loads: Default::default(),
            exec,
        }
    }
}

#[derive(Clone)]
pub struct ScriptModule {
    table: StashedTable,
    load_time: time::Instant,
}

#[derive(Default, Resource)]
pub struct LocalLua<F = fn() -> piccolo::Lua> {
    init_lua: F,
    lua: ThreadLocal<Rc<RefCell<Lua>>>,
}

pub struct LuaEnv<'w> {
    world: &'w mut World,
    lua: Rc<RefCell<Lua>>,
}

impl<'w> LuaEnv<'w> {
    pub fn from_world(world: &'w mut World) -> Self {
        let lua_res = world.resource::<LocalLua>();
        let lua = lua_res.lua.entry(|e| {
            e.or_insert_with(|| Rc::new(RefCell::new(Lua::new(lua_res.init_lua))))
                .clone()
        });
        Self { world, lua }
    }

    pub fn get_lua(&self) -> Rc<RefCell<Lua>> {
        self.lua.clone()
    }

    pub fn load_script(&mut self, asset: &LuaProto) -> Result<ScriptModule, StaticError> {
        let lua = self.get_lua();
        let Lua {
            env: ref mut vm,
            ref mut exec,
            ..
        } = *lua.borrow_mut();
        world::WorldMutF::in_scope(self.world, |worldf| {
            vm.try_enter(|ctx| {
                let fn_proto = FunctionPrototype::from_compiled_map_strings(
                    &ctx,
                    piccolo::String::from_slice(&ctx, &asset.path),
                    &asset.compiled,
                    |s| piccolo::String::from_slice(&ctx, s),
                );
                let closure = Closure::new(&ctx, fn_proto, Some(ctx.globals()))?;
                let exec = ctx.fetch(exec);
                ctx.set_global("WORLD", world::World(worldf)).unwrap();
                exec.restart(ctx, closure.into(), ());
                Ok(())
            })?;

            vm.finish(exec);

            vm.try_enter(|ctx| {
                let table = ctx.fetch(exec).take_result::<Table>(ctx)??;
                Ok(ctx.stash(table))
            })
            .map(|table| ScriptModule {
                table,
                load_time: asset.compile_time,
            })
        })
    }

    fn get_script(
        &mut self,
        script: impl Into<AssetId<LuaProto>>,
        reload: time::Duration,
    ) -> Result<StashedTable, Error> {
        enum ScriptState {
            NeedsLoad,
            AlreadyLoaded(StashedTable),
        }
        let script_id = script.into();
        let state = (|| -> anyhow::Result<_> {
            let Lua { script_loads, .. } = &*self.lua.borrow();
            let Some(script) = script_loads.get(&script_id) else {
                return Ok(ScriptState::NeedsLoad);
            };

            if time::Instant::now() - script.load_time > reload {
                let Some(asset) = self.world.resource::<Assets<LuaProto>>().get(script_id) else {
                    bail!("no asset found for script id: {:?}", script_id);
                };

                if script.load_time < asset.compile_time {
                    return Ok(ScriptState::NeedsLoad);
                }
            }

            Ok(ScriptState::AlreadyLoaded(script.table.clone()))
        })()?;

        match state {
            ScriptState::NeedsLoad => {
                let Some(asset) = self
                    .world
                    .resource::<Assets<LuaProto>>()
                    .get(script_id)
                    .cloned()
                else {
                    bail!("no asset found for script id: {script_id:?}");
                };
                debug!("loading script");
                let loaded = self.load_script(&asset)?;
                debug!("script loaded successfully");
                self.get_lua()
                    .borrow_mut()
                    .script_loads
                    .insert(script_id, loaded.clone());
                Ok(loaded.table)
            }
            ScriptState::AlreadyLoaded(script) => Ok(script),
        }
    }

    fn call<R>(
        &mut self,
        script: impl Into<AssetId<LuaProto>>,
        reload: time::Duration,
        func: &str,
        args: impl for<'a> IntoMultiValue<'a>,
    ) -> Result<R, Error>
    where
        R: for<'a> FromMultiValue<'a>,
    {
        let module = self.get_script(script, reload)?;
        let lua = self.get_lua();
        let Lua {
            env: ref mut vm,
            ref mut exec,
            ..
        } = *lua.borrow_mut();
        world::WorldMutF::in_scope(self.world, |worldf| {
            vm.try_enter(|ctx| {
                let exec = ctx.fetch(exec);
                let tab = ctx.fetch(&module);
                let f = tab.get(ctx, piccolo::String::from_slice(&ctx, func));
                ctx.set_global("WORLD", world::World(worldf)).unwrap();
                exec.restart(ctx, Function::from_value(ctx, f)?, args);
                Ok(())
            })?;
            Ok(vm.execute(exec)?)
        })
    }
}

pub fn global_lua_system(world: &mut World) {
    let mut env = LuaEnv::from_world(world);
    for id in env
        .world
        .resource::<Assets<LuaProto>>()
        .ids()
        .collect::<Vec<_>>()
    {
        let _: () = env
            .call(id, time::Duration::from_secs(5), "update_global", ())
            .unwrap_or_else(|e| {
                warn!("error calling global update: {e}");
            });
    }
}

pub fn local_lua_system(world: &mut World, scripted: Query<(Entity, &LuaScript)>) {
    scripted.iter().for_each(|(entity, script)| {
        let mut env = LuaEnv::from_world(world);
        let _: () = env
            .call(
                &script.0,
                time::Duration::from_secs(5),
                "update_self",
                query::Entity(entity),
            )
            .unwrap_or_else(|e| {
                warn!("error calling self update: {e}");
            });
    });
}

#[derive(Component, Default, Hash, Eq, PartialEq)]
pub struct LuaScript(Handle<LuaProto>);

impl LuaScript {
    pub fn new(proto: Handle<LuaProto>) -> Self {
        Self(proto)
    }
}

#[derive(Default, Resource)]
pub struct LuaSystems(HashSet<LuaScript>);

#[cfg(test)]
mod tests {

    use super::*;

    #[derive(Component, Reflect)]
    struct Foo {
        bar: usize,
        baz: String,
    }

    #[derive(Component, Reflect)]
    struct Spam(String);

    #[test]
    fn it_works() {
        let mut app = App::new();
        app.add_plugins(DefaultPlugins);
        app.add_plugins(LuaPlugin::default());
        app.register_type::<Foo>();
        app.register_type::<Spam>();
        app.add_lua_system("scripts/hello.lua");

        let foo_only = app
            .world_mut()
            .spawn(Foo {
                bar: 5,
                baz: "asdf".into(),
            })
            .id();
        let spam_only = app.world_mut().spawn(Spam("eggs".into())).id();
        let both = app
            .world_mut()
            .spawn((
                Foo {
                    bar: 501,
                    baz: "asejf".into(),
                },
                Spam("osaijfi".into()),
            ))
            .id();
        info!(?foo_only);
        info!(?spam_only);
        info!(?both);
        app.update();
    }
}
