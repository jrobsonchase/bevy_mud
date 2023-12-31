use std::{
  any::TypeId,
  borrow::Cow,
  fmt::Write,
  iter,
  sync::{
    Arc,
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
  },
};

use bevy::{
  app::AppExit,
  ecs::system::SystemParam,
  prelude::*,
  reflect::{
    serde::{
      TypedReflectDeserializer,
      TypedReflectSerializer,
    },
    DynamicTupleStruct,
    GetTypeRegistration,
    TypeRegistryArc,
  },
  scene::DynamicEntity,
  utils::{
    HashMap,
    HashSet,
  },
};
use futures::prelude::*;
use serde::{
  de::DeserializeSeed,
  Serialize,
};
use sqlx::{
  pool::PoolConnection,
  Either,
  Sqlite,
};

use crate::{
  coords::Cubic,
  core::CantonStartup,
  db::Db,
  map::{
    Map,
    MapCoords,
    MapName,
    Tile,
  },
  tasks::TokioRuntime,
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

#[derive(SystemParam)]
pub struct EntityExtractor<'w> {
  persisted: Res<'w, PersistComponents>,
  world: &'w World,
}

impl<'w> EntityExtractor<'w> {
  fn extract_entities(&self, entities: impl Iterator<Item = Entity>) -> Vec<DynamicEntity> {
    extract_entities(self.world, entities, self.persisted.filter())
  }
}

#[derive(SystemParam)]
pub struct SaveDb<'w> {
  db: Res<'w, Db>,
  type_registry: Res<'w, AppTypeRegistry>,
  map: Res<'w, SharedDbEntityMap>,
}

#[derive(Clone)]
pub struct SaveDbOwned {
  db: Db,
  type_registry: AppTypeRegistry,
  map: SharedDbEntityMap,
}

impl<'w> SaveDb<'w> {
  fn to_owned(&self) -> SaveDbOwned {
    SaveDbOwned {
      db: self.db.clone(),
      type_registry: self.type_registry.clone(),
      map: self.map.clone(),
    }
  }
}
fn find_parent(c: Box<dyn Reflect>) -> Either<Entity, Box<dyn Reflect>> {
  if c.represents::<Parent>() {
    Either::Left(
      c.downcast::<DynamicTupleStruct>()
        .unwrap()
        .field(0)
        .unwrap()
        .downcast_ref::<Entity>()
        .copied()
        .unwrap(),
    )
  } else {
    Either::Right(c)
  }
}

pub struct DynamicDbEntity {
  entity: DbEntity,
  parent: Option<DbEntity>,
  components: Vec<Box<dyn Reflect>>,
}

fn write_to_world(
  mappings: SharedDbEntityMap,
  mut ents: Vec<DynamicDbEntity>,
) -> impl FnOnce(&mut World) {
  let scene = DynamicScene {
    entities: ents
      .iter_mut()
      .map(|de| DynamicEntity {
        entity: de.entity.0,
        components: std::mem::take(&mut de.components),
      })
      .collect(),
    ..Default::default()
  };
  debug!("spawning fetched entities");
  move |world: &mut World| {
    {
      let mut map = mappings.write();
      if let Err(error) = scene.write_to_world(world, &mut map.db_to_world) {
        warn!(?error, "failed to spawn entities");
      }
      map.update();
    }
    let map = mappings.read();
    for entity in ents {
      let db_entity = entity.entity;
      let world_entity = map.world_entity(db_entity).unwrap();
      debug!(?world_entity, ?db_entity, "loaded entity from database");
      let mut entity_cmds = world.entity_mut(world_entity);
      entity_cmds.remove::<Load>().insert(db_entity);
      if let Some(db_parent) = entity.parent {
        let world_parent = map.world_entity(db_parent).unwrap();
        debug!(
          ?world_entity,
          ?world_parent,
          ?db_entity,
          ?db_parent,
          "setting loaded entity parent"
        );
        entity_cmds.set_parent(world_parent);
      }
    }
  }
}

