use anyhow::bail;
pub mod admin;
pub mod debug;
pub mod player;

use std::{
  fmt,
  fmt::Debug,
  sync::Arc,
};

use bevy::{
  ecs::system::EntityCommands,
  prelude::*,
};
use radix_trie::{
  Trie,
  TrieCommon,
};

use self::{
  admin::admin_commands,
  player::PlayerCommands,
};
use crate::{
  account::Session,
  net::{
    TelnetIn,
    TelnetOut,
  },
};

pub struct GameCommandsPlugin;

impl Plugin for GameCommandsPlugin {
  fn build(&self, app: &mut App) {
    app
      .add_systems(PreUpdate, game_commands_system)
      .add_systems(PreUpdate, add_command_sets_system);
  }
}

pub fn add_command_sets_system(
  mut cmd: Commands,
  new_sessions: Query<(Entity, &Session, &TelnetOut), Added<Session>>,
) {
  for (entity, sess, output) in new_sessions.iter() {
    let mut ecmd = cmd.entity(entity);
    ecmd.add_game_commands(PlayerCommands);
    if sess.admin {
      ecmd.add_game_commands(admin_commands());
      output.line("admin commands enabled");
    }
    output.string("> ");
    command!(output, GA);
  }
}

pub fn game_commands_system(world: &mut World) {
  let mut query = world.query::<(Entity, &mut TelnetIn, &TelnetOut, &CommandSet)>();
  let mut cmds = vec![];
  for (entity, mut telnet_in, telnet_out, cmdset) in query.iter_mut(world) {
    while let Some(line) = telnet_in.next_line() {
      let (cmd_str, args) = line
        .split_once(' ')
        .map(|(cmd, rest)| (cmd.trim(), rest.trim()))
        .unwrap_or_else(|| (line.as_str().trim(), ""));
      match cmdset
        .lookup(cmd_str)
        .ok_or_else(|| anyhow::format_err!("What?"))
        .and_then(|cmd| {
          cmd.build(CommandArgs {
            caller: Some(entity),
            owner: Some(entity),
            matched: cmd_str,
            args,
          })
        }) {
        Ok(cmd) => cmds.push((entity, cmd)),
        Err(err) => {
          telnet_out.line(format!("{}", err)).string("> ");
          command!(telnet_out, GA);
        }
      }
    }
  }
  for (entity, cmd) in cmds {
    cmd(world);
    let out = world.get_mut::<TelnetOut>(entity).unwrap();
    out.string("> ");
    command!(out, GA);
  }
}

#[derive(Clone, Deref)]
pub struct DynamicCommand(Arc<dyn GameCommand>);

impl fmt::Debug for DynamicCommand {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    f.debug_tuple("DynamicCommand").field(&self.key()).finish()
  }
}

impl<C> From<C> for DynamicCommand
where
  C: GameCommand + 'static,
{
  fn from(value: C) -> Self {
    DynamicCommand(Arc::new(value))
  }
}

pub struct CommandArgs<'a> {
  pub caller: Option<Entity>,
  pub owner: Option<Entity>,
  pub matched: &'a str,
  pub args: &'a str,
}

pub type WorldCommand = Box<dyn FnOnce(&mut World)>;

/// A game command, usually run by a player, but not necessarily.
///
/// Commands are always in the form `<command> <arguments>`, without silly
/// things like preceding adverbs or prepositional phrases.
pub trait GameCommand: Send + Sync {
  /// The command itself in long form.
  ///
  /// Incoming commands may be matched against this key in prefix fashion.
  fn key(&self) -> &str;

  /// Once a command has been matched, potentially build it into a world
  /// modification.
  ///
  /// The `caller` argument, if set, refers to the person or thing that sent the
  /// command. This is the player, _not_ the player's character. If not a
  /// player, it may be an AI controlling an NPC.
  ///
  /// `matched` will contain the matched command. May be a prefix of the
  /// command's [GameCommand::key].
  ///
  /// The `arg_string` contains the arguments _minus_ the matched command and
  /// following whitespace.
  #[allow(unused_variables)]
  fn build(&self, args: CommandArgs) -> anyhow::Result<WorldCommand> {
    bail!("What?");
  }
}

