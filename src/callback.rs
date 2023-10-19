#![allow(dead_code)]

use std::time::Duration;

use bevy::prelude::*;

#[allow(clippy::type_complexity)]
#[derive(Component)]
struct CallbackFn(Option<Box<dyn FnOnce(&mut Commands) + Send + Sync + 'static>>);

#[derive(Component)]
struct CallbackTimer(Timer);

#[derive(Bundle)]
pub struct Callback {
  timer: CallbackTimer,
  callback: CallbackFn,
}

impl Callback {
  pub fn new(after: Duration, cb: impl FnOnce(&mut Commands) + Send + Sync + 'static) -> Self {
    Callback {
      timer: CallbackTimer(Timer::from_seconds(after.as_secs_f32(), TimerMode::Once)),
      callback: CallbackFn(Some(Box::new(cb))),
    }
  }
}

fn run_callbacks(
  time: Res<Time>,
  mut query: Query<(Entity, &mut CallbackTimer, &mut CallbackFn)>,
  mut cmd: Commands,
) {
  for (ent, mut timer, mut cb) in query.iter_mut() {
    timer.0.tick(time.delta());

    if timer.0.finished() {
      if let Some(cb) = cb.0.take() {
        cb(&mut cmd);
      }
      cmd.entity(ent).despawn_recursive();
    }
  }
}

pub struct CallbackPlugin;

impl Plugin for CallbackPlugin {
  fn build(&self, app: &mut App) {
    app.add_systems(Update, run_callbacks);
  }
}