impl SaveDbOwned {
  pub async fn autoload_entities(self) -> anyhow::Result<Vec<DynamicDbEntity>> {
    let mut conn = self.db.acquire().await?;
    let results = sqlx::query! {
      "SELECT ec.entity
      FROM entity e, component c, entity_component ec
      WHERE e.id = ec.entity
      AND c.id = ec.component
      AND c.name = 'canton::savestate::AutoLoad'"
    }
    .fetch(&mut *conn)
    .map(|r| r.map(|r| DbEntity(Entity::from_bits(r.entity as _))));

    self
      .load_entities(results.try_collect().await?, Some(conn))
      .await
  }
  pub async fn load_entities(
    self,
    entities: Vec<DbEntity>,
    conn: Option<PoolConnection<Sqlite>>,
  ) -> anyhow::Result<Vec<DynamicDbEntity>> {
    let mut conn = if let Some(conn) = conn {
      conn
    } else {
      self.db.acquire().await?
    };

    let mut seen = HashSet::<DbEntity>::new();
    let mut to_load = vec![];

    for db_entity in entities {
      let db_id = db_entity.0.to_bits() as i64;
      let mut results = sqlx::query! {
          "WITH RECURSIVE
            children(id) as (values(?) union select e.id from entity e, children d where e.parent = d.id)
          select id, parent from entity where entity.id in children",
          db_id,
        }
        .fetch(&mut *conn);
      while let Some(res) = results.try_next().await? {
        let ent = DbEntity::from_bits(res.id as u64);
        let parent = res.parent.map(|p| DbEntity::from_bits(p as u64));

        if !seen.contains(&ent) {
          debug!(?ent, "adding entity to load");
          to_load.push(DynamicDbEntity {
            entity: ent,
            parent,
            components: vec![],
          });
          seen.insert(ent);
        }
      }
    }

    for DynamicDbEntity {
      entity, components, ..
    } in &mut to_load
    {
      let serialized_components = fetch_entity(&mut conn, *entity).await?;
      let deserialized_components =
        deserialize_entity(&self.type_registry, &serialized_components)?;
      *components = deserialized_components;
      debug!(entity=?entity, components=?components, "fetched entity");
    }

    Ok(to_load)
  }
  pub async fn delete_entities(self, entities: Vec<(Entity, DbEntity)>) -> anyhow::Result<()> {
    if entities.is_empty() {
      return Ok(());
    }
    let mut query = "DELETE FROM entity WHERE id IN (".to_string();
    write!(&mut query, "{}", entities[0].1 .0.to_bits() as i64)?;
    for entity in &entities[1..] {
      write!(&mut query, ",{}", entity.1 .0.to_bits() as i64)?;
    }
    write!(&mut query, ")")?;
    let mut conn = self.db.acquire().await?;
    sqlx::query(&query).execute(&mut *conn).await?;
    Ok(())
  }
  pub async fn save_entities(
    self,
    entities: Vec<DynamicEntity>,
  ) -> anyhow::Result<HashMap<Entity, DbEntity>> {
    let mut output = HashMap::new();
    let mut tx = self.db.begin().await?;
    for entity in entities {
      let id = entity.entity;

      let mut parent = None;
      let components = entity
        .components
        .into_iter()
        .filter_map(|c| match find_parent(c) {
          Either::Left(p) => {
            parent = Some(p);
            None
          }
          Either::Right(c) => Some(c),
        })
        .collect::<Vec<_>>();

      let (db_entity, db_parent) = {
        let read_map = self.map.read();
        let parent = parent.and_then(|p| read_map.db_entity(p));
        let entity = read_map.db_entity(entity.entity);
        (entity, parent)
      };

      let components = serialize_components(&self.type_registry, &components)?;

      debug!(
        entity = ?entity.entity,
        ?db_entity,
        ?db_parent,
        ?components,
        "saving entity",
      );

      let db_entity = store_entity(&mut tx, db_entity, db_parent, &components).await?;

      self.map.write().add_db_mapping(db_entity, id);
      output.insert(id, db_entity);
    }
    tx.commit().await?;
    Ok(output)
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

fn unload(
  mut cmd: Commands,
  extractor: EntityExtractor,
  db: SaveDb,
  rt: Res<TokioRuntime>,
  query: Query<Entity, (With<Persist>, With<DbEntity>, With<Unload>)>,
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
  for (entity, _) in result {
    cmd.entity(entity).despawn();
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
    let shared = db.map.clone();
    cmd.add(write_to_world(shared, entities));
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
    debug!(?entity, ?db_entity, "adding mapping");
    db.map.write().add_db_mapping(*db_entity, entity);
  }
  let entities = try_res!(rt.block_on(db.to_owned().load_entities(db_entities, None)), error => {
    warn!(?error, "failed to fetch entities");
    return
  });
  let shared = db.map.clone();
  cmd.add(write_to_world(shared, entities));
}

#[allow(clippy::type_complexity)]
fn delete(
  mut cmd: Commands,
  db: SaveDb,
  rt: Res<TokioRuntime>,
  query: Query<Entity, With<Delete>>,
) {
  if query.is_empty() {
    return;
  }
  let map = db.map.read();
  let entities = query
    .iter()
    .filter_map(|e| map.db_entity(e).map(|d| (e, d)))
    .collect::<Vec<_>>();
  drop(map);
  try_res!(rt.block_on(db.to_owned().delete_entities(entities)), error => {
    warn!(?error, "failed to delete entities");
    return;
  });
  for entity in query.iter() {
    cmd.entity(entity).remove::<Persist>();
  }
}

#[derive(Resource, Default, Debug, Clone)]
struct SharedDbEntityMap {
  inner: Arc<RwLock<DbEntityMap>>,
}

impl SharedDbEntityMap {
  fn read(&self) -> RwLockReadGuard<DbEntityMap> {
    self.inner.read().unwrap()
  }
  fn write(&self) -> RwLockWriteGuard<DbEntityMap> {
    self.inner.write().unwrap()
  }
}

#[allow(dead_code)]
#[derive(Resource, Default, Debug)]
struct DbEntityMap {
  world_to_db: HashMap<Entity, Entity>,
  db_to_world: HashMap<Entity, Entity>,
}

#[allow(dead_code)]
impl DbEntityMap {
  fn db_entity(&self, world_entity: Entity) -> Option<DbEntity> {
    self.world_to_db.get(&world_entity).copied().map(DbEntity)
  }

  fn world_entity(&self, db_entity: DbEntity) -> Option<Entity> {
    self.db_to_world.get(&db_entity.0).copied()
  }

  fn add_db_mapping(&mut self, db_entity: DbEntity, world_entity: Entity) {
    self.db_to_world.insert(db_entity.0, world_entity);
    self.world_to_db.insert(world_entity, db_entity.0);
  }

  // After entities are loaded from the db, we need to update the reverse
  // mappings.
  fn update(&mut self) {
    self.db_to_world.iter().for_each(|(f, t)| {
      self.world_to_db.insert(*t, *f);
    })
  }
}

pub struct SaveStatePlugin;

impl Plugin for SaveStatePlugin {
  fn build(&self, app: &mut App) {
    app
      .insert_resource(SharedDbEntityMap::default())
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
      .add_systems(Last, save_all.run_if(on_event::<AppExit>()));
  }
}

pub fn serialize_ron<S>(serialize: S) -> Result<String, ron::Error>
where
  S: Serialize,
{
  ron::ser::to_string(&serialize)
}

#[allow(dead_code)]
fn extract_entities(
  world: &World,
  entities: impl Iterator<Item = Entity>,
  filter: SceneFilter,
) -> Vec<DynamicEntity> {
  DynamicSceneBuilder::from_world(world)
    .with_filter(filter)
    .deny_all_resources()
    .extract_entities(entities)
    .remove_empty_entities()
    .build()
    .entities
}

#[allow(dead_code)]
// Component name -> serialized component
type SerializedComponents<'a> = HashMap<Cow<'a, str>, String>;
#[allow(dead_code)]
// Entity -> Components
type SerializedEntities<'a> = HashMap<DbEntity, SerializedComponents<'a>>;

fn serialize_component(
  type_registry: &TypeRegistryArc,
  component: &dyn Reflect,
) -> Result<(Cow<'static, str>, String), ron::Error> {
  let name = component
    .get_represented_type_info()
    .map(|i| Cow::Borrowed(i.type_path()))
    .unwrap_or_else(|| Cow::Owned(component.reflect_type_path().to_string()));
  Ok((
    name,
    serialize_ron(TypedReflectSerializer::new(
      component,
      &type_registry.read(),
    ))?,
  ))
}

fn serialize_components(
  type_registry: &TypeRegistryArc,
  components: &[Box<dyn Reflect>],
) -> Result<SerializedComponents<'static>, ron::Error> {
  components
    .iter()
    .map(AsRef::as_ref)
    .map(|c| serialize_component(type_registry, c))
    .collect()
}

#[allow(dead_code)]
fn serialize_entity(
  type_registry: &TypeRegistryArc,
  entity: &DynamicEntity,
) -> Result<(Entity, SerializedComponents<'static>), ron::Error> {
  serialize_components(type_registry, &entity.components).map(|c| (entity.entity, c))
}

#[allow(dead_code)]
fn serialize_entities(
  type_registry: &TypeRegistryArc,
  entities: &[DynamicEntity],
) -> Result<SerializedEntities<'static>, ron::Error> {
  entities
    .iter()
    .map(|entity| {
      serialize_components(type_registry, &entity.components).map(|c| (DbEntity(entity.entity), c))
    })
    .collect()
}

