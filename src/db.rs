use std::sync::{
    Arc,
    RwLock,
    RwLockReadGuard,
    RwLockWriteGuard,
};

use bevy::{
    ecs::{
        entity::EntityHashMap,
        reflect::ReflectMapEntities,
        system::{
            Command,
            SystemParam,
        },
    },
    prelude::*,
    reflect::{
        serde::{
            TypedReflectDeserializer,
            TypedReflectSerializer,
        },
        GetTypeRegistration,
        TypeInfo,
        TypeRegistration,
        TypeRegistry,
    },
    tasks::{
        block_on,
        futures_lite::StreamExt,
        IoTaskPool,
        Task,
    },
    utils::HashMap,
};
use bevy_async_util::nohup::{
    nohup,
    NoHup,
};
use serde::de::DeserializeSeed;
use sqlx::{
    Pool,
    Sqlite,
    SqliteConnection,
    SqlitePool,
};
use tracing::{
    debug,
    trace,
    warn,
};

use super::DbEntity;
use crate::{
    Creating,
    LoadedEntity,
    Loading,
    PersistComponents,
};

/// The actual sqlite database connection.
/// Most functionality expects it to be added to the `World` as a `Resource`.
#[derive(Resource, Clone, Deref, DerefMut)]
pub struct Db(pub Pool<Sqlite>);

impl Db {
    pub fn connect_lazy(uri: &str) -> Result<Self, sqlx::Error> {
        Ok(Db(SqlitePool::connect_lazy(uri)?))
    }
}

impl<'w> SaveDb<'w> {
    pub fn from_world(world: &'w World) -> Self {
        SaveDb {
            db: world.resource(),
            db_world: world.resource(),
            type_registry: world.resource(),
            persisted: world.resource(),
            map: world.resource(),
        }
    }
}

#[derive(Resource, Default, Clone)]
pub struct DbWorld(pub(crate) Arc<RwLock<World>>);

/// The collection of resources needed for most operations.
///
/// Publically, it only provides functionality to set up a new saved entity or
/// to delete a saved entity.
#[derive(Clone, Copy)]
pub struct SaveDb<'w> {
    pub db: &'w Db,
    pub db_world: &'w DbWorld,
    pub type_registry: &'w AppTypeRegistry,
    pub persisted: &'w PersistComponents,
    pub map: &'w SharedDbEntityMap,
}

impl<'w> From<SaveDb<'w>> for SaveDbOwned {
    fn from(value: SaveDb<'w>) -> Self {
        Self {
            db: value.db.clone(),
            db_world: value.db_world.clone(),
            type_registry: value.type_registry.clone(),
            persisted: value.persisted.clone(),
            map: value.map.clone(),
        }
    }
}

impl SaveDbOwned {
    pub fn borrowed(&self) -> SaveDb {
        SaveDb {
            db: &self.db,
            db_world: &self.db_world,
            type_registry: &self.type_registry,
            persisted: &self.persisted,
            map: &self.map,
        }
    }
}

/// An owned version of [SaveDb].
#[derive(Clone)]
pub struct SaveDbOwned {
    pub db: Db,
    pub db_world: DbWorld,
    pub type_registry: AppTypeRegistry,
    pub persisted: PersistComponents,
    pub map: SharedDbEntityMap,
}

unsafe impl<'w> SystemParam for SaveDb<'w> {
    type State = ();
    type Item<'world, 'state> = SaveDb<'world>;

    fn init_state(
        _world: &mut World,
        _system_meta: &mut bevy::ecs::system::SystemMeta,
    ) -> Self::State {
    }

    unsafe fn get_param<'world, 'state>(
        _state: &'state mut Self::State,
        _system_meta: &bevy::ecs::system::SystemMeta,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'world>,
        _change_tick: bevy::ecs::component::Tick,
    ) -> Self::Item<'world, 'state> {
        SaveDb::from_world(world.world())
    }
}

unsafe impl<'w> SystemParam for SaveDbOwned {
    type State = ();
    type Item<'world, 'state> = SaveDbOwned;

    fn init_state(
        _world: &mut World,
        _system_meta: &mut bevy::ecs::system::SystemMeta,
    ) -> Self::State {
    }

    unsafe fn get_param<'world, 'state>(
        _state: &'state mut Self::State,
        _system_meta: &bevy::ecs::system::SystemMeta,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'world>,
        _change_tick: bevy::ecs::component::Tick,
    ) -> Self::Item<'world, 'state> {
        SaveDb::from_world(world.world()).into()
    }
}

