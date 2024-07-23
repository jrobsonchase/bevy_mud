use bevy::{
  ecs::entity::{
    EntityHashMap,
    EntityHashSet,
  },
  prelude::*,
  scene::DynamicEntity,
  utils::{
    HashMap,
    HashSet,
  },
};

use super::{
  entity::SavedEntity,
  loader::{
    SavedEntitySeed,
    SavedEntityVisitor,
  },
};
use crate::savestate::entity::Save;

pub fn asset_events(
  mut reader: EventReader<AssetEvent<SavedEntity>>,
  mappings: Res<SavedEntityStates>,
  assets: Res<Assets<SavedEntity>>,
  query: Query<&Handle<SavedEntity>>,
  mut cmd: Commands,
) {
  for event in reader.read() {
    match event {
      AssetEvent::LoadedWithDependencies { id } | AssetEvent::Modified { id } => {
        debug!(%id, "saved entity loaded");
        let Some(saved) = assets.get(*id) else {
          continue;
        };
        for entity in mappings
          .handle_entities
          .get(id)
          .into_iter()
          .flat_map(|h| h.iter())
        {
          cmd.add(write_saved_entity(*entity, saved));
        }
      }
      _ => {}
    }
    debug!(?event, "finished SavedEntity asset event");
  }
}

pub(crate) fn saved_entity_added(
  trigger: Trigger<OnAdd, Handle<SavedEntity>>,
  mut states: ResMut<SavedEntityStates>,
  assets: Res<Assets<SavedEntity>>,
  query: Query<&Handle<SavedEntity>>,
  mut cmd: Commands,
) {
  let entity = trigger.entity();
  let handle = query.get(entity).unwrap();

  states
    .mappings
    .entry(entity)
    .or_insert_with(Default::default)
    .asset_id = handle.id();
  states
    .handle_entities
    .entry(handle.id())
    .or_insert_with(Default::default)
    .insert(entity);

  let Some(asset) = assets.get(handle) else {
    // warn!("no saved entity found for {handle:?}");
    return;
  };

  cmd.add(write_saved_entity(entity, asset));
}

fn write_saved_entity(entity: Entity, saved: &SavedEntity) -> impl FnOnce(&mut World) + 'static {
  let mut scene = DynamicScene::default();
  scene.entities.push(DynamicEntity {
    entity: Entity::PLACEHOLDER,
    components: saved.components.iter().map(|r| r.clone_value()).collect(),
  });
  scene
    .entities
    .extend(saved.entities.iter().map(|(id, components)| DynamicEntity {
      entity: id.clone(),
      components: components.iter().map(|r| r.clone_value()).collect(),
    }));
  let saved = format!("{saved:?}");
  move |world: &mut World| {
    debug!(%entity, saved, "writing saved entity to world");
    world.resource_scope::<SavedEntityStates, _>(|world, mut storage| {
      let state = storage
        .mappings
        .entry(entity)
        .or_insert_with(Default::default);

      let current_mappings = &mut state.entity_mappings;
      let new_mappings = &mut state.new_entity_mappings;
      new_mappings.clear();

      // Populate the new mappings from the entities and previous mappings, if they exist
      for entity in &scene.entities {
        if let Some(target) = current_mappings.get(&entity.entity).copied() {
          new_mappings.insert(entity.entity, target);
        }
      }

      if let Err(error) = scene.write_to_world(world, new_mappings) {
        warn!(%error, "error spawning saved entity");
        return;
      }

      // Remove all of the new mappings from the current mappings, leaving only
      // the no-longer-referenced entities
      for entity in new_mappings.keys() {
        current_mappings.remove(entity);
      }

      // Despawn all of the no-longer-referenced entities that aren't Saved
      for entity in current_mappings.values().copied() {
        if !world.get::<Save>(entity).is_some() {
          debug!(
            entity = entity.to_bits(),
            "despawning no longer referenced entity"
          );
          world.despawn(entity);
        }
      }

      let registry = world.resource::<AppTypeRegistry>().clone();
      let registry_read = registry.read();

      for entity in scene.entities {
        let Some(world_entity) = new_mappings.get(&entity.entity).copied() else {
          continue;
        };
        let entity_ref = world.entity(world_entity);

        let mut world_components = entity_ref.archetype().components().collect::<HashSet<_>>();
        entity
          .components
          .iter()
          .filter_map(|c| {
            let reg = registry_read.get_with_type_path(c.reflect_type_path())?;
            world.components().get_id(reg.type_id())
          })
          .for_each(|id| {
            world_components.remove(&id);
          });

        let mut world_entity = world.entity_mut(world_entity);
        for component_id in world_components {
          debug!(
            entity = world_entity.id().to_bits(),
            "removing component not in save file"
          );
          world_entity.remove_by_id(component_id);
        }
      }

      std::mem::swap(current_mappings, new_mappings);
    });
    debug!(entity = entity.to_bits(), "finished writing saved entity");
  }
}

#[derive(Debug, Resource, Default)]
pub(crate) struct SavedEntityStates {
  mappings: EntityHashMap<SavedEntityState>,
  handle_entities: HashMap<AssetId<SavedEntity>, EntityHashSet>,
}

#[derive(Debug, Default)]
pub(crate) struct SavedEntityState {
  /// Mappings from the save file to the world.
  entity_mappings: EntityHashMap<Entity>,
  new_entity_mappings: EntityHashMap<Entity>,

  /// The asset representing this entity.
  asset_id: AssetId<SavedEntity>,
}
