#![allow(clippy::type_complexity)]

#[cfg(test)]
mod test;

mod db;
mod ext;
mod util;

use std::any::TypeId;
use std::marker::PhantomData;
use std::pin::Pin;
use std::sync::Arc;
use std::sync::RwLock;
use std::sync::RwLockReadGuard;
use std::sync::RwLockWriteGuard;

use bevy::app::AppExit;
use bevy::ecs::reflect::ReflectMapEntities;
use bevy::ecs::system::EntityCommand;
use bevy::log::{
    debug,
    warn,
};
use bevy::tasks::{
    block_on,
    Task,
};
use bevy::utils::HashMap;
use bevy::{
    prelude::*,
    reflect::GetTypeRegistration,
    utils::HashSet,
};
use bevy_async_util::nohup::NoHup;
use bevy_async_util::{
    AppExt as _,
    BoxEntityCommand,
};
pub use db::*;
pub use ext::*;
use futures::Future;

/// The set of components that are persisted to the database.
#[derive(Resource, Default, Deref, DerefMut, Clone)]
pub struct PersistComponents {
    type_ids: Arc<RwLock<HashSet<TypeId>>>,
}

impl PersistComponents {
    pub fn write(&self) -> RwLockWriteGuard<HashSet<TypeId>> {
        self.type_ids.write().unwrap()
    }
    pub fn read(&self) -> RwLockReadGuard<HashSet<TypeId>> {
        self.type_ids.read().unwrap()
    }
    pub fn register<T>(&mut self)
    where
        T: Component,
    {
        self.write().insert(TypeId::of::<T>());
    }
}
/// The entity ID in the database.
#[derive(Component, Copy, Clone, Hash, Eq, PartialEq, Debug, Reflect)]
#[reflect(Component)]
pub struct DbEntity(pub Entity);

impl DbEntity {
    pub fn from_index(id: i64) -> Self {
        let bits = id as u64;
        let index = bits as u32;
        let generation = ((bits >> u32::BITS) + 1) as u32;
        Self(Entity::from_bits(
            ((generation as u64) << u32::BITS) | (index as u64),
        ))
    }
    pub fn to_index(self) -> i64 {
        let ent = self.0;
        let index = ent.index();
        let generation = ent.generation() - 1;
        (((generation as u64) << u32::BITS) | (index as u64)) as i64
    }
}

/// Marker for entities that should be loaded from the database.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Load;

/// Marker for entities that should be saved to the database.
///
/// This is an alternative to calling [SaveDb::save_entity].
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Persist;

/// Marker for entities that should be deleted from the database.
///
/// This is an alternative to calling [SaveDb::delete_entity].
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Delete;

/// Marker for entities that should be despawned but not necessarily deleted.
///
/// This is helpful for when you need to delete a persistent component and
/// despawn an entity in one frame.
#[derive(Component, Reflect)]
#[reflect(Component)]
pub struct Despawn;

/// Task indicating that the entity is being saved to the database for the first
/// time.
#[derive(Component, Deref, DerefMut)]
pub struct Creating {
    task: NoHup<sqlx::Result<DbEntity>>,
}

fn apply_created_output(res: sqlx::Result<DbEntity>) -> BoxEntityCommand {
    BoxEntityCommand::new(|entity, world: &mut World| {
        match res {
            Ok(db_entity) => {
                world.entity_mut(entity).insert(db_entity);
            }
            Err(error) => {
                warn!(?entity, %error,"error saving entity");
            }
        }
        world.entity_mut(entity).remove::<Creating>();
    })
}

impl Future for Creating {
    type Output = BoxEntityCommand;

    fn poll(
        mut self: std::pin::Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Pin::new(&mut self.as_mut().task)
            .poll(cx)
            .map(|res| apply_created_output(res))
    }
}

pub struct LoadedEntity {
    pub db_entity: DbEntity,
    pub components: HashMap<
        String,
        (
            ReflectComponent,
            Option<ReflectMapEntities>,
            Box<dyn Reflect>,
        ),
    >,
}

