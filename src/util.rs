use std::fmt::Debug;

use bevy::{
  ecs::query::QueryData,
  prelude::*,
};

pub trait DebugLifecycle {
  fn debug_lifecycle<B>(&mut self, name: &'static str) -> &mut Self
  where
    B: Bundle;
}

impl DebugLifecycle for App {
  fn debug_lifecycle<B>(&mut self, name: &'static str) -> &mut Self
  where
    B: Bundle,
  {
    self
      .observe(debug_lifecycle::<OnAdd, B>("Add", name))
      .observe(debug_lifecycle::<OnRemove, B>("Remove", name))
      .observe(debug_lifecycle::<OnInsert, B>("Insert", name))
  }
}

pub fn debug_lifecycle<E: Event, C: Bundle>(
  action: &'static str,
  name: &'static str,
) -> impl Fn(Trigger<E, C>) {
  move |trigger| {
    let target = trigger.entity().to_bits();
    debug!(target, "{} {}", action, name);
  }
}

pub fn debug_trigger<E: Event + Debug>(trigger: Trigger<E>) {
  let event = trigger.event();
  let target = trigger.entity();
  if target == Entity::PLACEHOLDER {
    debug!(?event)
  } else {
    debug!(?event, target = target.to_bits())
  }
}

pub fn debug_event<E: Event + Debug>(mut reader: EventReader<E>) {
  for event in reader.read() {
    debug!(?event)
  }
}

#[derive(QueryData)]
pub struct HierEntity<'a> {
  pub entity: Entity,
  pub parent: Option<&'a Parent>,
}

impl<'a, 'b> HierEntityItem<'a, 'b> {
  pub fn despawn(&self, cmds: &mut Commands) {
    if let Some(mut parent) = self.parent.and_then(|p| cmds.get_entity(**p)) {
      parent.remove_children(&[self.entity]);
    }
    if let Some(ent) = cmds.get_entity(self.entity) {
      debug!(?self.entity, "despawning recursively");
      ent.despawn_recursive();
    }
  }
}
