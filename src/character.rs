use bevy::prelude::*;

use crate::savestate::SaveExt;

/// Pointer from a character to the player who controls it.
#[derive(Component, Debug, Eq, PartialEq, Reflect, Deref)]
#[reflect(Component)]
pub struct Player(pub Entity);

impl Default for Player {
  fn default() -> Self {
    Player(Entity::PLACEHOLDER)
  }
}

/// Pointer from a player to the character they're controlling.
#[derive(Component, Deref, Debug, Reflect, Clone, Copy)]
#[reflect(Component)]
pub struct Puppet(pub Entity);

impl Default for Puppet {
  fn default() -> Self {
    Puppet(Entity::PLACEHOLDER)
  }
}

/// Marker for a character entity.
/// This can be Player or Non-Player, as determined by the presence or absence
/// of the [Player] component.
#[derive(Component, Debug, Reflect, Default)]
#[reflect(Component)]
pub struct Character;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Puppet>().register_type::<Player>();
    app.persist_component::<Character>();
  }
}
