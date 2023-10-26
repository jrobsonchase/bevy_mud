#![allow(dead_code)]

use std::fmt::Debug;

use bevy::{
  ecs::event::{
    ManualEventIterator,
    ManualEventReader,
  },
  prelude::*,
  utils::HashMap,
};

pub trait EntityEventsExt {
  fn add_entity_event<E: Event>(&mut self) -> &mut Self;
}

impl EntityEventsExt for App {
  fn add_entity_event<E: Event>(&mut self) -> &mut Self {
    self.add_systems(
      First,
      (
        EntityReader::<E>::update_system,
        EntityEvents::<E>::update_system,
      ),
    )
  }
}

#[derive(Component, Debug, Deref, DerefMut)]
pub struct EntityEvents<E: Event>(Events<E>);

impl<E: Event> Default for EntityEvents<E> {
  fn default() -> Self {
    Self(Events::default())
  }
}

impl<E: Event> EntityEvents<E> {
  fn update_system(mut query: Query<&mut EntityEvents<E>>) {
    for mut events in query.iter_mut() {
      events.update();
    }
  }
}

/// A scoped reader for an entity's events.
/// Should either be stored as a component, so that one entity can track the
/// events read from other entities, or (more likely) as a system [Local].
///
/// Internally keeps a HashMap of [ManualEventReader]s for each entity. This map
/// will grow indefinitely if [EntityReader::update] is not called.
#[derive(Component)]
pub struct EntityReader<E: Event> {
  readers: HashMap<Entity, (u8, ManualEventReader<E>)>,
  removed: Vec<Entity>,
}

impl<E: Event> Default for EntityReader<E> {
  fn default() -> Self {
    Self {
      readers: HashMap::default(),
      removed: Vec::default(),
    }
  }
}

impl<E: Event> EntityReader<E> {
  /// Iterate over the events for the given entity that haven't been seen by this reader.
  pub fn read<'a, 'b, 'c>(
    &'a mut self,
    entity: Entity,
    events: &'b EntityEvents<E>,
  ) -> ManualEventIterator<'c, E>
  where
    'a: 'c,
    'b: 'c,
  {
    let (ctr, reader) = self
      .readers
      .entry(entity)
      .or_insert_with(|| (0, events.get_reader()));
    *ctr = 2;
    reader.iter(events)
  }

  /// Drive the cleanup routine.
  /// If using an [EntityReader] as a component, this will be called
  /// automatically. It only needs to be called manually if using a [Local]
  /// [EntityReader].
  pub fn update(&mut self) {
    for (entity, (ctr, _)) in self.readers.iter_mut() {
      if *ctr == 0 {
        self.removed.push(*entity);
      } else {
        *ctr -= 1;
      }
    }

    for entity in self.removed.drain(..) {
      self.readers.remove(&entity);
    }
  }

  fn update_system(mut query: Query<&mut EntityReader<E>>) {
    for mut reader in query.iter_mut() {
      reader.update();
    }
  }
}

pub fn debug_event<T: Event + Debug>(
  mut reader: Local<EntityReader<T>>,
  query: Query<(Entity, &EntityEvents<T>)>,
) {
  reader.update();
  for (entity, events) in query.iter() {
    for event in reader.read(entity, events) {
      debug!(?entity, ?event);
    }
  }
}
