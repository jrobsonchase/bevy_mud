use std::time::Duration;

use bevy::{
  app::{
    AppExit,
    ScheduleRunnerPlugin,
  },
  asset::ChangeWatcher,
  prelude::*,
};

use crate::{
  net::TelnetPlugin,
  savestate::SaveStatePlugin,
  scripting::ScriptingPlugin,
  signal::{
    Signal,
    SignalPlugin,
  },
  tasks::TokioPlugin,
};

pub struct CorePlugin;

impl Plugin for CorePlugin {
  fn build(&self, app: &mut App) {
    app.add_plugins(
      MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
      ))),
    );

    app.add_plugins(AssetPlugin {
      watch_for_changes: ChangeWatcher::with_delay(Duration::from_millis(100)),
      ..Default::default()
    });

    app.add_plugins(ScriptingPlugin);

    app.add_plugins(SignalPlugin);
    app.add_plugins(crate::framerate::LogFrameRatePlugin::<10>);
    app.add_plugins((TokioPlugin, crate::db::DbPlugin, TelnetPlugin));
    app.add_systems(Update, signal_handler);
    app.add_plugins(SaveStatePlugin);
  }
}

fn signal_handler(mut signal: EventReader<Signal>, mut exit: EventWriter<AppExit>) {
  match try_opt!(signal.iter().next().cloned(), return) {
    signal @ Signal::SIGINT | signal @ Signal::SIGTERM | signal @ Signal::SIGQUIT => {
      debug!(?signal, "received signal, exiting");
      exit.send(AppExit);
    }
    _ => {}
  }
}
