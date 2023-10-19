#[macro_use]
mod macros;

#[macro_use]
mod net;

mod account;
mod buffer;
mod callback;
mod components;
mod db;
mod events;
mod framerate;
mod oneshot;
mod savestate;
mod signal;
mod tasks;

use std::{
  fmt::Debug,
  time::Duration,
};

use bevy::{
  app::{
    AppExit,
    ScheduleRunnerPlugin,
  },
  prelude::*,
};
use clap::Parser;
use net::TelnetPlugin;
use signal::Signal;
use tasks::*;
use tracing::info;
use tracing_subscriber::{
  fmt::{self,},
  prelude::*,
  EnvFilter,
};

use crate::{
  account::{
    AccountPlugin,
    StartLogin,
  },
  db::DbArg,
  net::{
    PortArg,
    *,
  },
  savestate::{
    SaveExt,
    SaveStatePlugin,
  },
  signal::SignalPlugin,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long, default_value = "sqlite://db.sqlite")]
  db: String,

  #[arg(short, long, default_value_t = 23840)]
  port: u32,
}

impl Plugin for Args {
  fn build(&self, app: &mut App) {
    app.insert_resource(PortArg(self.port));
    app.insert_resource(DbArg(self.db.clone()));
  }
}

fn main() -> anyhow::Result<()> {
  tracing_subscriber::registry()
    .with(fmt::layer().pretty())
    .with(
      EnvFilter::builder()
        .with_default_directive("canton=info".parse()?)
        .from_env_lossy(),
    )
    .init();

  let args = Args::parse();
  info!("Hello, args: {args:?}!");

  let mut app = App::new();

  app.add_plugins(
    MinimalPlugins.set(ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(
      1.0 / 60.0,
    ))),
  );

  app.add_plugins(args);

  app.add_plugins(SignalPlugin);
  app.add_plugins(framerate::LogFrameRatePlugin::<10>);
  app.add_plugins(callback::CallbackPlugin);
  app.add_plugins((TokioPlugin, db::DbPlugin, TelnetPlugin));
  app.add_systems(Update, signal_handler);

  app.add_plugins(SaveStatePlugin);
  app.persist_component::<crate::Name>();
  app.persist_component::<Parent>();
  app.add_plugins(AccountPlugin);

  app.add_systems(Update, telnet_handler);
  app.add_systems(Update, greeter);

  app.run();

  Ok(())
}

#[derive(Component, Reflect, Debug, Default)]
#[reflect(Component)]
struct Name(String);

#[derive(Component)]
struct Net(Entity);

#[derive(Component)]
struct Char(Entity);

fn greeter(mut cmd: Commands, mut query: Query<(Entity, &TelnetOut), Added<ClientConn>>) {
  for (entity, output) in query.iter_mut() {
    output.line("\x1b[1mWelcome!\x1b[0m");
    cmd.add(StartLogin(entity));
  }
}

fn telnet_handler(_cmd: Commands, mut query: Query<&mut TelnetIn>) {
  for mut input in query.iter_mut() {
    while let Some(event) = input.next_telnet() {
      debug!(?event, "ignoring telnet event");
    }
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
