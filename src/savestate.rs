use std::{
  any::TypeId,
  borrow::{
    Borrow,
    Cow,
  },
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
  ecs::system::SystemParam,
  prelude::*,
  reflect::{
    serde::{
      TypedReflectDeserializer,
      TypedReflectSerializer,
    },
    DynamicTupleStruct,
    GetTypeRegistration,
    TypeRegistry,
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
  Either,
  Sqlite,
};

use crate::{
  db::Db,
  tasks::TokioRuntime,
};

/// Marker component for entities that should be saved to the database.
/// Removing this has no effect in and of itself - to delete the entity from the
/// database, mark it with [Delete].
#[derive(Component, Reflect, Default)]
#[reflect(Component)]
pub struct Persist;

/// The entity ID in the database.
/// If an entity is marked [Persist], but does not have an ID, it will be saved
/// to the database as soon as possible, and this component added with the newly
/// allocated ID.
/// If an entity has this component, but not [Loaded], it will be loaded from
/// the database as soon as possible, and the [Loaded] component added.
#[derive(Component, Copy, Clone, Hash, Eq, PartialEq, Debug)]
pub struct DbEntity(pub Entity);

/// Marker component for entities slated for deletion.
/// These entities will be deleted from the database as soon as possible, and
/// this component subsequently removed, along with the [DbEntity] and [Persist]
/// components.
/// No effect if there is no DbEntity component.
#[derive(Component)]
pub struct Delete;

/// Marker struct for entities that need to be loaded from the database.
#[derive(Component)]
pub struct Load;

/// Marker struct for entities that should be saved.
#[derive(Component)]
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
impl SaveDbOwned {
  pub async fn load_entities(
    self,
    entities: Vec<(Entity, DbEntity)>,
  ) -> anyhow::Result<DynamicScene> {
    let mut conn = self.db.acquire().await?;

    let mut seen = HashSet::<Entity>::new();
    let mut to_load = vec![];
    let mut orig = vec![];

    for (entity, db_entity) in entities {
      debug!(?entity, ?db_entity, "adding mapping");
      self.map.write().add_db_mapping(db_entity, entity);
      orig.push(entity);

      let db_id = db_entity.0.to_bits() as i64;
      let mut results = sqlx::query! {
          "WITH RECURSIVE
            children(id) as (values(?) union select e.id from entity e, children d where e.parent = d.id)
          select id from entity where entity.id in children",
          db_id,
        }
        .fetch(&mut *conn);
      while let Some(res) = results.try_next().await? {
        let ent = Entity::from_bits(res.id as u64);

        if !seen.contains(&ent) {
          debug!(?ent, "adding entity to load");
          to_load.push(ent);
          seen.insert(ent);
        }
      }
    }

    let mut scene = DynamicScene {
      entities: vec![],
      ..Default::default()
    };
    for entity in to_load {
      let components = fetch_entity(&mut conn, entity).await?;
      let entity = deserialize_entity(&self.type_registry.read(), entity, &components)?;
      debug!(entity=?entity.entity, components=?entity.components, "fetched entity");
      scene.entities.push(entity);
    }

    Ok(scene)
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
      debug!(entity = ?entity.entity, components = ?entity.components, "saving new entity");
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

      let entity = {
        let read_map = self.map.read();
        debug!(?parent);
        parent = parent.and_then(|p| read_map.db_entity(p));
        debug!(?parent);
        read_map.db_entity(entity.entity)
      };

      let components = serialize_components(&self.type_registry.read(), &components)?;

      let result = store_entity(&mut tx, entity, parent, &components).await?;

      let db_entity = DbEntity(result);

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

#[allow(clippy::type_complexity)]
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

#[allow(clippy::type_complexity)]
fn load(
  mut cmd: Commands,
  db: SaveDb,
  rt: Res<TokioRuntime>,
  query: Query<(Entity, &DbEntity), With<Load>>,
  _children: Query<&Children>,
) {
  if query.is_empty() {
    return;
  }
  let mut orig = vec![];
  let mut entities = vec![];
  for (entity, db_entity) in query.iter() {
    entities.push((entity, *db_entity));
    orig.push(entity);
  }
  let scene = try_res!(rt.block_on(db.to_owned().load_entities(entities)), error => {
    warn!(?error, "failed to load entities");
    return
  });
  let shared = db.map.clone();
  cmd.add(move |world: &mut World| {
    debug!("spawning fetched entities");
    let mut map = shared.write();
    if let Err(error) = scene.write_to_world(world, &mut map.db_to_world) {
      warn!(?error, "failed to load entities");
    }
    map.update();
    for entity in orig {
      world.entity_mut(entity).remove::<Load>();
    }
  });
}

#[allow(clippy::type_complexity)]
fn delete(
  mut cmd: Commands,
  db: SaveDb,
  rt: Res<TokioRuntime>,
  query: Query<(Entity, &DbEntity), (With<Delete>, Without<Load>)>,
) {
  if query.is_empty() {
    return;
  }
  try_res!(rt.block_on(db.to_owned().delete_entities(query.iter().map(|(e, d)| (e, *d)).collect())), error => {
    warn!(?error, "failed to delete entities");
    return;
  });
  for entity in query.iter() {
    cmd.entity(entity.0).remove::<(DbEntity, Persist, Delete)>();
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
  fn db_entity(&self, world_entity: impl Borrow<Entity>) -> Option<Entity> {
    self.world_to_db.get(world_entity.borrow()).copied()
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
      .persist_component::<Parent>()
      .persist_component::<Persist>()
      .add_systems(PostUpdate, (delete, load, save));
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
type SerializedEntities<'a> = HashMap<Entity, SerializedComponents<'a>>;

fn serialize_component(
  type_registry: &TypeRegistry,
  component: &dyn Reflect,
) -> Result<(Cow<'static, str>, String), ron::Error> {
  let name = component
    .get_represented_type_info()
    .map(|info| info.type_path())
    .expect("missing type info");
  Ok((
    Cow::Borrowed(name),
    serialize_ron(TypedReflectSerializer::new(component, type_registry))?,
  ))
}

fn serialize_components(
  type_registry: &TypeRegistry,
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
  type_registry: &TypeRegistry,
  entity: &DynamicEntity,
) -> Result<(Entity, SerializedComponents<'static>), ron::Error> {
  serialize_components(type_registry, &entity.components).map(|c| (entity.entity, c))
}

#[allow(dead_code)]
fn serialize_entities(
  type_registry: &TypeRegistry,
  entities: &[DynamicEntity],
) -> Result<SerializedEntities<'static>, ron::Error> {
  entities
    .iter()
    .map(|entity| {
      serialize_components(type_registry, &entity.components).map(|c| (entity.entity, c))
    })
    .collect()
}

#[allow(dead_code)]
fn deserialize_component(
  type_registry: &TypeRegistry,
  name: &str,
  value: &str,
) -> Result<Box<dyn Reflect>, ron::Error> {
  let registration =
    type_registry
      .get_with_type_path(name)
      .ok_or_else(|| ron::Error::NoSuchStructField {
        expected: &["a valid component"],
        found: name.to_string(),
        outer: None,
      })?;
  let deserializer = TypedReflectDeserializer::new(registration, type_registry);
  let mut seed = ron::Deserializer::from_str(value)?;
  deserializer.deserialize(&mut seed)
}

#[allow(dead_code)]
fn deserialize_entity(
  type_registry: &TypeRegistry,
  entity: Entity,
  components: &SerializedComponents,
) -> Result<DynamicEntity, ron::Error> {
  let components = components
    .iter()
    .map(|(name, serialized)| deserialize_component(type_registry, name, serialized))
    .collect::<Result<_, ron::Error>>()?;
  Ok(DynamicEntity { entity, components })
}

#[allow(dead_code)]
fn deserialize_entities(
  type_registry: &TypeRegistry,
  entities: &SerializedEntities,
) -> Result<Vec<DynamicEntity>, ron::Error> {
  entities
    .iter()
    .map(|(entity, components)| deserialize_entity(type_registry, *entity, components))
    .collect::<Result<_, ron::Error>>()
}

async fn store_entity<'a, 'b, D>(
  db: D,
  entity: impl Into<Option<Entity>>,
  parent: impl Into<Option<Entity>>,
  components: &SerializedComponents<'_>,
) -> Result<Entity, sqlx::Error>
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
          delete from entity where id = ?;
          insert into entity (id, parent) values (?, ?);
        "#,
        id,
        id,
        parent_id,
      )
      .execute(&mut *conn)
      .await?;
      id
    }
    None => {
      sqlx::query!(
        "INSERT INTO entity (parent) values (?) returning id",
        parent_id
      )
      .fetch_one(&mut *conn)
      .await?
      .id
    }
  };

  for (name, value) in components.iter() {
    sqlx::query!(
      r#"
        insert or ignore into component (name) values (?);
        insert into entity_component (entity, component, data)
        values (?, (select id from component where name = ?), ?);
      "#,
      name,
      entity_id,
      name,
      value,
    )
    .execute(&mut *conn)
    .await?;
  }

  conn.commit().await?;
  Ok(Entity::from_bits(entity_id as u64))
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
  entity: Entity,
) -> Result<SerializedComponents<'static>, sqlx::Error>
where
  D: sqlx::Acquire<'a, Database = Sqlite>,
{
  debug!(?entity, "fetching entity");
  let mut conn = db.acquire().await?;

  let id = entity.to_bits() as i64;

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
          Entity::from_bits(*id),
          components
            .iter()
            .map(|(name, value)| (Cow::Borrowed(*name), value.to_string()))
            .collect(),
        )
      })
      .collect()
  }

  #[test]
  fn test_deserialize() {
    let registry = AppTypeRegistry::default();
    let mut reg_w = registry.write();
    reg_w.register::<MyComponent>();
    drop(reg_w);

    let filter = SceneFilter::Allowlist(registry.read().iter().map(|r| r.type_id()).collect());

    let entities = test_entities(&[
      (5, &[("canton::savestate::test::MyComponent", "(23)")]),
      (32, &[("canton::savestate::test::MyComponent", "(16)")]),
    ]);

    let entities = deserialize_entities(&registry.read(), &entities).expect("deserialize entities");

    let mut world = World::new();
    world.insert_resource(registry.clone());

    let mut mappings = HashMap::default();

    DynamicScene {
      entities,
      ..Default::default()
    }
    .write_to_world(&mut world, &mut mappings)
    .expect("write to world");

    let new_entities = world.query::<Entity>().iter(&world).collect::<Vec<_>>();

    println!(
      "entities: {:#?}",
      serialize_entities(
        &registry.read(),
        &extract_entities(&world, new_entities.into_iter(), filter)
      )
    );
    println!("mappings: {:#?}", mappings);

    // panic!();
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

    let entity = fetch_entity(&db, Entity::from_bits(8)).await?;
    println!("{:#?}", entity);

    // panic!();
    Ok(())
  }
}
