use std::{
  any::Any,
  collections::VecDeque,
};

use bevy::{
  ecs::system::EntityCommands,
  prelude::*,
};

pub struct ActionPlugin;

impl Plugin for ActionPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<StopEvent>();

    app.register_type::<Queue>().register_type::<Busy>();

    app.add_systems(Update, (run_actions, apply_deferred).chain());
  }
}

#[derive(Deref, Event, Debug, Clone, Copy, Eq, PartialEq)]
pub struct StopEvent(pub Entity);

pub trait Action: Any + Send + Sync + 'static {
  /// Execute the action.
  fn execute(&self, cmd: &mut EntityCommands);

  /// Whether the action waits for un-busy, or triggers immediately
  fn waits(&self) -> bool {
    false
  }
  /// Whether the action sets the busy flag
  /// Note that the action is responsible for clearing this flag.
  fn busies(&self) -> bool {
    false
  }
}

#[derive(Component, Reflect, Default, Deref, DerefMut)]
#[reflect(Component)]
pub struct Queue(#[reflect(ignore)] VecDeque<Box<dyn Action>>);

#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Busy;

fn run_actions(cmd: ParallelCommands, mut query: Query<(Entity, &mut Queue), Without<Busy>>) {
  query.par_iter_mut().for_each(|(ent, mut q)| {
    let mut busy = false;
    while let Some(action) = q.front() {
      if busy && action.waits() {
        break;
      }
      cmd.command_scope(|mut c| {
        let mut ec = c.entity(ent);
        if action.busies() {
          busy = true;
          ec.insert(Busy);
        }
        action.execute(&mut ec);
      });

      q.pop_front();
    }
  });
}
