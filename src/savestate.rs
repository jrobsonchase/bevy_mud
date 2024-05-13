use std::{
  io::Write,
  time::{
    Duration,
    Instant,
  },
};

use bevy::{
  app::AppExit,
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
use bevy_replicon::core::Replicated;
use serde::de::DeserializeSeed;

use crate::{
  account::UserDb,
  core::MudStartup,
};

#[derive(Debug, Clone, Copy)]
pub struct SaveStatePlugin(f32);

#[derive(Resource, Copy, Clone, Deref)]
struct SaveInterval(f32);

impl Default for SaveStatePlugin {
  fn default() -> Self {
    Self::with_interval(SAVE_INTERVAL)
  }
}

impl SaveStatePlugin {
  pub fn with_interval(secs: f32) -> Self {
    Self(secs)
  }
}

const SAVE_INTERVAL: f32 = 30.0;

impl Plugin for SaveStatePlugin {
  fn build(&self, app: &mut App) {
    app
      .add_plugins(bevy_replicon::RepliconPlugins)
      .add_event::<LoadFailed>()
      .insert_resource(SaveInterval(self.0))
      .add_systems(Startup, load_system.in_set(MudStartup::System))
      .add_systems(
        Last,
        final_save_system.run_if(on_event::<AppExit>().and_then(not(on_event::<LoadFailed>()))),
      )
      .add_systems(Last, save_system.run_if(not(on_event::<AppExit>())));
  }
}

#[derive(Event)]
struct LoadFailed;

fn load_system(world: &mut World) {
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
      entity.components.push(Replicated.clone_value());
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
    world.send_event(LoadFailed);
    world.send_event(AppExit);
  }
}

fn final_save_system(world: &World) {
  let registry = world.resource::<AppTypeRegistry>().clone();
  let mut scene = DynamicSceneBuilder::from_world(&world)
    .deny_all()
    .deny_all_resources()
    .allow_resource::<UserDb>()
    .extract_resources()
    .build();
  bevy_replicon::scene::replicate_into(&mut scene, world);
  let serialized = match scene.serialize_ron(&registry) {
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

fn save_system(
  interval: Res<SaveInterval>,
  time: Res<Time>,
  mut stop: Local<Stopwatch>,
  world: &World,
) {
  stop.tick(time.delta());
  if stop.elapsed_secs() > **interval {
    let extract_start = Instant::now();
    info!("saving world");
    stop.reset();
    let registry = world.resource::<AppTypeRegistry>().clone();
    let mut scene = DynamicSceneBuilder::from_world(&world)
      .deny_all()
      .deny_all_resources()
      .allow_resource::<UserDb>()
      .extract_resources()
      .build();
    bevy_replicon::scene::replicate_into(&mut scene, world);
    let extract_time = Instant::now().duration_since(extract_start);
    AsyncComputeTaskPool::get()
      .spawn(async move {
        let serialize_start = Instant::now();
        let serialized = match scene.serialize_ron(&registry) {
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
        if extract_time + serialize_time + write_time > Duration::from_secs_f32(SAVE_INTERVAL) {
          warn!("save took longer than save interval");
        }
      })
      .detach();
  }
}