impl<'a, C> GameCommand for &'a C
where
  C: GameCommand,
{
  fn key(&self) -> &str {
    C::key(self)
  }

  fn build(&self, args: CommandArgs) -> anyhow::Result<WorldCommand> {
    C::build(self, args)
  }
}

impl<'a, F> GameCommand for (&'a str, F)
where
  F: Fn(CommandArgs<'_>) -> anyhow::Result<WorldCommand> + Send + Sync,
{
  fn key(&self) -> &str {
    self.0
  }

  fn build(&self, args: CommandArgs) -> anyhow::Result<WorldCommand> {
    (self.1)(args)
  }
}

#[derive(Component, Default, Clone, Debug)]
pub struct CommandSet {
  commands: Trie<String, DynamicCommand>,
}

impl CommandSet {
  pub fn add_command(&mut self, cmd: impl Into<DynamicCommand>) {
    let cmd = cmd.into();
    self.commands.insert(cmd.key().into(), cmd);
  }

  pub fn lookup(&self, cmd_string: &str) -> Option<&DynamicCommand> {
    let subtrie = self.commands.get_raw_descendant(cmd_string);
    subtrie.and_then(|st| st.value())
  }

  pub fn commands(&self) -> impl Iterator<Item = DynamicCommand> + '_ {
    self.commands.iter().map(|(_, val)| val.clone())
  }

  pub fn merge<C: Into<DynamicCommand>>(&mut self, other: impl IntoIterator<Item = C>) {
    for cmd in other {
      self.add_command(cmd);
    }
  }
}

impl<C> FromIterator<C> for CommandSet
where
  C: Into<DynamicCommand>,
{
  fn from_iter<T: IntoIterator<Item = C>>(iter: T) -> Self {
    let mut set = CommandSet::default();
    set.merge(iter);
    set
  }
}

pub trait EntityCommandsExt {
  fn add_game_commands<C: Into<DynamicCommand>>(
    &mut self,
    commands: impl IntoIterator<Item = C> + Send + 'static,
  ) -> &mut Self;
}

impl<'w> EntityCommandsExt for EntityWorldMut<'w> {
  fn add_game_commands<C: Into<DynamicCommand>>(
    &mut self,
    commands: impl IntoIterator<Item = C> + Send + 'static,
  ) -> &mut Self {
    if let Some(mut set) = self.get_mut::<CommandSet>() {
      set.merge(commands.into_iter().map(Into::into));
    } else {
      self.insert(commands.into_iter().collect::<CommandSet>());
    }
    self
  }
}

impl<'w, 's, 'a> EntityCommandsExt for EntityCommands<'w, 's, 'a> {
  fn add_game_commands<C: Into<DynamicCommand>>(
    &mut self,
    commands: impl IntoIterator<Item = C> + Send + 'static,
  ) -> &mut Self {
    self.add(move |mut entity_world: EntityWorldMut| {
      entity_world.add_game_commands(commands);
    })
  }
}

#[cfg(test)]
mod test {
  use super::*;

  struct TestCmd;
  impl GameCommand for TestCmd {
    fn key(&self) -> &'static str {
      "test"
    }
  }
  struct TestCmd2;
  impl GameCommand for TestCmd2 {
    fn key(&self) -> &'static str {
      "testcommand"
    }
  }
  struct TestCmd3;
  impl GameCommand for TestCmd3 {
    fn key(&self) -> &'static str {
      "help"
    }
  }

  #[test]
  fn test_matching() {
    let mut set = CommandSet::default();
    set.add_command(TestCmd2);
    set.add_command(TestCmd);
    set.add_command(TestCmd3);

    assert_eq!(set.lookup("test").map(|c| c.key()), Some("test"));
    assert_eq!(set.lookup("testc").map(|c| c.key()), Some("testcommand"));
    assert_eq!(set.lookup("testd").map(|c| c.key()), None);
    assert_eq!(set.lookup("te").map(|c| c.key()), Some("test"));
    assert!(set.lookup("foo").is_none());
  }
}