/// Task indicating that the entity is being loaded from the database.
#[derive(Component)]
pub struct Loading {
    task: Task<sqlx::Result<LoadedEntity>>,
}

impl Future for Loading {
    type Output = BoxEntityCommand;

    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Pin::new(&mut self.as_mut().task)
            .poll(cx)
            .map(|result| apply_loaded(result))
    }
}

/// Marker for components that have changed and are in need of saving to the
/// database.
#[derive(Component, Copy, Clone)]
pub struct Dirty<T>(PhantomData<fn() -> T>);

/// Task indicating that the component is in the process of being saved to the
/// database.
#[derive(Component)]
pub struct Saving<T> {
    task: NoHup<Result<(), sqlx::Error>>,
    _ph: PhantomData<fn() -> T>,
}

impl<T: Component + GetTypeRegistration> Future for Saving<T> {
    type Output = BoxEntityCommand;
    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Pin::new(&mut self.as_mut().task).poll(cx).map(move |res| {
            BoxEntityCommand::new(move |entity, world: &mut World| {
                world.entity_mut(entity).remove::<Saving<T>>();
                debug!(
                    ?entity,
                    component = T::get_type_registration().type_info().type_path(),
                    "finished saving component"
                );

                if let Err(error) = res {
                    warn!(%error, "error saving component");
                }
            })
        })
    }
}

/// Task indicating that the component is being deleted from the database.
#[derive(Component)]
pub struct Deleting<T> {
    task: Task<sqlx::Result<()>>,
    _ph: PhantomData<fn() -> T>,
}

impl<T: Component + GetTypeRegistration> Future for Deleting<T> {
    type Output = BoxEntityCommand;
    fn poll(
        mut self: Pin<&mut Self>,
        cx: &mut std::task::Context<'_>,
    ) -> std::task::Poll<Self::Output> {
        Pin::new(&mut self.as_mut().task).poll(cx).map(move |res| {
            BoxEntityCommand::new(move |entity, world: &mut World| {
                world.entity_mut(entity).remove::<Deleting<T>>();
                debug!(
                    ?entity,
                    component = T::get_type_registration().type_info().type_path(),
                    "finished deleting component"
                );

                if let Err(error) = res {
                    warn!(%error, "error deleting component");
                }
            })
        })
    }
}

fn cleanup(
    mut cmd: Commands,
    db: SaveDb,
    query: Query<
        (EntityRef, Has<DbEntity>, Has<Delete>, Has<Despawn>),
        Or<(With<Delete>, With<Despawn>)>,
    >,
) {
    for (entity, is_persisted, delete, despawn) in query.iter() {
        debug!(entity = ?entity.id(), despawn, delete, "found entity to dispose of");
        if despawn && !delete && is_persisted {
            let task = db.save_entity(entity);
            cmd.entity(entity.id()).insert(task);
        }
        if delete {
            if let Some(task) = db.delete_entity(entity.id()) {
                cmd.entity(entity.id()).remove::<DbEntity>();
                task.detach();
            };
        }
        if despawn {
            debug!(entity = ?entity.id(), "despawning entity");
            cmd.entity(entity.id()).despawn();
        } else {
            cmd.entity(entity.id()).remove::<(Despawn, Delete)>();
        }
    }
}

