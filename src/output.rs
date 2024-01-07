use bevy::{
  ecs::system::SystemParam,
  prelude::*,
};

use crate::{
  character::Player,
  net::TelnetOut,
};

#[derive(SystemParam)]
pub struct PlayerOutput<'w, 's> {
  players: Query<'w, 's, &'static TelnetOut>,
  puppets: Query<'w, 's, &'static Player>,
}

impl<'w, 's> PlayerOutput<'w, 's> {
  pub fn get(&self, entity: Entity) -> Option<&TelnetOut> {
    self
      .players
      .get(entity)
      .or_else(|_| self.puppets.get(entity).and_then(|p| self.players.get(**p)))
      .ok()
  }
}
