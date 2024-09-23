use std::{
  any::TypeId,
  sync::{
    Arc,
    RwLock,
  },
};

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

#[derive(Resource, Default, Clone)]
pub struct PersistentComponents {
  pub components: Arc<RwLock<HashSet<TypeId>>>,
}

#[derive(Debug, Resource, Default)]
pub struct SavedEntityStates {
  pub handle_entities: HashMap<AssetId<SavedEntity>, EntityHashSet>,
}
