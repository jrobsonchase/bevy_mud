use bevy::{
  prelude::*,
  utils::{
    HashMap,
    HashSet,
  },
};

use crate::{
  command::CommandQueue,
  net::TelnetOut,
};

#[derive(Resource, Default, Deref, DerefMut)]
pub struct PublicHelp(HashMap<String, String>);

#[derive(Resource, Default, Deref, DerefMut)]
pub struct PrivateHelp(HashMap<String, String>);

#[derive(Component, Default)]
pub struct HelpListener {
  pub private_access: HashSet<String>,
}

pub fn help_system(
  public_files: Res<PublicHelp>,
  private_files: Res<PrivateHelp>,
  mut query: Query<(&HelpListener, &mut CommandQueue, &TelnetOut)>,
) {
  query.par_iter_mut().for_each(|(listener, mut cmds, out)| {
    if let Some("help") = cmds.first_command() {
      let args = cmds.dequeue().unwrap();
      let key = args[1..].join(" ");
      if listener.private_access.contains(&key) {
        if let Some(content) = private_files.0.get(&key) {
          out.line(content);
          return;
        } else {
          warn!(key, "failed to find private helpfile");
        }
      }
      if let Some(content) = public_files.0.get(&key) {
        out.line(content);
        return;
      }
      out.line(format!("No help found for \"{key}\"."));
    }
  })
}

pub struct HelpPlugin;

impl Plugin for HelpPlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(PublicHelp::default())
      .insert_resource(PrivateHelp::default())
      .add_systems(Update, help_system);
  }
}
