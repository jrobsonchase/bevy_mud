use bevy::{
    ecs::system::EntityCommands,
    prelude::*,
};

use crate::DbEntity;
use crate::*;

pub trait WorldExt {
    fn load(&mut self, entity: Entity);
    fn db_entity(&mut self, db_entity: DbEntity) -> EntityWorldMut;
}

impl WorldExt for World {
    fn load(&mut self, entity: Entity) {
        let Some(db_entity) = self
            .resource::<SharedDbEntityMap>()
            .read()
            .db_entity(entity)
        else {
            warn!(?entity, "no db mapping for entity load");
            return;
        };

        self.db_entity(db_entity);
    }

    fn db_entity(&mut self, db_entity: DbEntity) -> EntityWorldMut {
        let Some(world_entity) = self
            .resource::<SharedDbEntityMap>()
            .read()
            .world_entity(db_entity)
        else {
            warn!(
                ?db_entity,
                "missing mapping for db_entity, creating a new one"
            );
            self.spawn_empty();
            let entity = self.spawn(db_entity).id();
            self.resource::<SharedDbEntityMap>()
                .write()
                .add_db_mapping(db_entity, entity);
            return self.entity_mut(entity);
        };

        let entity = if let Some(entity) = self.get_or_spawn(world_entity).map(|mut em| {
            if !em.contains::<DbEntity>() {
                em.insert(db_entity)
            } else {
                &mut em
            }
            .id()
        }) {
            entity
        } else {
            let entity = self.spawn(db_entity).id();
            self.resource::<SharedDbEntityMap>()
                .write()
                .add_db_mapping(db_entity, entity);
            entity
        };

        self.entity_mut(entity)
    }
}

pub struct DbEntityCommands<'a> {
    entity: DbEntity,
    commands: Commands<'a, 'a>,
}

impl<'a> DbEntityCommands<'a> {
    pub fn remove<B: Bundle>(&mut self) -> &mut Self {
        self.add(move |entity, world: &mut World| {
            world.entity_mut(entity).remove::<B>();
        })
    }
    pub fn insert(&mut self, bundle: impl Bundle) -> &mut Self {
        self.add(move |entity, world: &mut World| {
            world.entity_mut(entity).insert(bundle);
        })
    }
    pub fn add<M: 'static>(&mut self, command: impl EntityCommand<M>) -> &mut Self {
        let db_entity = self.entity;
        self.commands.add(move |world: &mut World| {
            let entity = world.db_entity(db_entity).id();
            command.apply(entity, world)
        });
        self
    }
}

pub trait EntityCommandsExt {
    fn load(&mut self) -> &mut Self;
}

impl<'a> EntityCommandsExt for EntityCommands<'a> {
    fn load(&mut self) -> &mut Self {
        self.add(|entity, world: &mut World| {
            world.load(entity);
        })
    }
}

pub trait CommandsExt<'a> {
    fn db_entity(&'a mut self, db_entity: DbEntity) -> DbEntityCommands<'a>;
}

impl<'a> CommandsExt<'a> for &mut Commands<'a, 'a> {
    fn db_entity(&'a mut self, db_entity: DbEntity) -> DbEntityCommands<'a> {
        DbEntityCommands {
            entity: db_entity,
            commands: self.reborrow(),
        }
    }
}
/// Extension trait to make it easier to persist components with an [App].
pub trait AppExt {
    fn persist_component<T: Component + Reflect + GetTypeRegistration>(&mut self) -> &mut Self;
}

impl AppExt for App {
    fn persist_component<T: Component + Reflect + GetTypeRegistration>(&mut self) -> &mut Self {
        self.register_type::<T>();
        self.world
            .resource_mut::<PersistComponents>()
            .register::<T>();
        self.add_systems(
            PostUpdate,
            (mark_dirty::<T>, (delete_removed::<T>, save_dirty::<T>))
                .chain()
                .in_set(DbSystem),
        )
        .register_callback_in_set::<Saving<T>>(PostUpdate, DbSystem)
        .register_callback_in_set::<Deleting<T>>(PostUpdate, DbSystem)
        .add_systems(
            Last,
            exit_flush::<T>
                .run_if(on_event::<AppExit>())
                .in_set(DbSystem),
        );
        self
    }
}
