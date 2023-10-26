use bevy::prelude::*;

use crate::help::{
  HelpListener,
  PrivateHelp,
};

#[derive(Component)]
pub struct CharacterCommands;

pub struct CharacterPlugin;

fn setup_character_system(mut files: ResMut<PrivateHelp>) {
  files.insert("delete".into(), "delete a character".into());
  files.insert("create".into(), "create a character".into());
  files.insert("character".into(), "load a character".into());
}

fn character_commands_added(mut query: Query<&mut HelpListener, Added<CharacterCommands>>) {
  query.par_iter_mut().for_each(|mut h| {
    h.private_access.insert("delete".into());
    h.private_access.insert("create".into());
    h.private_access.insert("character".into());
  })
}

fn character_commands_removed(
  mut query: Query<&mut HelpListener>,
  mut removed: RemovedComponents<CharacterCommands>,
) {
  for entity in removed.read() {
    if let Ok(mut h) = query.get_mut(entity) {
      h.private_access.remove("delete");
      h.private_access.remove("create");
      h.private_access.remove("character");
    }
  }
}

impl Plugin for CharacterPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(Startup, setup_character_system)
      .add_systems(
        PreUpdate,
        (character_commands_added, character_commands_removed),
      );
  }
}
