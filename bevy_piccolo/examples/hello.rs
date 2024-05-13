use std::time::Duration;

use bevy::{
    app::ScheduleRunnerPlugin,
    diagnostic::DiagnosticsPlugin,
    log::LogPlugin,
    prelude::*,
};
use bevy_piccolo::*;

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Foo {
    bar: usize,
    baz: String,
}

#[derive(Component, Reflect)]
#[reflect(Component)]
struct Spam(String);

fn main() {
    let mut app = App::new();
    app.add_plugins((
        MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
            1.0 / 60.0,
        ))),
        HierarchyPlugin,
        DiagnosticsPlugin,
        AssetPlugin::default(),
        LogPlugin::default(),
    ));
    app.add_plugins(LuaPlugin::default());
    app.register_type::<Foo>();
    app.register_type::<Spam>();
    app.add_lua_system("scripts/hello.lua");
    app.world_mut().spawn(Foo {
        bar: 5,
        baz: "asdf".into(),
    });
    app.world_mut().spawn(Spam("eggs".into()));
    app.world_mut().spawn((
        Foo {
            bar: 501,
            baz: "asejf".into(),
        },
        Spam("osaijfi".into()),
    ));
    app.run();
}