impl<'w> SaveDb<'w> {
    pub(crate) fn init(world: &mut World) {
        debug!("initializing mappings from db");
        let db = world.resource::<Db>().clone();
        let indices = block_on(async move {
            let records = sqlx::query!(r#"select id from entity"#,).fetch(&*db);

            records
                .map(|v| v.map(|r| r.id))
                .try_collect::<i64, _, Vec<i64>>()
                .await
        })
        .expect("fetch entity ids");
        let db_world = world.resource::<DbWorld>().clone();
        let mut db_world = db_world.write();
        let map = world.resource::<SharedDbEntityMap>().clone();
        let mut map_mut = map.write();
        let reserved = world.entities().reserve_entities(indices.len() as _);
        for (i, world_entity) in reserved.enumerate() {
            let db_entity = DbEntity::from_index(indices[i]);
            map_mut.add_db_mapping(db_entity, world_entity);
            db_world.get_or_spawn(db_entity.0).unwrap();
        }
        drop(map_mut);
        debug!(
            mappings = ?world.resource::<SharedDbEntityMap>().read(),
            "created {} entity mappings", indices.len()
        );
    }

    /// Delete an entity from the database.
    ///
    /// Detaches the task and returns a command that will remove the [DbEntity]
    /// from the world Entity.
    pub(crate) fn delete_entity(&self, entity: Entity) -> Option<Task<sqlx::Result<()>>> {
        let db = self.db.clone();
        let db_entity = self.remove_mapping(entity)?;
        self.db_world.write().despawn(db_entity.0);
        Some(IoTaskPool::get().spawn(async move {
            let id = db_entity.to_index();
            sqlx::query!(
                r#"
                            DELETE FROM entity WHERE id = ?
                        "#,
                id
            )
            .execute(&*db)
            .await?;
            sqlx::Result::<()>::Ok(())
        }))
    }

    fn serialize_component(
        &self,
        value: &dyn Reflect,
        registry: &TypeRegistry,
    ) -> ron::Result<String> {
        let serialized = ron::ser::to_string(&TypedReflectSerializer { value, registry })?;
        Ok(serialized)
    }

    fn save_serialized_component(
        &self,
        idx: i64,
        type_info: &'static TypeInfo,
        serialized: String,
    ) -> NoHup<Result<(), sqlx::Error>> {
        let db = self.db.clone();
        nohup(async move {
            let mut tx = db.begin().await?;
            save_serialized_component(&mut tx, idx, type_info, serialized).await?;
            tx.commit().await
        })
    }

    fn write_to_db_world<'d>(
        &self,
        db_world: &'d mut World,
        db_entity: DbEntity,
        value: &dyn Reflect,
        registration: &TypeRegistration,
        registry: &TypeRegistry,
    ) -> Option<&'d dyn Reflect> {
        let reflect_component = registration.data::<ReflectComponent>()?;
        reflect_component.apply_or_insert(
            &mut db_world.get_or_spawn(db_entity.0).unwrap(),
            value,
            &registry,
        );
        if let Some(map_entities) = registration.data::<ReflectMapEntities>() {
            map_entities.map_entities(db_world, &mut self.map.write().world_to_db, &[db_entity.0]);
        }

        let value = reflect_component.reflect(db_world.entity(db_entity.0))?;

