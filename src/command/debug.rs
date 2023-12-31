use std::fmt::Write;

use anyhow::anyhow;
use bevy::{
  ecs::system::Command,
  prelude::*,
  reflect::serde::TypedReflectDeserializer,
  scene::{
    serde::EntitiesSerializer,
    serialize_ron,
    DynamicEntity,
  },
  utils::HashMap,
};
use serde::de::DeserializeSeed;

use super::{
  CommandArgs,
  WorldCommand,
};
use crate::net::TelnetOut;

fn entities(args: CommandArgs) -> anyhow::Result<WorldCommand> {
  let mut entities = args
    .args
    .split(' ')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .filter_map(|s| s.parse::<u64>().ok())
    .map(Entity::from_bits)
    .collect::<Vec<_>>();
  args
    .caller
    .map(|caller| {
      move |world: &mut World| {
        let out = try_opt!(world.get::<TelnetOut>(caller), return).clone();
        if entities.is_empty() {
          entities = world.query::<Entity>().iter(world).collect::<Vec<_>>();
        } else {
          entities.retain(|e| world.get_entity(*e).is_some());
        }
        let scene = DynamicSceneBuilder::from_world(world)
          .allow_all()
          .extract_entities(entities.into_iter())
          .build();
        let registry = world.resource::<AppTypeRegistry>();
        let serializer = EntitiesSerializer {
          entities: &scene.entities,
          registry,
        };
        let serialized = serialize_ron(serializer).unwrap();
        out.line("Entities:");
        out.line(serialized);
      }
    })
    .map(|f| Box::new(f) as _)
    .ok_or_else(|| anyhow!("missing caller"))
}

fn insert(args: CommandArgs) -> anyhow::Result<WorldCommand> {
  let cmd_args = args
    .args
    .split(' ')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>();

  let usage = || anyhow!("Usage: @insert <entity id> <component name> <component data>");

  let ent = Entity::from_bits(
    cmd_args
      .first()
      .ok_or_else(usage)
      .and_then(|s| Ok(s.parse::<u64>()?))?,
  );
  let component = cmd_args.get(1).ok_or_else(usage)?.to_string();
  let data = cmd_args.get(2).ok_or_else(usage)?.to_string();
  ron::Deserializer::from_str(&data)?;

  Ok(Box::new(move |world: &mut World| {
    let registry = world.resource::<AppTypeRegistry>().clone();
    let out = world
      .get::<TelnetOut>(args.caller.unwrap())
      .unwrap()
      .clone();
    let reg = registry.read();
    try_opt!(world.get_entity(ent), {
      out.line(format!("No such entity: {}", ent.to_bits()));
      return;
    });
    let res = reg
      .get_with_type_path(&component)
      .ok_or_else(|| anyhow!("No such component: '{}'", component))
      .and_then(|info| {
        let de = TypedReflectDeserializer::new(info, &reg);
        let mut seed = ron::Deserializer::from_str(&data).unwrap();
        Ok(de.deserialize(&mut seed)?)
      });
    let component = try_res!(res, err => {
      out.line(format!("Invaid component data: {}", err));
      return;
    });
    let scene = DynamicScene {
      entities: vec![DynamicEntity {
        entity: ent,
        components: vec![component],
      }],
      ..Default::default()
    };
    let mut mappings = HashMap::default();
    mappings.insert(ent, ent);
    try_res!(scene.write_to_world(world, &mut mappings), err => {
      out.line(format!("Failed to insert component: {}", err));
    });
    out.line("Success!");
  }))
}

fn remove(args: CommandArgs) -> anyhow::Result<WorldCommand> {
  let cmd_args = args
    .args
    .split(' ')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>();

  let usage = || anyhow!("Usage: @remove <entity id> <component name>");

  let entity = Entity::from_bits(
    cmd_args
      .first()
      .ok_or_else(usage)
      .and_then(|s| Ok(s.parse::<u64>()?))?,
  );
  let component = cmd_args.get(1).ok_or_else(usage)?.to_string();

  Ok(Box::new(move |world| {
    let out = world
      .get::<TelnetOut>(args.caller.unwrap())
      .unwrap()
      .clone();
    let registry = world.resource::<AppTypeRegistry>().clone();
    let reg = registry.read();
    let entity_mut = world.get_entity_mut(entity);
    let mut entity = try_opt!(entity_mut, {
      out.line(format!("No such entity: {}", entity.to_bits()));
      return;
    });
    let res = reg
      .get_with_type_path(&component)
      .ok_or_else(|| anyhow!("No such component: '{}'", component));
    let info = try_res!(res, err => {
      out.line(format!("{}", err));
      return;
    });
    let reflect_component = try_opt!(info.data::<ReflectComponent>(), return);
    reflect_component.remove(&mut entity);
    out.line("Success!");
  }))
}

fn despawn(args: CommandArgs) -> anyhow::Result<WorldCommand> {
  let cmd_args = args
    .args
    .split(' ')
    .map(|s| s.trim())
    .filter(|s| !s.is_empty())
    .collect::<Vec<_>>();

  let usage = || anyhow!("Usage: @despawn <entity id>");

  let ent = Entity::from_bits(
    cmd_args
      .first()
      .ok_or_else(usage)
      .and_then(|s| Ok(s.parse::<u64>()?))?,
  );
  Ok(Box::new(move |world| {
    let out = world
      .get::<TelnetOut>(args.caller.unwrap())
      .unwrap()
      .clone();
    if let Some(parent) = world.get::<Parent>(ent) {
      world.entity_mut(parent.get()).remove_children(&[ent]);
    }
    if world.get_entity(ent).is_some() {
      DespawnRecursive { entity: ent }.apply(world);
      writeln!(&out, "Despawned entity: {}", ent.to_bits()).unwrap();
    } else {
      writeln!(&out, "No such entity: {}", ent.to_bits()).unwrap();
    }
  }))
}
fn spawn(args: CommandArgs) -> anyhow::Result<WorldCommand> {
  Ok(Box::new(move |world| {
    let out = world
      .get::<TelnetOut>(args.caller.unwrap())
      .unwrap()
      .clone();
    let id = world.spawn_empty().id();
    writeln!(&out, "Spawned new entity: {}", id.to_bits()).unwrap();
  }))
}

command_set! { DebugCommands =>
  ("@entities", entities),
  ("@insert", insert),
  ("@remove", remove),
  ("@spawn", spawn),
  ("@despawn", despawn),
}
