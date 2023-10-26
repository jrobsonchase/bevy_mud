#[macro_use]
mod macros;

#[macro_use]
mod net;

mod account;

mod coords;
mod core;
mod db;
mod framerate;
mod oneshot;
mod savestate;
mod signal;
mod tasks;

mod ascii_map;

use std::fmt::Debug;

use bevy::prelude::*;
use clap::Parser;
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
  core::CorePlugin,
  db::DbArg,
  net::{
    PortArg,
    *,
  },
  savestate::SaveExt,
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

  app.add_plugins(CorePlugin);

  app.add_plugins(args);

  app.persist_component::<crate::Name>();
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