        Some(value)
    }

    pub(crate) fn save_component<T: Component + GetTypeRegistration + Reflect>(
        &self,
        db_entity: DbEntity,
        component: &T,
    ) -> Option<NoHup<Result<(), sqlx::Error>>> {
        let value = component as &dyn Reflect;
        let type_info = T::get_type_registration().type_info();
        let registry = self.type_registry.read();
        let registration = registry.get(type_info.type_id())?;
        let mut db_world = self.db_world.write();
        let value =
            self.write_to_db_world(&mut db_world, db_entity, value, registration, &registry)?;
        let idx = db_entity.to_index();
        let serialized = match self.serialize_component(value, &registry) {
            Ok(s) => s,
            Err(error) => {
                warn!(
                    ?db_entity,
                    component = type_info.type_path(),
                    %error,
                    "failed to serialize component to save",
                );
                return None;
            }
        };
        self.save_serialized_component(idx, type_info, serialized)
            .into()
    }

    pub(crate) fn delete_component<T: Component + GetTypeRegistration>(
        &self,
        db_entity: DbEntity,
    ) -> Task<sqlx::Result<()>> {
        let idx = db_entity.to_index();
        let reg = T::get_type_registration();
        let type_path = reg.type_info().type_path();
        let db = self.db.clone();
        self.db_world.write().entity_mut(db_entity.0).remove::<T>();
        IoTaskPool::get().spawn(async move {
            let mut tx = db.begin().await?;
            // TODO cache component dbids
            let component_id = sqlx::query!(
                r#"
                    INSERT INTO component (name)
                    VALUES (?)
                    ON CONFLICT DO UPDATE SET name = excluded.name
                    RETURNING id
                "#,
                type_path
            )
            .fetch_one(&mut *tx)
            .await?
            .id;

            debug!(entity_id = idx, component_id, "deleting entity_component");

            let res = sqlx::query!(
                r#"
                    DELETE FROM entity_component
                    WHERE entity = ?
                    AND component = ?
                    RETURNING *
                "#,
                idx,
                component_id,
            )
            .fetch_one(&mut *tx)
            .await?;

            debug!(
                entity = res.entity,
                component = res.component,
                "deleted entry from db"
            );

            tx.commit().await?;

            Ok(())
        })
    }

    pub fn load_entity(&self, entity: Entity) -> Option<Loading> {
        self.map
            .read()
            .db_entity(entity)
            .and_then(|db_entity| self.load_db_entity(db_entity))
    }

    pub fn load_db_entity(&self, db_entity: DbEntity) -> Option<Loading> {
        let db = self.db.clone();
        let db_world = self.db_world.clone();
        let registry = self.type_registry.clone();
        let idx = db_entity.to_index();
        let task = IoTaskPool::get().spawn(async move {
            let mut components = HashMap::new();

            let results = sqlx::query!(
                r#"
                    SELECT c.name, ec.data
                    FROM component c, entity_component ec
                    WHERE ec.component = c.id
                    AND ec.entity = ?
                "#,
                idx,
            )
            .fetch_all(&*db)
            .await?;

            let registry = registry.read();

            let mut db_world = db_world.write();

            for record in results {
                let name = record.name;
                let data = record.data;

                let Some(registration) = registry.get_with_type_path(&name) else {
                    warn!(name, "component not registered");
                    continue;
                };

                let Some(component) = registration.data::<ReflectComponent>().cloned() else {
                    warn!(name, "Component trait not reflected");
                    continue;
                };

                let map_entities = registration.data::<ReflectMapEntities>().cloned();

                let seed = TypedReflectDeserializer::new(registration, &registry);

                let data = match ron::Deserializer::from_str(&data)
                    .map_err(ron::Error::from)
                    .and_then(|mut deserializer| seed.deserialize(&mut deserializer))
                {
                    Ok(de) => de,
                    Err(error) => {
                        warn!(%error, name, "error deserializing component");
                        continue;
                    }
                };

                component.apply_or_insert(
                    &mut db_world.get_or_spawn(db_entity.0).unwrap(),
                    &*data,
                    &registry,
                );

                components.insert(name, (component, map_entities, data));
            }

            Ok(LoadedEntity {
                db_entity,
                components,
            })
        });
        Some(Loading { task })
    }

    /// Save an entity to the database.
    ///
    /// This will also add the necessary components to populate its [DbEntity]
    /// once allocated.
    pub fn save_entity(&self, entity_ref: EntityRef) -> Creating {
        debug!(world_entity = ?entity_ref.id(), "save_entity");
        let world_entity = entity_ref.id();
        let db_entity = self.map.read().db_entity(world_entity);
        let db_entity = if db_entity.map(|e| e.to_index()).is_some() {
            let db_entity = db_entity.unwrap();
            db_entity
        } else {
            let mut db_world = self.db_world.write();
            let db_entity = DbEntity(db_world.spawn_empty().id());
            db_entity
        };
        self.map.write().add_db_mapping(db_entity, world_entity);

        let registry = self.type_registry.read();
        let mut db_world = self.db_world.write();
        let mut components = HashMap::<&'static str, (&'static TypeInfo, String)>::new();
        for registration in self
            .persisted
            .read()
            .iter()
            .filter_map(|id| registry.get(*id))
        {
            let type_info = registration.type_info();
            let type_path = type_info.type_path();
            let Some(reflect_component) = registration.data::<ReflectComponent>() else {
                warn!(type_path, "persisted component not reflected");
                continue;
            };
            let Some(value) = reflect_component.reflect(entity_ref) else {
                continue;
            };

            let Some(value) =
                self.write_to_db_world(&mut db_world, db_entity, value, registration, &registry)
            else {
                continue;
            };

            let serialized = match self.serialize_component(value, &registry) {
                Ok(s) => s,
                Err(error) => {
                    warn!(%error, type_path, "failed to serialize component");
                    continue;
                }
            };
            components.insert(type_path, (type_info, serialized));
        }

        trace!(map = ?&*self.map.read());

        let db = self.db.clone();
        let task = IoTaskPool::get()
            .spawn(async move {
                let mut tx = db.begin().await?;
                let idx = db_entity.to_index();
                sqlx::query!(
                    r#"
                    INSERT INTO entity (id) VALUES (?)
                    ON CONFLICT DO NOTHING
                "#,
                    idx,
                )
                .execute(&mut *tx)
                .await?;

                debug!(?db_entity, ?components, "saving components");
                for (info, data) in components.into_values() {
                    save_serialized_component(&mut tx, db_entity.to_index(), info, data).await?;
                }

                tx.commit().await?;
                Ok(db_entity)
            })
            .into();

        Creating { task }
    }
    pub(crate) fn remove_mapping(&self, world_entity: Entity) -> Option<DbEntity> {
        let mut map = self.map.write();
        map.remove_db_mapping(world_entity)
    }
}

