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
use bevy_replicon::{
  core::{
    replication_rules::AppRuleExt,
    Replicated,
  },
  parent_sync::ParentSync,
};
use serde::{
  de::DeserializeSeed,
  Serialize,
};

use self::{
  entity::{
    Save,
    SavedEntity,
  },
  loader::SavedEntityLoader,
};
use crate::{
  account::UserDb,
  core::MudStartup,
  util::DebugLifecycle,
};

pub mod entity;
pub mod loader;
pub mod systems;

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
      .register_type::<Save>()
      .init_asset::<SavedEntity>()
      .init_asset_loader::<SavedEntityLoader>()
      .add_plugins(bevy_replicon::RepliconPlugins)
      .add_event::<LoadFailed>()
      .insert_resource(SaveInterval(self.0))
      .insert_resource(systems::SavedEntityStates::default())
      .add_systems(Startup, load_system.in_set(MudStartup::World))
      .add_systems(Update, replicate_parent)
      .add_systems(
        Last,
        final_save_system.run_if(on_event::<AppExit>().and_then(not(on_event::<LoadFailed>()))),
      )
      .add_systems(Last, save_system.run_if(not(on_event::<AppExit>())))
      .add_systems(PreUpdate, systems::asset_events)
      .debug_lifecycle::<Save>("Save")
      .debug_lifecycle::<Replicated>("Replicated")
      .debug_lifecycle::<Handle<SavedEntity>>("Handle<SavedEntity>")
      .observe(systems::saved_entity_added)
      .observe(save_added)
      .replicate::<Save>();
  }
}

fn save_added(
  trigger: Trigger<OnAdd, Save>,
  query: Query<&Save, Without<Handle<SavedEntity>>>,
  asset_server: Res<AssetServer>,
  mut cmd: Commands,
) {
  let entity = trigger.entity();
  let Ok(data) = query.get(entity) else {
    return;
  };

  let handle = asset_server.load::<SavedEntity>(data);

  cmd.entity(entity).insert(handle);
}

fn parent_replicated(
  trigger: Trigger<OnAdd, Parent>,
  query: Query<(), With<Replicated>>,
  mut cmd: Commands,
) {
  if query.get(trigger.entity()).is_ok() {
    cmd.entity(trigger.entity()).insert(ParentSync::default());
  }
}

fn replicated_parent(
  trigger: Trigger<OnAdd, Replicated>,
  query: Query<(), With<Parent>>,
  mut cmd: Commands,
) {
  if query.get(trigger.entity()).is_ok() {
    cmd.entity(trigger.entity()).insert(ParentSync::default());
  }
}

fn replicate_parent(
  query: Query<
    Entity,
    (
      Or<(
        (Added<Parent>, With<Replicated>),
        (Added<Replicated>, With<Parent>),
      )>,
      Without<ParentSync>,
    ),
  >,
  mut cmd: Commands,
) {
  for ent in query.iter() {
    cmd.entity(ent).insert(ParentSync::default());
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

    let static_scene = Scene::from_dynamic_scene(&scene, &registry).unwrap();
    let dynamic_scene = DynamicScene::from_scene(&static_scene);

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
    world.send_event(AppExit::Error(1.try_into().unwrap()));
  }
}

fn final_save_system(world: &World) {
  let registry = world.resource::<AppTypeRegistry>().clone();
  let mut scene = DynamicSceneBuilder::from_world(world)
    .deny_all()
    .deny_all_resources()
    .allow_resource::<UserDb>()
    .extract_resources()
    .build();
  bevy_replicon::scene::replicate_into(&mut scene, world);
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
    let mut scene = DynamicSceneBuilder::from_world(world)
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
        if extract_time + serialize_time + write_time > Duration::from_secs_f32(SAVE_INTERVAL) {
          warn!("save took longer than save interval");
        }
      })
      .detach();
  }
}
