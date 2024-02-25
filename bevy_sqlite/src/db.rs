use std::{
  borrow::Cow,
  fmt::Write,
  sync::{
    Arc,
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
  },
};

use bevy::{
  ecs::{
    entity::{
      EntityHashMap,
      SceneEntityMapper,
    },
    system::SystemParam,
  },
  prelude::*,
  reflect::DynamicTupleStruct,
  scene::DynamicEntity,
  utils::{
    HashMap,
    HashSet,
  },
};
use futures::prelude::*;
use sqlx::{
  Acquire,
  Either,
  Pool,
  Sqlite,
  SqliteConnection,
  SqlitePool,
};

use super::{
  ser::*,
  DbEntity,
  Load,
};
use crate::BoxError;

/// The actual sqlite database connection.
/// Most functionality expects it to be added to the `World` as a `Resource`.
#[derive(Resource, Clone, Deref, DerefMut)]
pub struct Db(Pool<Sqlite>);

impl Db {
  pub fn connect_lazy(uri: &str) -> Result<Self, sqlx::Error> {
    Ok(Db(SqlitePool::connect_lazy(uri)?))
  }
}

#[derive(SystemParam)]
pub struct SaveDb<'w> {
  pub(crate) db: Res<'w, Db>,
  pub(crate) type_registry: Res<'w, AppTypeRegistry>,
  pub(crate) map: Res<'w, SharedDbEntityMap>,
}

#[derive(Clone)]
pub struct SaveDbOwned {
  pub(crate) db: Db,
  pub(crate) type_registry: AppTypeRegistry,
  pub(crate) map: SharedDbEntityMap,
}

impl From<&'_ World> for SaveDbOwned {
  fn from(value: &'_ World) -> Self {
    let db = SaveDbOwned {
      db: value.resource::<Db>().clone(),
      type_registry: value.resource::<AppTypeRegistry>().clone(),
      map: value.resource::<SharedDbEntityMap>().clone(),
    };
    db
  }
}

impl<'w> SaveDb<'w> {
  pub fn to_owned(&self) -> SaveDbOwned {
    SaveDbOwned {
      db: self.db.clone(),
      type_registry: self.type_registry.clone(),
      map: self.map.clone(),
    }
  }

  pub fn add_mapping(&self, db_entity: DbEntity, world_entity: Entity) {
    let mut map = self.map.write();
    map.add_db_mapping(db_entity, world_entity);
  }
  pub fn remove_mapping(&self, world_entity: Entity) {
    let mut map = self.map.write();
    map.remove_db_mapping(world_entity);
  }
  pub fn db_entities(&self, entities: impl Iterator<Item = Entity>) -> Vec<(Entity, DbEntity)> {
    let db = self.map.read();
    entities
      .filter_map(|e| db.db_entity(e).map(|d| (e, d)))
      .collect()
  }
}

impl SaveDbOwned {
  pub fn write_to_world(
    &self,
    mut ents: Vec<DynamicDbEntity>,
  ) -> impl FnOnce(&mut World) + 'static {
    let mappings = self.map.clone();
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

  pub async fn hydrate_entities(self) -> Result<impl FnOnce(&mut World), BoxError> {
    let mut conn = self.db.acquire().await?;
    let db_entities = sqlx::query!("select id from entity;")
      .fetch(&mut *conn)
      .map(|res| res.map(|r| DbEntity::from_index(r.id)))
      .try_collect::<Vec<_>>()
      .await?;

    Ok(move |world: &mut World| {
      let mut guard = self.map.write();
      let DbEntityMap {
        ref mut db_to_world,
        ref mut world_to_db,
      } = &mut *guard;

      SceneEntityMapper::world_scope(db_to_world, world, |_, mapper| {
        db_entities.into_iter().for_each(|db_ent| {
          world_to_db.insert(mapper.map_entity(db_ent.0), db_ent.0);
        })
      })
    })
  }

  pub async fn autoload_entities(self) -> Result<impl FnOnce(&mut World), BoxError> {
    let mut conn = self.db.acquire().await?;
    let db_entities = sqlx::query! {
      "SELECT ec.entity
      FROM entity e, component c, entity_component ec
      WHERE e.id = ec.entity
      AND c.id = ec.component
      AND c.name = 'bevy_sqlite::AutoLoad'"
    }
    .fetch(&mut *conn)
    .map(|r| r.map(|r| DbEntity::from_index(r.entity)))
    .try_collect::<Vec<_>>()
    .await?;

    let entities = self.load_entities(Some(&mut *conn), db_entities).await?;

    Ok(move |world: &mut World| {
      if !entities.is_empty() {
        let db = SaveDbOwned::from(&*world);
        debug!("writing {} entities to world", entities.len());
        db.write_to_world(entities)(world)
      } else {
        debug!("nothing to autoload!");
      }
    })
  }

