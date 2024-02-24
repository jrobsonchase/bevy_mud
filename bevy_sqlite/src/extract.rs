use bevy::{
  ecs::system::SystemParam,
  prelude::*,
  scene::DynamicEntity,
};

use super::PersistComponents;

#[allow(dead_code)]
fn extract_entities(
  world: &World,
  entities: impl Iterator<Item = Entity>,
  filter: SceneFilter,
) -> Vec<DynamicEntity> {
  DynamicSceneBuilder::from_world(world)
    .with_filter(filter)
    .deny_all_resources()
    .extract_entities(entities)
    .remove_empty_entities()
    .build()
    .entities
}

#[derive(SystemParam)]
pub struct EntityExtractor<'w> {
  persisted: Res<'w, PersistComponents>,
  world: &'w World,
}

impl<'w> EntityExtractor<'w> {
  pub fn extract_entities(&self, entities: impl Iterator<Item = Entity>) -> Vec<DynamicEntity> {
    extract_entities(self.world, entities, self.persisted.filter())
  }
}
