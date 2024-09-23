use std::any::TypeId;

use bevy::{
  ecs::{
    component::ComponentId,
    entity::{
      EntityHashMap,
      EntityHashSet,
    },
  },
  prelude::*,
  utils::{
    HashMap,
    HashSet,
  },
};

use super::assets::SavedEntity;

#[derive(Resource, Copy, Clone, Deref)]
pub struct SaveInterval(pub f32);

#[derive(Resource, Default, Reflect)]
#[reflect(Resource)]
pub struct PersistentComponents {
  pub components: HashSet<TypeId>,
}

#[derive(Debug, Resource, Default)]
pub struct SavedEntityStates {
  pub handle_entities: HashMap<AssetId<SavedEntity>, EntityHashSet>,
}
