mod db;
mod extract;
mod ser;

use std::{
  any::TypeId,
  iter,
};

use bevy::{
  app::AppExit,
  prelude::*,
  reflect::GetTypeRegistration,
  utils::HashSet,
};
use db::*;
use extract::EntityExtractor;

use crate::{
  coords::Cubic,
  core::CantonStartup,
  map::{
    Map,
    MapCoords,
    MapName,
    Tile,
  },
  tasks::TokioRuntime,
  util::HierEntity,
};

/// Marker component for entities that should be saved to the database.
/// Removing this has no effect in and of itself - to delete the entity from the
/// database, mark it with [Delete].
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Persist;

/// Marker component for entities that should be loaded on startup.
/// Usually for things like maps and their children.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct AutoLoad;

/// The entity ID in the database.
/// If an entity is marked [Persist], but does not have an ID, it will be saved
/// to the database as soon as possible, and this component added with the newly
/// allocated ID.
/// If an entity has this component, but not [Loaded], it will be loaded from
/// the database as soon as possible, and the [Loaded] component added.
#[derive(Component, Copy, Clone, Hash, Eq, PartialEq, Debug, Reflect)]
#[reflect(Component)]
pub struct DbEntity(pub Entity);

impl Default for DbEntity {
  fn default() -> Self {
    DbEntity(Entity::PLACEHOLDER)
  }
}

impl DbEntity {
  pub fn from_bits(bits: u64) -> Self {
    Self(Entity::from_bits(bits))
  }
  pub fn to_bits(self) -> u64 {
    self.0.to_bits()
  }
}

/// Marker component for entities slated for deletion.
/// These entities will be deleted from the database as soon as possible, and
/// this component subsequently removed, along with the [DbEntity] and [Persist]
/// components.
/// No effect if there is no DbEntity component.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Delete;

/// Marker struct for entities that need to be loaded from the database.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Load;

/// Marker struct for entities that should be saved and then despawned.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Unload;

/// Marker struct for entities that should be saved.
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Save;

#[derive(Resource, Default)]
pub struct PersistComponents {
  type_ids: HashSet<TypeId>,
}

impl PersistComponents {
  pub fn register<T>(&mut self)
  where
    T: Component,
  {
    let id = TypeId::of::<T>();
    if Self::is_hier(id) {
      return;
    }
    self.type_ids.insert(TypeId::of::<T>());
  }
  pub fn filter(&self) -> SceneFilter {
    SceneFilter::Allowlist(self.type_ids.clone())
  }
  fn is_hier(id: TypeId) -> bool {
    id == TypeId::of::<Children>()
  }
}

pub trait SaveExt {
  fn persist_component<T: Component + GetTypeRegistration>(&mut self) -> &mut Self;
}

impl SaveExt for App {
  fn persist_component<T: Component + GetTypeRegistration>(&mut self) -> &mut Self {
    self
      .register_type::<T>()
      .world
      .resource_mut::<PersistComponents>()
      .register::<T>();
    self
  }
}

#[allow(dead_code)]
#[allow(clippy::type_complexity)]
fn save_all(
  mut cmd: Commands,
  extractor: EntityExtractor,
  db: SaveDb,
  rt: Res<TokioRuntime>,
  query: Query<Entity, With<Persist>>,
  children: Query<&Children>,
) {
  if query.is_empty() {
    return;
  }
  let entities = extractor.extract_entities(
    query
      .iter()
      .flat_map(|e| iter::once(e).chain(children.iter_descendants(e))),
  );
  let result = try_res!(rt.block_on(db.to_owned().save_entities(entities)), error => {
    warn!(?error, "failed to save entities to db");
    return;
  });
  for (entity, db_entity) in result {
    cmd.entity(entity).insert(db_entity);
  }
}

fn track_hier(
  mut cmd: Commands,
  needs_delete: Query<
    Entity,
    (
      With<DbEntity>,
      Without<Parent>,
      Without<Persist>,
      Without<Delete>,
      Without<Load>,
    ),
  >,
  children: Query<&Children>,
) {
  for entity in needs_delete.iter() {
    debug!(?entity, "delete orphaned entity");
    cmd.entity(entity).insert(Delete);
    for child in children.iter_descendants(entity) {
      debug!(?child, "delete child of orphaned entity");
      cmd.entity(child).insert(Delete);
    }
  }
}

fn untrack(db: SaveDb, mut removed: RemovedComponents<DbEntity>) {
  for ent in removed.read() {
    db.remove_mapping(ent);
  }
}

