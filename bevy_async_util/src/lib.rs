use std::{future::Future, marker::PhantomData, pin::Pin, task::Context, task::Poll};

use bevy::{
    core::FrameCount,
    ecs::{
        schedule::ScheduleLabel,
        system::{Command, EntityCommands},
    },
    prelude::*,
};
use bevy::{
    ecs::system::EntityCommand,
    tasks::{block_on, poll_once, IoTaskPool, Task},
};
use nohup::NoHup;

pub mod nohup;

pub struct BoxEntityCommand(Box<dyn FnOnce(Entity, &mut World) + Send + 'static>);

impl EntityCommand for BoxEntityCommand {
    fn apply(self, id: Entity, world: &mut World) {
        (self.0)(id, world)
    }
}

impl BoxEntityCommand {
    pub fn new<C: EntityCommand + Send + 'static>(value: C) -> Self {
        BoxEntityCommand(Box::new(|id, world| value.apply(id, world)))
    }
}

pub trait Callback: Component + Future<Output = Self::Then> + Unpin {
    type Then: EntityCommand;
}

impl<T> Callback for T
where
    T: Component + Future + Unpin,
    T::Output: EntityCommand,
{
    type Then = T::Output;
}

#[derive(Component)]
pub struct NoHupCallback(NoHup<BoxEntityCommand>);

impl Future for NoHupCallback {
    type Output = BoxEntityCommand;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.as_mut().0).poll(cx)
    }
}

#[derive(Component)]
pub struct TaskCallback(Task<BoxEntityCommand>);

impl Future for TaskCallback {
    type Output = BoxEntityCommand;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.as_mut().0).poll(cx)
    }
}

fn run_callback<C>(entity: Entity, cmds: &ParallelCommands, cb: &mut C)
where
    C: Callback,
{
    let Some(task_result) = check_task(cb) else {
        return;
    };

    cmds.command_scope(move |mut cmds| {
        cmds.add(move |world: &mut World| {
            if let Some(mut entity) = world.get_entity_mut(entity) {
                entity.remove::<C>();
            }
            task_result.apply(entity, world);
        });
    });
}

fn run_callbacks<T: Callback>(cmds: ParallelCommands, mut query: Query<(Entity, &mut T)>) {
    query.par_iter_mut().for_each(|(entity, mut cb)| {
        run_callback(entity, &cmds, &mut *cb);
    })
}

fn check_task<F, T>(fut: &mut F) -> Option<T>
where
    F: Future<Output = T> + Unpin,
{
    block_on(poll_once(fut))
}

fn new_nohup_callback<T>(
    fut: impl Future<Output = T> + Send + 'static,
    cb: impl FnOnce(T, Entity, &mut World) + Send + Sync + 'static,
) -> NoHupCallback
where
    T: Send + 'static,
{
    let rt = IoTaskPool::get();
    let task = NoHup::new(rt.spawn(async move {
        let task_result = fut.await;
        BoxEntityCommand::new(move |entity, world: &mut World| cb(task_result, entity, world))
    }));
    NoHupCallback(task)
}

fn new_task_callback<T>(
    fut: impl Future<Output = T> + Send + 'static,
    cb: impl FnOnce(T, Entity, &mut World) + Send + Sync + 'static,
) -> TaskCallback
where
    T: Send + 'static,
{
    let rt = IoTaskPool::get();
    let task = rt.spawn(async move {
        let task_result = fut.await;
        BoxEntityCommand::new(move |entity, world: &mut World| cb(task_result, entity, world))
    });
    TaskCallback(task)
}

pub struct SpawnCallback<T, F, O> {
    fut: F,
    cb: O,
    nohup: bool,
    _ph: PhantomData<fn() -> T>,
}

impl<T, F, O> Command for SpawnCallback<T, F, O>
where
    T: Send + 'static,
    F: Future<Output = T> + Send + 'static,
    O: FnOnce(T, &mut World) + Send + Sync + 'static,
{
    fn apply(self, world: &mut World) {
        if self.nohup {
            let callback = new_nohup_callback(self.fut, |v, ent, world| {
                world.despawn(ent);
                (self.cb)(v, world)
            });
            world.spawn(callback);
        } else {
            let callback = new_task_callback(self.fut, |v, ent, world| {
                world.despawn(ent);
                (self.cb)(v, world)
            });
            world.spawn(callback);
        }
    }
}

