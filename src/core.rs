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
  scene::ScenePlugin,
};
use serde::{
  Deserialize,
  Serialize,
};
use tracing::Level;
use tracing_subscriber::{
  prelude::*,
  registry::Registry,
  EnvFilter,
};
use tracing_tracy::DefaultConfig;

use crate::{
  account::AccountPlugin,
  action::ActionPlugin,
  character::CharacterPlugin,
  command::GameCommandsPlugin,
  framerate::LogFrameRatePlugin,
  map::MapPlugin,
  movement::MovementPlugin,
  net::TelnetPlugin,
  savestate::{
    traits::AppWorldExt,
    SaveStatePlugin,
  },
  signal::{
    Signal,
    SignalPlugin,
  },
  util::DebugLifecycle,
};

/// Marker for entites that are "live" and should be included in update queries.
///
/// Note that this is different than alive/dead status and is more for
/// differentiating between entities that are in the world vs in "storage".
#[derive(Component, Reflect, Debug, Default, Clone, Copy, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Live;

fn live_removed(
  trigger: Trigger<OnRemove, Live>,
  mut cmd: Commands,
  children_query: Query<&Children>,
  parent_query: Query<&Parent>,
  live_query: LiveQuery<()>,
) {
  let entity = trigger.entity();
  // Propagate un-liveness to children
  let children = children_query
    .get(entity)
    .map(|c| c.iter().cloned())
    .into_iter()
    .flatten()
    .collect::<Vec<Entity>>();
  for child in children {
    debug!(?entity, ?child, "propagating un-liveness to child");
    cmd.entity(child).remove::<Live>();
  }

  // Don't let live entities have dead children. That's just sad :(
  if parent_query
    .get(entity)
    .and_then(|p| live_query.get(p.get()))
    .is_ok()
  {
    warn!(
      ?entity,
      parent = ?parent_query.get(entity).unwrap().get(),
      "removing dead child of live parent"
    );
    cmd.entity(entity).remove_parent();
  }
}
fn live_added(trigger: Trigger<OnAdd, Live>, mut cmd: Commands, children_query: Query<&Children>) {
  let entity = trigger.entity();
  // Propagate liveness to children
  let children = children_query
    .get(entity)
    .map(|c| c.iter().cloned())
    .into_iter()
    .flatten()
    .collect::<Vec<Entity>>();
  for child in children {
    debug!(?entity, ?child, "propagating liveness to child");
    cmd.entity(child).insert(Live);
  }
}

fn live_parent_inserted(
  trigger: Trigger<OnInsert, Parent>,
  mut cmd: Commands,
  live: Query<Has<Live>>,
  parent: Query<&Parent>,
) {
  let child = trigger.entity();
  let Ok(parent) = parent.get(child).map(|p| p.get()) else {
    warn!(?child, "inserted parent component not found for child");
    return;
  };
  let parent_live = live.get(parent).unwrap_or_else(|_| {
    warn!(?parent, ?child, "parent not found, assuming dead");
    false
  });
  if parent_live {
    debug!(?child, ?parent, "adding Live due to hierarchy change");
    cmd.entity(child).insert(Live);
  } else {
    debug!(?child, ?parent, "removing Live due to hierarchy change");
    cmd.entity(child).remove::<Live>();
  }
}

pub type LiveQuery<'w, 's, D, F = ()> = Query<'w, 's, D, (With<Live>, F)>;
pub type UnLiveQuery<'w, 's, D, F = ()> = Query<'w, 's, D, (Without<Live>, F)>;

#[derive(SystemSet, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MudStartup {
  System,
  Io,
  World,
}

#[derive(SystemSet, Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub enum MudUpdate {
  Input,
  Resolve,
  Output,
}

pub struct CorePlugin;

impl Plugin for CorePlugin {
  fn build(&self, app: &mut App) {
    app.configure_sets(
      Startup,
      (
        MudStartup::System,
        MudStartup::Io.run_if(not(on_event::<AppExit>)),
        MudStartup::World.run_if(not(on_event::<AppExit>)),
      )
        .chain(),
    );
    app.configure_sets(
      Update,
      (MudUpdate::Input, MudUpdate::Resolve, MudUpdate::Output).chain(),
    );

    app
      .register_type::<Parent>()
      .register_type::<Children>()
      .register_type::<Live>();

    app.add_plugins((
      LogPlugin::default(),
      MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
        1.0 / 60.0,
      ))),
      HierarchyPlugin,
      DiagnosticsPlugin,
      AssetPlugin::default(),
      ScenePlugin,
      SignalPlugin,
      LogFrameRatePlugin::<10>,
    ));

    app.add_plugins((
      SaveStatePlugin::default(),
      TelnetPlugin,
      CharacterPlugin,
      MapPlugin,
      ActionPlugin,
      AccountPlugin,
      GameCommandsPlugin,
      MovementPlugin,
    ));

    app.persist::<Live>();

    app
      .debug_lifecycle::<Live>("Live")
      .observe(live_added)
      .observe(live_removed)
      .observe(signal_handler)
      .observe(live_parent_inserted);
  }
}

fn signal_handler(signal: Trigger<Signal>, mut exit: EventWriter<AppExit>) {
  match signal.event() {
    signal @ Signal::SIGINT | signal @ Signal::SIGTERM | signal @ Signal::SIGQUIT => {
      debug!(?signal, "received signal, exiting");
      exit.send(AppExit::Success);
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
    let tracy_layer = tracing_tracy::TracyLayer::new(DefaultConfig::default());

    let fmt_layer = tracing_subscriber::fmt::Layer::default()
      .pretty()
      .with_writer(std::io::stderr);

    // bevy_mud::framerate logs a `tracy.frame_mark` event every frame
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
