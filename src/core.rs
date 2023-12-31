use std::{
  panic,
  time::Duration,
};

use bevy::{
  app::{
    AppExit,
    ScheduleRunnerPlugin,
  },
  diagnostic::DiagnosticsPlugin,
  prelude::*,
};
use tokio::runtime::Handle;
use tracing::Level;
use tracing_subscriber::{
  prelude::*,
  registry::Registry,
  EnvFilter,
};

use crate::{
  framerate::LogFrameRatePlugin,
  net::TelnetPlugin,
  savestate::SaveStatePlugin,
  signal::{
    Signal,
    SignalPlugin,
  },
  tasks::TokioPlugin,
};

#[derive(SystemSet, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum CantonStartup {
  System,
  Io,
  World,
}

#[derive(Default)]
pub struct CorePlugin(Option<Handle>);

impl CorePlugin {
  pub fn with_runtime(handle: Handle) -> Self {
    Self(Some(handle))
  }
}

impl Plugin for CorePlugin {
  fn build(&self, app: &mut App) {
    app.configure_sets(
      Startup,
      (
        CantonStartup::System,
        CantonStartup::Io,
        CantonStartup::World,
      )
        .chain(),
    );

    app.add_plugins((
      LogPlugin::default(),
      MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
      ))),
      HierarchyPlugin,
      DiagnosticsPlugin,
      AssetPlugin::default(),
      SignalPlugin,
      LogFrameRatePlugin::<10>,
    ));

    app.add_systems(Update, signal_handler);

    app.add_plugins((
      TokioPlugin::new(self.0.clone()),
      crate::db::DbPlugin,
      SaveStatePlugin,
      TelnetPlugin,
    ));
  }
}

fn signal_handler(mut signal: EventReader<Signal>, mut exit: EventWriter<AppExit>) {
  match try_opt!(signal.read().next().cloned(), return) {
    signal @ Signal::SIGINT | signal @ Signal::SIGTERM | signal @ Signal::SIGQUIT => {
      debug!(?signal, "received signal, exiting");
      exit.send(AppExit);
    }
    _ => {}
  }
}

#[cfg(feature = "tracy_memory")]
#[global_allocator]
static GLOBAL: tracy_client::ProfiledAllocator<std::alloc::System> =
  tracy_client::ProfiledAllocator::new(std::alloc::System, 100);

/// LogPlugin mostly lifted from bevy_log, but tweaked for prettier output and
/// stripped of unused features.
pub struct LogPlugin {
  /// Filters logs using the [`EnvFilter`] format
  pub filter: String,

  /// Filters out logs that are "less than" the given level.
  /// This can be further filtered using the `filter` setting.
  pub level: Level,
}

impl Default for LogPlugin {
  fn default() -> Self {
    Self {
      filter: "wgpu=error,naga=warn".to_string(),
      level: Level::INFO,
    }
  }
}

impl Plugin for LogPlugin {
  fn build(&self, _app: &mut App) {
    let old_handler = panic::take_hook();
    panic::set_hook(Box::new(move |infos| {
      println!("{}", tracing_error::SpanTrace::capture());
      old_handler(infos);
    }));

    let finished_subscriber;
    let default_filter = { format!("{},{}", self.level, self.filter) };
    let filter_layer = EnvFilter::try_from_default_env()
      .or_else(|_| EnvFilter::try_new(&default_filter))
      .unwrap();
    let subscriber = Registry::default().with(filter_layer);

    let subscriber = subscriber.with(tracing_error::ErrorLayer::default());

    #[cfg(feature = "tracy")]
    let tracy_layer = tracing_tracy::TracyLayer::new();

    let fmt_layer = tracing_subscriber::fmt::Layer::default()
      .pretty()
      .with_writer(std::io::stderr);

    // canton::framerate logs a `tracy.frame_mark` event every frame
    // at Level::INFO. Formatted logs should omit it.
    #[cfg(feature = "tracy")]
    let fmt_layer = fmt_layer.with_filter(tracing_subscriber::filter::FilterFn::new(|meta| {
      meta.fields().field("tracy.frame_mark").is_none()
    }));

    let subscriber = subscriber.with(fmt_layer);

    #[cfg(feature = "tracy")]
    let subscriber = subscriber.with(tracy_layer);

    finished_subscriber = subscriber;

    finished_subscriber.init();
  }
}