async fn save_serialized_component(
    conn: &mut SqliteConnection,
    idx: i64,
    type_info: &'static TypeInfo,
    serialized: String,
) -> sqlx::Result<()> {
    let type_path = type_info.type_path();
    // TODO cache component dbids
    let component_id = sqlx::query!(
        r#"
            INSERT INTO component (name)
            VALUES (?)
            ON CONFLICT DO UPDATE SET name = excluded.name
            RETURNING id
        "#,
        type_path
    )
    .fetch_one(&mut *conn)
    .await?
    .id;

    sqlx::query!(
        r#"
            INSERT INTO entity_component (entity, component, data)
            VALUES (?, ?, ?)
            ON CONFLICT DO
            UPDATE SET data = excluded.data
        "#,
        idx,
        component_id,
        serialized,
    )
    .execute(&mut *conn)
    .await?;
    Ok(())
}

#[derive(Resource, Default, Debug, Clone)]
pub struct SharedDbEntityMap {
    inner: Arc<RwLock<DbEntityMap>>,
}

impl SharedDbEntityMap {
    pub fn read(&self) -> RwLockReadGuard<DbEntityMap> {
        self.inner.read().unwrap()
    }
    pub fn write(&self) -> RwLockWriteGuard<DbEntityMap> {
        self.inner.write().unwrap()
    }
}

#[allow(dead_code)]
#[derive(Resource, Default, Debug)]
pub struct DbEntityMap {
    pub(crate) world_to_db: EntityHashMap<Entity>,
    pub(crate) db_to_world: EntityHashMap<Entity>,
}

#[allow(dead_code)]
impl DbEntityMap {
    pub(crate) fn new_mapped_entities(&mut self) -> Vec<(DbEntity, Entity)> {
        let mut out = vec![];
        for (db_entity, world_entity) in self.db_to_world.iter().map(|(k, v)| (*k, *v)) {
            if self.world_to_db.insert(world_entity, db_entity).is_none() {
                out.push((DbEntity(db_entity), world_entity));
            }
        }
        out
    }

    pub fn db_entity(&self, world_entity: Entity) -> Option<DbEntity> {
        let res = self.world_to_db.get(&world_entity).copied().map(DbEntity);
        res
    }

    pub fn world_entity(&self, DbEntity(db_entity): DbEntity) -> Option<Entity> {
        let res = self.db_to_world.get(&db_entity).copied();
        trace!(world_entity=?res, ?db_entity, "map world_entity");
        res
    }

    pub fn add_db_mapping(
        &mut self,
        DbEntity(db_entity): DbEntity,
        world_entity: Entity,
    ) -> Option<Entity> {
        let prev = self.db_to_world.insert(db_entity, world_entity);
        self.world_to_db.insert(world_entity, db_entity);

        // Make sure we clear the world -> db mapping so that the new one won't
        // be erroneously removed by remove_db_mapping.
        if let Some(prev) = prev {
            self.world_to_db.remove(&prev);
        }

        prev
    }

    pub fn remove_db_mapping(&mut self, world_entity: Entity) -> Option<DbEntity> {
        if let Some(db_entity) = self.world_to_db.remove(&world_entity) {
            debug!(?db_entity, ?world_entity, "remove_db_mapping");
            self.db_to_world.remove(&db_entity);
            Some(DbEntity(db_entity))
        } else {
            None
        }
    }
}

impl DbWorld {
    pub fn write(&self) -> RwLockWriteGuard<World> {
        self.0.write().unwrap()
    }
    pub fn read(&self) -> RwLockReadGuard<World> {
        self.0.read().unwrap()
    }
}