  pub async fn load_entities(
    self,
    conn: Option<&mut SqliteConnection>,
    entities: Vec<DbEntity>,
  ) -> Result<Vec<DynamicDbEntity>, BoxError> {
    let mut acquired;
    let conn = if let Some(conn) = conn {
      conn
    } else {
      acquired = Some(self.db.acquire().await?);
      acquired.as_mut().unwrap()
    };

    let mut seen = HashSet::<DbEntity>::new();
    let mut to_load = vec![];

    for db_entity in entities {
      let db_id = db_entity.to_index();
      let mut results = sqlx::query! {
          "WITH RECURSIVE
            children(id) as (values(?) union select e.id from entity e, children d where e.parent = d.id)
          select id, parent from entity where entity.id in children",
          db_id,
        }
        .fetch(&mut *conn);
      while let Some(res) = results.try_next().await? {
        let ent = DbEntity::from_index(res.id);
        let parent = res.parent.map(DbEntity::from_index);

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
      let serialized_components = fetch_entity(conn, *entity).await?;
      let deserialized_components =
        deserialize_entity(&self.type_registry, &serialized_components)?;
      *components = deserialized_components;
      debug!(entity=?entity, components=?components, "fetched entity");
    }

    Ok(to_load)
  }
  pub async fn delete_entities(self, entities: Vec<(Entity, DbEntity)>) -> Result<(), BoxError> {
    if entities.is_empty() {
      return Ok(());
    }
    let mut query = "DELETE FROM entity WHERE id IN (".to_string();
    write!(&mut query, "{}", entities[0].1.to_index())?;
    for entity in &entities[1..] {
      write!(&mut query, ",{}", entity.1.to_index())?;
    }
    write!(&mut query, ")")?;
    let mut conn = self.db.acquire().await?;
    sqlx::query(&query).execute(&mut *conn).await?;
    let mut map = self.map.write();
    for entity in &entities {
      map.remove_db_mapping(entity.0);
    }
    Ok(())
  }
  pub async fn save_entities(
    self,
    entities: Vec<DynamicEntity>,
  ) -> Result<HashMap<Entity, DbEntity>, BoxError> {
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

#[derive(Resource, Default, Debug, Clone)]
pub struct SharedDbEntityMap {
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
  world_to_db: EntityHashMap<Entity>,
  db_to_world: EntityHashMap<Entity>,
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
    debug!(?world_entity, ?db_entity, "adding mapping");
    self.db_to_world.insert(db_entity.0, world_entity);
    self.world_to_db.insert(world_entity, db_entity.0);
  }

  fn remove_db_mapping(&mut self, world_entity: Entity) {
    if let Some(db_entity) = self.world_to_db.remove(&world_entity) {
      debug!(?world_entity, ?db_entity, "removing mapping");
      self.db_to_world.remove(&db_entity);
    }
  }

  // After entities are loaded from the db, we need to update the reverse
  // mappings.
  fn update(&mut self) {
    self.db_to_world.iter().for_each(|(f, t)| {
      self.world_to_db.insert(*t, *f);
    })
  }
}

pub struct DynamicDbEntity {
  entity: DbEntity,
  parent: Option<DbEntity>,
  components: Vec<Box<dyn Reflect>>,
}

async fn fetch_entity(
  conn: &mut SqliteConnection,
  entity: DbEntity,
) -> Result<SerializedComponents<'static>, sqlx::Error> {
  debug!(?entity, "fetching entity");
  let conn = conn.acquire().await?;

  let id = entity.to_index();

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
  .fetch(conn);

  let mut components = SerializedComponents::default();

  while let Some(a) = results.next().await.transpose()? {
    components.insert(Cow::Owned(a.name), a.data);
  }

  Ok(components)
}

async fn store_entity(
  conn: &mut SqliteConnection,
  entity: impl Into<Option<DbEntity>>,
  parent: impl Into<Option<DbEntity>>,
  components: &SerializedComponents<'_>,
) -> Result<DbEntity, sqlx::Error> {
  let parent_id = parent.into().map(|p| p.to_index());
  let entity_id = match entity.into() {
    Some(entity) => {
      let id = entity.to_index();
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
    None => DbEntity::from_index(
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
    let entity_bits = entity_id.to_index() as i64;
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

  Ok(entity_id)
}

#[allow(dead_code)]
async fn delete_entity(conn: &mut SqliteConnection, entity: DbEntity) -> Result<(), sqlx::Error> {
  let id = entity.to_index();

  sqlx::query!(
    r#"
      delete from entity where id = ?;
    "#,
    id,
  )
  .execute(conn)
  .await?;

  Ok(())
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

#[cfg(test)]
mod test {
  use super::*;

  #[derive(Default, Reflect, Component)]
  #[reflect(Component)]
  pub struct MyComponent(usize);

  fn test_entities<'a>(entities: &'a [(i64, &'a [(&'a str, &'a str)])]) -> SerializedEntities<'a> {
    entities
      .iter()
      .map(|(id, components)| {
        (
          DbEntity::from_index(*id),
          components
            .iter()
            .map(|(name, value)| (Cow::Borrowed(*name), value.to_string()))
            .collect(),
        )
      })
      .collect()
  }

  #[tokio::test]
  async fn test_store_fetch() -> Result<(), BoxError> {
    let db = SqlitePool::connect_lazy("sqlite::memory:")?;

    sqlx::query(include_str!("../../schema.sql"))
      .execute(&db)
      .await?;

    let mut conn = db.acquire().await?;

    let mut entities = test_entities(&[
      (2, &[("foo", "bar")]),
      (8, &[("foo", "baz"), ("spam", "eggs")]),
    ]);

    for (entity, components) in entities.iter_mut() {
      store_entity(&mut conn, *entity, None, components).await?;
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

    let mut conn = db.acquire().await?;
    let entity = fetch_entity(&mut conn, DbEntity::from_index(8)).await?;
    println!("{:#?}", entity);

    // panic!();
    Ok(())
  }
}