pub trait CommandsExt {
    fn spawn_callback<T, F, O>(&mut self, fut: F, cb: O)
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
        O: FnOnce(T, &mut World) + Send + Sync + 'static;
    fn spawn_nohup_callback<T, F, O>(&mut self, fut: F, cb: O)
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
        O: FnOnce(T, &mut World) + Send + Sync + 'static;
}

impl CommandsExt for Commands<'_, '_> {
    fn spawn_nohup_callback<T, F, O>(&mut self, fut: F, cb: O)
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
        O: FnOnce(T, &mut World) + Send + Sync + 'static,
    {
        self.add(SpawnCallback {
            fut,
            cb,
            nohup: true,
            _ph: PhantomData,
        })
    }

    fn spawn_callback<T, F, O>(&mut self, fut: F, cb: O)
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
        O: FnOnce(T, &mut World) + Send + Sync + 'static,
    {
        self.add(SpawnCallback {
            fut,
            cb,
            nohup: false,
            _ph: PhantomData,
        })
    }
}

pub trait EntityCommandsExt {
    fn attach_nohup_callback<T, F, O>(&mut self, fut: F, cb: O) -> &mut Self
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
        O: FnOnce(T, Entity, &mut World) + Send + Sync + 'static;
    fn attach_callback<T, F, O>(&mut self, fut: F, cb: O) -> &mut Self
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
        O: FnOnce(T, Entity, &mut World) + Send + Sync + 'static;
}

impl EntityCommandsExt for EntityWorldMut<'_> {
    fn attach_nohup_callback<T, F, O>(&mut self, fut: F, cb: O) -> &mut Self
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
        O: FnOnce(T, Entity, &mut World) + Send + Sync + 'static,
    {
        trace!(
            frame = self.world().resource::<FrameCount>().0,
            "attaching async task"
        );
        self.insert(new_nohup_callback(fut, cb));
        self
    }
    fn attach_callback<T, F, O>(&mut self, fut: F, cb: O) -> &mut Self
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
        O: FnOnce(T, Entity, &mut World) + Send + Sync + 'static,
    {
        trace!(
            frame = self.world().resource::<FrameCount>().0,
            "attaching async task"
        );
        self.insert(new_task_callback(fut, cb));
        self
    }
}

impl EntityCommandsExt for EntityCommands<'_> {
    fn attach_nohup_callback<T, F, O>(&mut self, fut: F, cb: O) -> &mut Self
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
        O: FnOnce(T, Entity, &mut World) + Send + Sync + 'static,
    {
        self.add(move |mut entity: EntityWorldMut| {
            entity.attach_nohup_callback(fut, cb);
        })
    }
    fn attach_callback<T, F, O>(&mut self, fut: F, cb: O) -> &mut Self
    where
        T: Send + 'static,
        F: Future<Output = T> + Send + 'static,
        O: FnOnce(T, Entity, &mut World) + Send + Sync + 'static,
    {
        self.add(move |mut entity: EntityWorldMut| {
            entity.attach_callback(fut, cb);
        })
    }
}

pub struct CallbackPlugin;

impl Plugin for CallbackPlugin {
    fn build(&self, app: &mut App) {
        app.register_callback::<TaskCallback>(PreUpdate)
            .register_callback::<NoHupCallback>(PreUpdate);
    }
}

/// The default system set callback systems are registered in.
#[derive(SystemSet, Debug, Clone, Copy, Hash, Default, PartialEq, Eq)]
pub struct CallbackSystem;

pub trait AppExt {
    fn register_callback_in_set<T: Callback>(
        &mut self,
        label: impl ScheduleLabel,
        set: impl SystemSet,
    ) -> &mut Self;
    fn register_callback<T: Callback>(&mut self, label: impl ScheduleLabel) -> &mut Self {
        self.register_callback_in_set::<T>(label, CallbackSystem)
    }
}

impl AppExt for App {
    fn register_callback_in_set<T: Callback>(
        &mut self,
        label: impl ScheduleLabel,
        set: impl SystemSet,
    ) -> &mut Self {
        let system = run_callbacks::<T>.run_if(any_with_component::<T>);
        self.add_systems(label, system.in_set(set))
    }
}
