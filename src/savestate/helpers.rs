use std::sync::PoisonError;

use bevy::{
  prelude::*,
  scene::DynamicEntity,
  utils::HashSet,
};

use super::{
  assets::SavedEntity,
  components::Save,
  resources::SavedEntityStates,
};
use crate::savestate::{
  components::SavedEntityState,
  resources::PersistentComponents,
};

pub fn write_saved_entity(
  entity: Entity,
  saved: &SavedEntity,
) -> impl FnOnce(&mut World) + 'static {
  let mut scene = DynamicScene::default();
  scene.entities.push(DynamicEntity {
    entity: Entity::PLACEHOLDER,
    components: saved.components.iter().map(|r| r.clone_value()).collect(),
  });
  scene
    .entities
    .extend(saved.entities.iter().map(|(id, components)| DynamicEntity {
      entity: *id,
      components: components.iter().map(|r| r.clone_value()).collect(),
    }));
  let saved = format!("{saved:?}");
  move |world: &mut World| {
    debug!(?entity, saved, "writing saved entity to world");
    let Some(mut world_state) = world.get_mut::<SavedEntityState>(entity) else {
      warn!(?entity, "saved entity missing state");
      return;
    };
    let mut state = Default::default();
    std::mem::swap(&mut state, &mut *world_state);

    let current_mappings = &mut state.entity_mappings;
    let new_mappings = &mut state.new_entity_mappings;
    new_mappings.clear();
    new_mappings.insert(Entity::PLACEHOLDER, entity);

    // Populate the new mappings from the entities and previous mappings, if they exist
    for entity in &scene.entities {
      if let Some(target) = current_mappings.get(&entity.entity).copied() {
        new_mappings.insert(entity.entity, target);
      }
    }

    if let Err(error) = scene.write_to_world(world, new_mappings) {
      warn!(%error, "error spawning saved entity");
      std::mem::swap(
        &mut state,
        &mut *world.get_mut::<SavedEntityState>(entity).unwrap(),
      );
      return;
    }

    debug!(?entity, ?new_mappings, "savestate mappings");

    // Remove all of the new mappings from the current mappings, leaving only
    // the no-longer-referenced entities
    for entity in new_mappings.keys() {
      current_mappings.remove(entity);
    }

    // Despawn all of the no-longer-referenced entities that aren't Saved
    for entity in current_mappings.values().copied() {
      if world.get::<Save>(entity).is_none() {
        debug!(?entity, "despawning no longer referenced entity");
        world.despawn(entity);
      }
    }

    let registry = world.resource::<AppTypeRegistry>().clone();
    let persistent = world.resource::<PersistentComponents>().clone();
    let registry_read = registry.read();
    let persist_read = persistent
      .components
      .read()
      .unwrap_or_else(PoisonError::into_inner);

    for entity in scene.entities {
      let Some(world_entity) = new_mappings.get(&entity.entity).copied() else {
        continue;
      };
      let entity_ref = world.entity(world_entity);

      let mut remove_components = entity_ref.archetype().components().collect::<HashSet<_>>();
      entity
        .components
        .iter()
        .filter_map(|c| c.get_represented_type_info())
        .filter_map(|c| registry_read.get_with_type_path(c.type_path()))
        .filter_map(|reg| world.components().get_id(reg.type_id()))
        .for_each(|id| {
          remove_components.remove(&id);
        });

      remove_components.retain(|c| {
        let Some(info) = world.components().get_info(*c) else {
          return false;
        };
        let Some(type_id) = info.type_id() else {
          return false;
        };
        persist_read.contains(&type_id)
      });

      let mut world_entity = world.entity_mut(world_entity);
      for id in remove_components {
        world_entity.remove_by_id(id);
      }
    }

    std::mem::swap(current_mappings, new_mappings);
    std::mem::swap(
      &mut state,
      &mut *world.get_mut::<SavedEntityState>(entity).unwrap(),
    );
    debug!(?entity, "finished writing saved entity");
  }
}
