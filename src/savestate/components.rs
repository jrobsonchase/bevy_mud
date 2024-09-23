use std::path::PathBuf;

use bevy::{
  ecs::entity::EntityHashMap,
  prelude::*,
};
use serde::{
  Deserialize,
  Serialize,
};

#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Persistent;

#[derive(Component, Debug, Clone, Reflect, Deref, Default, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Save(pub PathBuf);

#[derive(Component, Debug, Default)]
pub struct SavedEntityState {
  /// Mappings from the save file to the world.
  pub entity_mappings: EntityHashMap<Entity>,
  pub new_entity_mappings: EntityHashMap<Entity>,
}
