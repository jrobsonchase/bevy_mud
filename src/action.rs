use std::{
  any::Any,
  collections::VecDeque,
  fmt::Debug,
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

#[derive(Event, Debug, Clone, Copy, Eq, PartialEq)]
pub struct StopEvent;

pub trait Action: Debug + Any + Send + Sync + 'static {
  /// Execute the action.
  fn execute(&self, cmd: EntityCommands);

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
      debug!(entity = %ent, ?action, "running action");
      if busy && action.waits() {
        break;
      }
      cmd.command_scope(|mut c| {
        let mut ec = c.entity(ent);
        if action.busies() {
          debug!(entity = %ent, "busying");
          busy = true;
          ec = ec.insert(Busy);
        }
        action.execute(ec);
      });

      q.pop_front();
    }
  });
}
