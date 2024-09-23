use std::{
  io::Write,
  time::{
    Duration,
    Instant,
  },
};

use bevy::{
  ecs::entity::{
    EntityHashMap,
    MapEntities,
    SceneEntityMapper,
  },
  prelude::*,
  scene::serde::SceneDeserializer,
  tasks::AsyncComputeTaskPool,
  time::Stopwatch,
};
use serde::de::DeserializeSeed;

use super::{
  assets::SavedEntity,
  components::{
    self,
    Persistent,
  },
  helpers::write_saved_entity,
  resources::{
    self,
    PersistentComponents,
    SavedEntityStates,
  },
};
use crate::{
  account::UserDb,
  savestate::events,
};

pub fn handle_asset_events(
  mut reader: EventReader<AssetEvent<SavedEntity>>,
  mappings: Res<SavedEntityStates>,
  assets: Res<Assets<SavedEntity>>,
  query: Query<&Handle<SavedEntity>>,
  mut cmd: Commands,
) {
  for event in reader.read() {
    match event {
      AssetEvent::LoadedWithDependencies { id } | AssetEvent::Modified { id } => {
        debug!(%id, "saved entity loaded");
        let Some(saved) = assets.get(*id) else {
          continue;
        };
        for entity in mappings
          .handle_entities
          .get(id)
          .into_iter()
          .flat_map(|h| h.iter())
        {
          cmd.queue(write_saved_entity(*entity, saved));
        }
      }
      _ => {}
    }
    debug!(?event, "finished SavedEntity asset event");
  }
}

pub fn load_system(world: &mut World) {
  let registry = world.resource::<AppTypeRegistry>().clone();
  let reg_read = registry.read();
  let seed = SceneDeserializer {
    type_registry: &reg_read,
  };
  let res = (|| {
    let serialized = std::fs::read_to_string("world.ron")?;
    let mut de = ron::Deserializer::from_str(&serialized)?;
    let mut scene = seed.deserialize(&mut de)?;

    for entity in &mut scene.entities {
      entity.components.push(components::Persistent.clone_value());
    }
    let mut entities = EntityHashMap::default();

    scene.write_to_world(world, &mut entities)?;
    world.resource_scope::<UserDb, ()>(|world, mut db| {
      SceneEntityMapper::world_scope(&mut entities, world, |_, mapper| {
        db.map_entities(mapper);
      });
    });

    Result::<_, anyhow::Error>::Ok(())
  })();

  if let Err(error) = res {
    error!(%error, "failed to load world from file, exiting");
    world.send_event(events::LoadFailed);
    world.send_event(AppExit::Error(1.try_into().unwrap()));
  }
}

fn extract_save(world: &World, entities: Query<Entity, With<Persistent>>) -> DynamicScene {
  let persistent_components = world.resource::<PersistentComponents>();
  DynamicSceneBuilder::from_world(world)
    .deny_all()
    .deny_all_resources()
    .allow_resource::<UserDb>()
    .with_component_filter(SceneFilter::Allowlist(
      persistent_components.components.clone(),
    ))
    .extract_resources()
    .extract_entities(entities.iter())
    .build()
}

pub fn final_save_system(world: &World, entities: Query<Entity, With<Persistent>>) {
  let registry = world.resource::<AppTypeRegistry>().clone();
  let scene = extract_save(world, entities);
  let serialized = match scene.serialize(&registry.read()) {
    Ok(s) => s,
    Err(error) => {
      warn!(%error, "error serializing world");
      return;
    }
  };
  let mut f = match std::fs::File::create("world.ron") {
    Ok(f) => f,
    Err(error) => {
      warn!(%error, "error opening world file for writing");
      return;
    }
  };
  if let Err(error) = f.write_all(serialized.as_bytes()) {
    warn!(%error, "error writing world file");
  }
}

pub fn save_system(
  interval: Res<resources::SaveInterval>,
  time: Res<Time>,
  mut stop: Local<Stopwatch>,
  world: &World,
  entities: Query<Entity, With<Persistent>>,
) {
  stop.tick(time.delta());
  if stop.elapsed_secs() > **interval {
    let extract_start = Instant::now();
    info!("saving world");
    stop.reset();
    let registry = world.resource::<AppTypeRegistry>().clone();
    let scene = extract_save(world, entities);
    let extract_time = Instant::now().duration_since(extract_start);
    AsyncComputeTaskPool::get()
      .spawn(async move {
        let serialize_start = Instant::now();
        let serialized = match scene.serialize(&registry.read()) {
          Ok(s) => s,
          Err(error) => {
            warn!(%error, "error serializing world");
            return;
          }
        };
        let serialize_time = Instant::now().duration_since(serialize_start);
        let mut f = match std::fs::File::create("world.ron") {
          Ok(f) => f,
          Err(error) => {
            warn!(%error, "error opening world file for writing");
            return;
          }
        };
        let write_start = Instant::now();
        if let Err(error) = f.write_all(serialized.as_bytes()) {
          warn!(%error, "error writing world file");
        }
        let _ = f.flush();
        drop(f);
        let write_time = Instant::now().duration_since(write_start);
        info!(?extract_time, ?serialize_time, ?write_time, "world saved");
        if extract_time + serialize_time + write_time
          > Duration::from_secs_f32(super::SAVE_INTERVAL)
        {
          warn!("save took longer than save interval");
        }
      })
      .detach();
  }
}