fn unload(
  mut cmd: Commands,
  extractor: EntityExtractor,
  db: SaveDb,
  rt: Res<TokioRuntime>,
  query: Query<(HierEntity, Option<&Persist>, Option<&DbEntity>), With<Unload>>,
  children: Query<&Children>,
) {
  if query.is_empty() {
    return;
  }
  let entities = extractor.extract_entities(
    query
      .iter()
      .filter_map(|(e, p, d)| {
        if p.is_some() && d.is_some() {
          Some(e.entity)
        } else {
          None
        }
      })
      .flat_map(|e| iter::once(e).chain(children.iter_descendants(e))),
  );
  try_res!(rt.block_on(db.to_owned().save_entities(entities)), error => {
    warn!(?error, "failed to save entities to db");
    return;
  });
  for (entity, _, _) in query.iter() {
    entity.despawn(&mut cmd);
  }
}

fn save(
  mut cmd: Commands,
  extractor: EntityExtractor,
  db: SaveDb,
  rt: Res<TokioRuntime>,
  query: Query<Entity, Or<(With<Save>, (With<Persist>, Without<DbEntity>))>>,
  children: Query<&Children>,
) {
  if query.is_empty() {
    return;
  }
  let entities = extractor.extract_entities(
    query
      .iter()
      .flat_map(|e| iter::once(e).chain(children.iter_descendants(e))),
  );
  let result = try_res!(rt.block_on(db.to_owned().save_entities(entities)), error => {
    warn!(?error, "failed to save entities to db");
    return;
  });
  for (entity, db_entity) in result {
    cmd.entity(entity).insert(db_entity).remove::<Save>();
  }
}

fn autoload(mut cmd: Commands, db: SaveDb, rt: Res<TokioRuntime>) {
  debug!("autoloading entities");
  let entities = try_res!(rt.block_on(db.to_owned().autoload_entities()), error => {
    warn!(?error, "failed to fetch entities");
    return
  });
  if !entities.is_empty() {
    cmd.add(db.write_to_world(entities));
  } else {
    debug!("spawning empty-ish map");
    cmd
      .spawn((Persist, AutoLoad, Map, MapName("default".into())))
      .with_children(|cmd| {
        cmd.spawn((Tile, MapName("default".into()), MapCoords(Cubic(0, 0, 0))));
      });
  }
}

#[allow(clippy::type_complexity)]
fn load(
  mut cmd: Commands,
  db: SaveDb,
  rt: Res<TokioRuntime>,
  query: Query<(Entity, &DbEntity), With<Load>>,
) {
  if query.is_empty() {
    return;
  }
  let mut orig = vec![];
  let mut db_entities = vec![];
  for (entity, db_entity) in query.iter() {
    db_entities.push(*db_entity);
    orig.push(entity);
    db.add_mapping(*db_entity, entity);
  }
  let entities = try_res!(rt.block_on(db.to_owned().load_entities(db_entities, None)), error => {
    warn!(?error, "failed to fetch entities");
    return
  });
  cmd.add(db.write_to_world(entities));
}

#[allow(clippy::type_complexity)]
fn delete(
  mut cmd: Commands,
  db: SaveDb,
  rt: Res<TokioRuntime>,
  query: Query<HierEntity, With<Delete>>,
) {
  if query.is_empty() {
    return;
  }
  let entities = db.db_entities(query.iter().map(|e| e.entity));
  debug!(?entities, "attempting to delete entities");
  try_res!(rt.block_on(db.to_owned().delete_entities(entities)), error => {
    warn!(?error, "failed to delete entities");
    return;
  });
  for entity in query.iter() {
    cmd
      .entity(entity.entity)
      .remove::<(Persist, DbEntity, Delete)>();
  }
}

pub struct SaveStatePlugin;

impl Plugin for SaveStatePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(db::SharedDbEntityMap::default())
      .insert_resource(PersistComponents::default())
      .register_type::<DbEntity>()
      .register_type::<Load>()
      .register_type::<Unload>()
      .register_type::<Save>()
      .register_type::<Delete>()
      .persist_component::<Parent>()
      .persist_component::<AutoLoad>()
      .persist_component::<Persist>()
      .add_systems(Startup, autoload.in_set(CantonStartup::World))
      .add_systems(PostUpdate, (delete, load, unload, save))
      .add_systems(PostUpdate, untrack.after(unload).after(delete))
      .add_systems(PostUpdate, track_hier.before(delete).before(save))
      .add_systems(Last, save_all.run_if(on_event::<AppExit>()));
  }
}