fn load(
    mut cmd: Commands,
    db: SaveDb,
    db_added: Query<(Entity, Ref<DbEntity>)>,
    with_load: Query<Entity, With<Load>>,
    mut just_saved: RemovedComponents<Creating>,
    mut just_loaded: RemovedComponents<Loading>,
) {
    let just_saved = HashSet::from_iter(just_saved.read());
    let just_loaded = HashSet::from_iter(just_loaded.read());
    let mut added_loading = HashSet::new();
    for (entity, db_entity) in db_added.iter() {
        if db_entity.is_added() && !just_saved.contains(&entity) && !just_loaded.contains(&entity) {
            let world_entity = db.map.read().world_entity(*db_entity);
            if let Some(world_entity) = world_entity {
                if world_entity != entity {
                    warn!("attempted to assign an already-mapped db entity to a different world entity");
                    cmd.entity(entity).remove::<DbEntity>();
                    continue;
                }
            } else {
                warn!(
                    ?db_entity,
                    ?world_entity,
                    "creating new db entity mapping - shoudn't need to do this during a load"
                );
                db.map.write().add_db_mapping(*db_entity, entity);
            }

            debug!(?world_entity, ?db_entity, "loading entity from db");
            let task = db.load_db_entity(*db_entity).unwrap();

            cmd.entity(entity).insert(task);
            added_loading.insert(entity);
        }
    }

    for entity in with_load.iter() {
        if !added_loading.contains(&entity) {
            if let Some(task) = db.load_entity(entity) {
                cmd.entity(entity).insert(task);
            } else {
                warn!(?entity, "cannot load world entity without db mapping");
            }
        }

        cmd.entity(entity).remove::<Load>();
    }
}

fn save(mut cmd: Commands, db: SaveDb, query: Query<EntityRef, With<Persist>>) {
    for entity in query.iter() {
        cmd.entity(entity.id())
            .remove::<Persist>()
            .insert(db.save_entity(entity));
    }
}

fn created_flush_exit(mut cmd: Commands, mut query: Query<(Entity, &mut Creating)>) {
    for (world_entity, mut task) in query.iter_mut() {
        let res = block_on(&mut task.task);

        match res {
            Ok(db_entity) => {
                cmd.entity(world_entity).insert(db_entity);
            }
            Err(error) => {
                warn!(?world_entity, %error,"error saving entity");
            }
        }
        cmd.entity(world_entity).remove::<Creating>();
    }
}

fn apply_loaded(res: sqlx::Result<LoadedEntity>) -> BoxEntityCommand {
    BoxEntityCommand::new(|entity, world: &mut World| {
        world.entity_mut(entity).remove::<Loading>();
        let loaded = match res {
            Ok(c) => c,
            Err(error) => {
                warn!(%error, ?entity, "error loading components for entity");
                return;
            }
        };

        let map = world.resource::<SharedDbEntityMap>().clone();
        let registry = world.resource::<AppTypeRegistry>().clone();
        let registry = registry.read();
        let mut map_mut = map.write();

        let mut entity_mut = world.entity_mut(entity);
        if !entity_mut.contains::<DbEntity>() {
            entity_mut.insert(loaded.db_entity);
        }

        debug!(?entity, "applying loaded components");
        for (_name, (component, map_entities, data)) in loaded.components {
            component.apply_or_insert(&mut world.entity_mut(entity), &*data, &registry);
            if let Some(map_entities) = map_entities {
                map_entities.map_entities(world, &mut map_mut.db_to_world, &[entity]);
            }
        }
    })
}

fn untrack(db: SaveDb, mut removed: RemovedComponents<DbEntity>) {
    for world_entity in removed.read() {
        db.remove_mapping(world_entity);
    }
}

fn exit_flush<T: Component + Reflect + GetTypeRegistration>(
    mut cmd: Commands,
    mut saving: Query<(Entity, &mut Saving<T>)>,
    mut deleting: Query<(Entity, &mut Deleting<T>)>,
) {
    for (entity, mut saving) in saving.iter_mut() {
        let res = block_on(&mut saving.task);

        let mut cmd = cmd.entity(entity);
        cmd.remove::<Saving<T>>();

        if let Err(error) = res {
            warn!(%error, "error saving component");
        }
    }
    for (entity, mut saving) in deleting.iter_mut() {
        let res = block_on(&mut saving.task);

        let mut cmd = cmd.entity(entity);
        cmd.remove::<Deleting<T>>();

        if let Err(error) = res {
            warn!(%error, "error deleting component");
        }
    }
}

