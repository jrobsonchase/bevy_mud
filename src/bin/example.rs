use std::fmt::Debug;

use bevy::prelude::*;
use bevy_mud::{
  account::StartLogin,
  core::CorePlugin,
  negotiate,
  net::*,
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
  }
}

fn main() -> anyhow::Result<()> {
  let args = Args::parse();

  let mut app = App::new();

  app.add_plugins(CorePlugin);

  app.add_plugins(args);

  app.add_systems(Update, greeter);

  app.run();

  Ok(())
}

fn greeter(mut cmd: Commands, mut query: Query<(Entity, &TelnetOut), Added<ClientConn>>) {
  for (entity, output) in query.iter_mut() {
    output.line("\x1b[1mWelcome!\x1b[0m");
    negotiate!(output, WILL, GMCP);
    cmd.queue(StartLogin(entity));
  }
}
