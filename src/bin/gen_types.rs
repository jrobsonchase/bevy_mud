#[cfg(feature = "scripting")]
fn main() {
  use bevy::{
    app::ScheduleRunnerPlugin,
    prelude::*,
  };
  use canton::{
    core::LogPlugin,
    scripting::ScriptingPlugin,
  };

  let mut app = App::new();

  app.add_plugins(LogPlugin::default());
  app.add_plugins(MinimalPlugins.set(ScheduleRunnerPlugin::run_once()));
  app.add_plugins(AssetPlugin::default());
  app.add_plugins(ScriptingPlugin::gen_docs(true));

  app.run();
}

#[cfg(not(feature = "scripting"))]
fn main() {
  println!("the scripting feature must be enabled to generate documentation, silly goose!");
}
