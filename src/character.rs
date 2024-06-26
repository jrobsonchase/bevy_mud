use bevy::{
  prelude::*,
  utils::HashSet,
};
use bevy_replicon::core::replication_rules::AppRuleExt;
use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  action::Queue,
  core::Live,
  movement::Speed,
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

    app.replicate::<Character>();
    app.replicate::<NonPlayer>();

    app.register_type::<Character>();
    app.register_type::<NonPlayer>();
    app.add_systems(
      Update,
      unpuppet_system
        .before(despawn_system)
        .run_if(any_component_removed::<Player>().or_else(any_component_removed::<Puppet>())),
    );
    app.add_systems(Update, despawn_system);
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
  children: Query<&Children>,
) {
  for (ent, _) in query.iter() {
    debug!(?ent, "unloading controllerless character");
    for child in children.iter_descendants(ent) {
      cmd.entity(child).remove::<Live>();
    }
    cmd.entity(ent).remove::<Live>();
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
      debug!(?ent, "removing player from orphaned puppet");
      cmd.entity(ent).remove::<Player>();
    }
  }
  for (ent, puppet) in players.iter() {
    if players_removed.contains(&**puppet) {
      debug!(?ent, "removing puppet from detached player");
      cmd.entity(ent).remove::<Puppet>();
    }
  }
}
