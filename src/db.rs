use std::ops::{
  Deref,
  DerefMut,
};

use bevy::{
  app::AppExit,
  prelude::*,
};
use sqlx::{
  Pool,
  Sqlite,
  SqlitePool,
};

use crate::tasks::TokioRuntime;

#[derive(Resource)]
pub struct DbArg(pub String);

pub struct DbPlugin;

impl Plugin for DbPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Startup, connect_db);
  }
}

#[derive(Resource)]
pub struct Db(Pool<Sqlite>);

impl Deref for Db {
  type Target = Pool<Sqlite>;

  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for Db {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
}

fn connect_db(
  arg: Res<DbArg>,
  rt: Res<TokioRuntime>,
  mut commands: Commands,
  mut exit: EventWriter<AppExit>,
) {
  let _entered = rt.enter();
  let db = match SqlitePool::connect_lazy(&arg.0) {
    Ok(db) => db,
    Err(err) => {
      warn!(?err, "failed to connect to db, exiting.");
      exit.send(AppExit);
      return;
    }
  };

  commands.insert_resource(Db(db));
}
