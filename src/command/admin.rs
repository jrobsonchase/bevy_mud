use anyhow::anyhow;
use bevy::ecs::entity::Entity;
use hexx::hex;

use super::{
  debug::DebugCommands,
  CommandArgs,
  DynamicCommand,
  WorldCommand,
};
use crate::{
  map::Transform,
  net::TelnetOut,
};

pub fn admin_commands() -> impl Iterator<Item = DynamicCommand> {
  DebugCommands.into_iter().chain(AdminCommands)
}

fn rotate_ent(args: CommandArgs) -> anyhow::Result<WorldCommand> {
  let cmd_args = args
    .args
    .split(' ')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>();

  let usage = || anyhow!("Usage: @rotate <entity id> <r>");

  if cmd_args.len() < 2 {
    return Err(usage());
  }

  let entity_id = Entity::from_bits(cmd_args[0].parse::<u64>()?);

  let rotation = cmd_args[1].parse::<i8>()?;

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
    out.line(format!("Rotating {:?} by {}", entity_id, rotation));
    if let Some(mut xform) = entity.get_mut::<Transform>() {
      xform.facing = (xform.facing + rotation) % 6;
    } else {
      entity.insert(Transform {
        facing: rotation,
        ..Default::default()
      });
    }
  }))
}

fn move_ent(args: CommandArgs) -> anyhow::Result<WorldCommand> {
  let cmd_args = args
    .args
    .split(' ')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>();

  let usage = || anyhow!("Usage: @move <entity id> <q> <r> <s>");

  if cmd_args.len() < 4 {
    return Err(usage());
  }

  let entity_id = Entity::from_bits(cmd_args[0].parse::<u64>()?);

  let coords = hex(cmd_args[1].parse()?, cmd_args[2].parse()?);

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
    out.line(format!("Moving {:?} by {:?}", entity_id, coords));
    if let Some(mut xform) = entity.get_mut::<Transform>() {
      xform.coords += coords;
    } else {
      entity.insert(Transform {
        coords,
        ..Default::default()
      });
    }
  }))
}

fn teleport_ent(args: CommandArgs) -> anyhow::Result<WorldCommand> {
  let cmd_args = args
    .args
    .split(' ')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>();

  let usage = || anyhow!("Usage: @teleport <entity id> <q> <r> <s> [<map name>]");

  if cmd_args.len() < 4 {
    return Err(usage());
  }

  let entity_id = Entity::from_bits(cmd_args[0].parse::<u64>()?);

  let coords = hex(cmd_args[1].parse()?, cmd_args[2].parse()?);

  let map_name = Some(cmd_args[4..].join(" "))
    .filter(|s| !s.is_empty())
    .map(|s| if s == "None" { "".into() } else { s });

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
      "Moving {:?} to {:?}, {:?}",
      entity_id, map_name, coords
    ));
    if let Some(mut xform) = entity.get_mut::<Transform>() {
      if let Some(map) = map_name {
        xform.map = map;
      }
      xform.coords = coords;
    } else {
      entity.insert(Transform {
        map: map_name.unwrap_or("default".into()),
        coords,
        ..Default::default()
      });
    }
  }))
}

command_set! { AdminCommands =>
  ("@teleport", teleport_ent),
  ("@move", move_ent),
  ("@rotate", rotate_ent),
}
