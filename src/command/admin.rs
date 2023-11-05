use super::{
  debug::DebugCommands,
  DynamicCommand,
};

pub fn admin_commands() -> impl Iterator<Item = DynamicCommand> {
  DebugCommands.into_iter()
}
