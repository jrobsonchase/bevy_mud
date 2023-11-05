use std::fmt::Debug;

use bevy::prelude::*;
use canton::{
  account::{
    AccountPlugin,
    StartLogin,
  },
  character::CharacterPlugin,
  command::GameCommandsPlugin,
  core::CorePlugin,
  db::DbArg,
  map::MapPlugin,
  net::{
    PortArg,
    *,
  },
};
use clap::Parser;
#[cfg(feature = "otel")]
use opentelemetry_api::KeyValue;
#[cfg(feature = "otel")]
use opentelemetry_otlp::WithExportConfig;
#[cfg(feature = "otel")]
use opentelemetry_sdk::{
  trace::{
    self,
    RandomIdGenerator,
    Sampler,
  },
  Resource,
};

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
  #[arg(short, long, default_value = "sqlite://db.sqlite")]
  db: String,

  #[arg(short, long, default_value_t = 23840)]
  port: u32,

  #[arg(long, default_value_t = false)]
  otel: bool,
}

impl Plugin for Args {
  fn build(&self, app: &mut App) {
    app.insert_resource(PortArg(self.port));
    app.insert_resource(DbArg(self.db.clone()));
  }
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let rt = tokio::runtime::Builder::new_multi_thread()
    .enable_all()
    .build()
    .unwrap();

  let mut app = App::new();

  app.add_plugins(CorePlugin::with_runtime(rt.handle().clone()));

  app.add_plugins(args);

  app.add_plugins(AccountPlugin);
  app.add_plugins(GameCommandsPlugin);
  app.add_plugins(CharacterPlugin);
  app.add_plugins(MapPlugin);

  app.add_systems(Update, telnet_handler);
  app.add_systems(Update, greeter);

  app.run();

  Ok(())
}

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