#[allow(dead_code)]
fn deserialize_component(
  type_registry: &TypeRegistryArc,
  name: &str,
  value: &str,
) -> Result<Box<dyn Reflect>, ron::Error> {
  let type_registry = type_registry.read();
  let registration =
    type_registry
      .get_with_type_path(name)
      .ok_or_else(|| ron::Error::NoSuchStructField {
        expected: &["a valid component"],
        found: name.to_string(),
        outer: None,
      })?;
  let deserializer = TypedReflectDeserializer::new(registration, &type_registry);
  let mut seed = ron::Deserializer::from_str(value)?;
  deserializer.deserialize(&mut seed)
}

#[allow(dead_code)]
fn deserialize_entity(
  type_registry: &TypeRegistryArc,
  components: &SerializedComponents,
) -> Result<Vec<Box<dyn Reflect>>, ron::Error> {
  let components = components
    .iter()
    .map(|(name, serialized)| deserialize_component(type_registry, name, serialized))
    .collect::<Result<_, ron::Error>>()?;
  Ok(components)
}

async fn store_entity<'a, 'b, D>(
  db: D,
  entity: impl Into<Option<DbEntity>>,
  parent: impl Into<Option<DbEntity>>,
  components: &SerializedComponents<'_>,
) -> Result<DbEntity, sqlx::Error>
where
  D: sqlx::Acquire<'a, Database = Sqlite> + 'b,
{
  let mut conn = db.begin().await?;
  let parent_id = parent.into().map(|p| p.to_bits() as i64);
  let entity_id = match entity.into() {
    Some(entity) => {
      let id = entity.to_bits() as i64;
      // Make sure the entity exists and that the existing state is cleared.
      sqlx::query!(
        r#"
          delete from entity_component where entity = ?;
          insert into entity (id, parent) values (?, ?)
          on conflict do update set parent = excluded.parent;
        "#,
        id,
        id,
        parent_id,
      )
      .execute(&mut *conn)
      .await?;
      entity
    }
    None => DbEntity::from_bits(
      sqlx::query!(
        "INSERT INTO entity (parent) values (?) returning id",
        parent_id,
      )
      .fetch_one(&mut *conn)
      .await?
      .id as _,
    ),
  };

  for (name, value) in components.iter() {
    let entity_bits = entity_id.to_bits() as i64;
    sqlx::query!(
      r#"
        insert or ignore into component (name) values (?);
        insert into entity_component (entity, component, data)
        values (?, (select id from component where name = ?), ?);
      "#,
      name,
      entity_bits,
      name,
      value,
    )
    .execute(&mut *conn)
    .await?;
  }

  conn.commit().await?;
  Ok(entity_id)
}

