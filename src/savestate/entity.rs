use std::path::PathBuf;

use bevy::{
  asset::AssetPath,
  ecs::entity::EntityHashMap,
  prelude::*,
};
use serde::{
  Deserialize,
  Serialize,
};

#[derive(Asset, Debug, Default, TypePath)]
pub struct SavedEntity {
  pub components: Vec<Box<dyn Reflect>>,
  pub entities: EntityHashMap<Vec<Box<dyn Reflect>>>,
}

#[derive(Component, Debug, Clone, Reflect, Deref, Default, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Save(PathBuf);

impl<'a> Into<AssetPath<'static>> for &'a Save {
  fn into(self) -> AssetPath<'static> {
    self.0.clone().into()
  }
}
