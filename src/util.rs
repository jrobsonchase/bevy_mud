use bevy::{
  ecs::query::WorldQuery,
  prelude::*,
};

#[derive(WorldQuery)]
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
      ent.despawn_recursive();
    }
  }
}