#[allow(dead_code)]
async fn delete_entity<D>(db: D, entity: Entity) -> Result<(), sqlx::Error>
where
  D: for<'a> sqlx::Acquire<'a, Database = Sqlite>,
{
  let mut tx = db.begin().await?;

  let id = entity.to_bits() as i64;

  sqlx::query!(
    r#"
      delete from entity where id = ?;
    "#,
    id,
  )
  .execute(&mut *tx)
  .await?;

  tx.commit().await?;

  Ok(())
}

async fn fetch_entity<'a, D>(
  db: D,
  entity: DbEntity,
) -> Result<SerializedComponents<'static>, sqlx::Error>
where
  D: sqlx::Acquire<'a, Database = Sqlite>,
{
  debug!(?entity, "fetching entity");
  let mut conn = db.acquire().await?;

  let id = entity.0.to_bits() as i64;

  let mut results = sqlx::query!(
    r#"
      select c.name, ec.data
      from entity_component ec
      inner join component c
      on ec.component = c.id
      where ec.entity = ?
    "#,
    id,
  )
  .fetch(&mut *conn);

  let mut components = SerializedComponents::default();

  while let Some(a) = results.next().await.transpose()? {
    components.insert(Cow::Owned(a.name), a.data);
  }

  Ok(components)
}

