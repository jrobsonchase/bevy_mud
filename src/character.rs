use bevy::{
  prelude::*,
  utils::HashSet,
};
use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  action::Queue,
  core::Live,
  movement::Speed,
  savestate::traits::AppWorldExt,
};

/// Pointer from a character to the player who controls it.
#[derive(Component, Debug, Eq, PartialEq, Reflect, Deref, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Player(pub Entity);

/// Marker for a Non-Player character.
/// Should eventually be a pointer to an AI controller of some sort.
#[derive(Component, Debug, Eq, PartialEq, Reflect, Default, Serialize, Deserialize)]
#[reflect(Component)]
pub struct NonPlayer;

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
/// This can be Player or Non-Player, as determined by the presence
/// of the [Player] or [NonPlayer] component. [Player] will always take
/// precendence, and a character with neither should be despawned.
#[derive(Component, Debug, Reflect, Default, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Character;

pub struct CharacterPlugin;

impl Plugin for CharacterPlugin {
  fn build(&self, app: &mut App) {
    app.register_type::<Puppet>().register_type::<Player>();

    app.persist::<Character>();
    app.persist::<NonPlayer>();

    app.register_type::<Character>();
    app.register_type::<NonPlayer>();
    app.add_systems(Update, despawn_system);
    app
      .observe(puppet_removed_unplayer)
      .observe(player_removed_unpuppet);
  }
}

#[derive(Bundle, Default)]
pub struct NewCharacterBundle {
  base: CharacterBundle,
  character: Character,
  speed: Speed,
}

#[derive(Bundle, Default)]
pub struct CharacterBundle {
  queue: Queue,
}

fn despawn_system(
  mut cmd: Commands,
  query: Query<(Entity, &Character), (Without<Player>, Without<NonPlayer>, With<Live>)>,
) {
  for (ent, _) in query.iter() {
    debug!(?ent, "unloading controllerless character");
    cmd.entity(ent).remove::<Live>();
  }
}

// When Puppet is removed from an entity, find the Player that points to it and detach.
fn puppet_removed_unplayer(
  trigger: Trigger<OnRemove, Puppet>,
  pcs: Query<(Entity, &Player)>,
  mut cmd: Commands,
) {
  let entity = trigger.entity();
  for pc in pcs.iter().filter(|pc| **pc.1 == entity).map(|p| p.0) {
    debug!(entity = %pc, "removing player from orphaned puppet");
    cmd.entity(pc).remove::<Player>();
  }
}

// When Player is removed from an entity, find the Puppet that points to it and detach.
fn player_removed_unpuppet(
  trigger: Trigger<OnRemove, Player>,
  pcs: Query<(Entity, &Puppet)>,
  mut cmd: Commands,
) {
  let entity = trigger.entity();
  for player in pcs.iter().filter(|pc| **pc.1 == entity).map(|p| p.0) {
    debug!(entity = %player, "removing puppet from detached player");
    cmd.entity(player).remove::<Puppet>();
  }
}

fn unpuppet_system(
  mut cmd: Commands,
  mut puppets_removed: RemovedComponents<Puppet>,
  mut players_removed: RemovedComponents<Player>,
  pcs: Query<(Entity, &Player)>,
  players: Query<(Entity, &Puppet)>,
) {
  let puppets_removed: HashSet<Entity> = puppets_removed.read().collect();
  let players_removed: HashSet<Entity> = players_removed.read().collect();
  for (ent, player) in pcs.iter() {
    if puppets_removed.contains(&**player) {
      cmd.entity(ent).remove::<Player>();
    }
  }
  for (ent, puppet) in players.iter() {
    if players_removed.contains(&**puppet) {
      cmd.entity(ent).remove::<Puppet>();
    }
  }
}