fn mark_dirty<T: Component + GetTypeRegistration>(
    mut cmd: Commands,
    query: Query<(Entity, Ref<T>), (Or<(With<DbEntity>, With<Creating>)>, Without<Dirty<T>>)>,
    mut just_loaded: RemovedComponents<Loading>,
) {
    let just_loaded = just_loaded.read().collect::<HashSet<Entity>>();
    for (entity, component) in query.iter() {
        if (component.is_changed() || component.is_added()) && !just_loaded.contains(&entity) {
            debug!(
                ?entity,
                component = T::get_type_registration().type_info().type_path(),
                "marking dirty"
            );
            cmd.entity(entity).insert(Dirty::<T>(PhantomData));
        }
    }
}

fn save_dirty<T: Component + Reflect + GetTypeRegistration>(
    mut cmd: Commands,
    db: SaveDb,
    mut query: Query<
        (Entity, &DbEntity, &T),
        (
            With<Dirty<T>>,
            Without<Saving<T>>,
            Without<Deleting<T>>,
            Without<Creating>,
        ),
    >,
) {
    for (world_entity, db_entity, component) in query.iter_mut() {
        let Some(task) = db.save_component::<T>(*db_entity, component) else {
            warn!(
                ?world_entity,
                ?db_entity,
                component = T::get_type_registration().type_info().type_path(),
                "failed to create save task"
            );
            continue;
        };

        cmd.entity(world_entity)
            .remove::<Dirty<T>>()
            .insert(Saving::<T> {
                task,
                _ph: PhantomData,
            });
    }
}

fn delete_removed<T: Component + GetTypeRegistration>(
    mut cmd: Commands,
    db: SaveDb,
    live_entities: Query<&DbEntity>,
    mut removed: RemovedComponents<T>,
) {
    for world_entity in removed.read() {
        debug!(
            entity = ?world_entity,
            component = T::get_type_registration().type_info().type_path(),
            "component removed",
        );
        // Only delete the component from the db entity if the world entity is
        // still alive. If the entity despawns altogether, it's on the user to
        // call delete if they so wish.
        if let Ok(db_entity) = live_entities.get(world_entity) {
            debug!(
                entity = ?world_entity,
                component = T::get_type_registration().type_info().type_path(),
                "deleting removed component",
            );
            cmd.entity(world_entity).insert(Deleting::<T> {
                task: db.delete_component::<T>(*db_entity),
                _ph: PhantomData,
            });
        } else {
            debug!(entity = ?world_entity, "entity is no longer live, skipping delete");
        }
    }
}

#[derive(SystemSet, Debug, Default, PartialEq, Eq, Hash, Clone, Copy)]
pub struct DbSystem;

/// Plugin which sets up the necessary resources to persist entities to an
/// sqlite database.
pub struct SqlitePlugin;

impl Plugin for SqlitePlugin {
    fn build(&self, app: &mut App) {
        let track_untrack = (save, load, untrack);
        let systems = (track_untrack, cleanup).chain();
        app.insert_resource(db::SharedDbEntityMap::default())
            .insert_resource(PersistComponents::default())
            .insert_resource(DbWorld::default())
            .register_callback_in_set::<Creating>(PreUpdate, DbSystem)
            .register_callback_in_set::<Loading>(PreUpdate, DbSystem)
            .register_type::<Entity>()
            .register_type::<DbEntity>()
            .register_type::<Persist>()
            .register_type::<Despawn>()
            .register_type::<Delete>()
            .register_type::<Load>()
            .add_systems(
                Startup,
                (|world: &mut World| {
                    SaveDb::init(world);
                })
                .in_set(DbSystem),
            )
            .add_systems(PostUpdate, systems.in_set(DbSystem))
            .add_systems(
                Last,
                created_flush_exit
                    .run_if(on_event::<AppExit>())
                    .in_set(DbSystem),
            );
    }
}