#[cfg(test)]
mod test {

  use sqlx::SqlitePool;

  use super::*;

  #[derive(Default, Reflect, Component)]
  #[reflect(Component)]
  pub struct MyComponent(usize);

  fn test_entities<'a>(entities: &'a [(u64, &'a [(&'a str, &'a str)])]) -> SerializedEntities<'a> {
    entities
      .iter()
      .map(|(id, components)| {
        (
          DbEntity::from_bits(*id),
          components
            .iter()
            .map(|(name, value)| (Cow::Borrowed(*name), value.to_string()))
            .collect(),
        )
      })
      .collect()
  }

  #[tokio::test]
  async fn test_store_fetch() -> anyhow::Result<()> {
    let db = SqlitePool::connect_lazy("sqlite::memory:")?;

    sqlx::query(include_str!("../schema.sql"))
      .execute(&db)
      .await?;

    let mut entities = test_entities(&[
      (2, &[("foo", "bar")]),
      (8, &[("foo", "baz"), ("spam", "eggs")]),
    ]);

    for (entity, components) in entities.iter_mut() {
      store_entity(&db, *entity, None, components).await?;
    }

    println!("entity:");
    let mut results = sqlx::query!("select * from entity",).fetch(&db);
    while let Some(row) = results.next().await.transpose()? {
      println!("\t{:?}", row);
    }
    println!("component:");
    let mut results = sqlx::query!("select * from component",).fetch(&db);
    while let Some(row) = results.next().await.transpose()? {
      println!("\t{:?}", row);
    }
    println!("entity_component:");
    let mut results = sqlx::query!("select * from entity_component",).fetch(&db);
    while let Some(row) = results.next().await.transpose()? {
      println!("\t{:?}", row);
    }

    let entity = fetch_entity(&db, DbEntity::from_bits(8)).await?;
    println!("{:#?}", entity);

    // panic!();
    Ok(())
  }
}
