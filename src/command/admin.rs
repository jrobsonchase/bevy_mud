use anyhow::anyhow;
use bevy::ecs::entity::Entity;

use super::{
  debug::DebugCommands,
  CommandArgs,
  DynamicCommand,
  WorldCommand,
};
use crate::{
  coords::Cubic,
  map::{
    MapCoords,
    MapName,
  },
  net::TelnetOut,
};

pub fn admin_commands() -> impl Iterator<Item = DynamicCommand> {
  DebugCommands.into_iter().chain(AdminCommands)
}

fn teleport(args: CommandArgs) -> anyhow::Result<WorldCommand> {
  let cmd_args = args
    .args
    .split(' ')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>();

  let usage = || anyhow!("Usage: @teleport <entity id> <map name> <q> <r> <s>");

  if cmd_args.len() < 5 {
    return Err(usage());
  }

  let entity_id = Entity::from_bits(
    cmd_args
      .first()
      .ok_or_else(usage)
      .and_then(|s| Ok(s.parse::<u64>()?))?,
  );

  let coords = Cubic(
    cmd_args[cmd_args.len() - 3].parse::<i64>()?,
    cmd_args[cmd_args.len() - 2].parse::<i64>()?,
    cmd_args[cmd_args.len() - 1].parse::<i64>()?,
  );

  let map_name = cmd_args[1..cmd_args.len() - 3].join(" ");

  Ok(Box::new(move |world| {
    let out = world
      .get::<TelnetOut>(args.caller.unwrap())
      .unwrap()
      .clone();
    let entity_mut = world.get_entity_mut(entity_id);
    let mut entity = try_opt!(entity_mut, {
      out.line(format!("No such entity: {}", entity_id.to_bits()));
      return;
    });
    out.line(format!(
      "Moving {:?} to {}, {:?}",
      entity_id, map_name, coords
    ));
    entity.insert((MapName(map_name), MapCoords(coords)));
  }))
}

command_set! { AdminCommands =>
  ("@teleport", teleport),
}
