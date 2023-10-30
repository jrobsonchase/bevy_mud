use std::{
  fmt::Debug,
  future::Future,
  pin::Pin,
  task::{
    Context,
    Poll,
  },
};

use anyhow::Error;
use bevy::{
  core::FrameCount,
  ecs::system::{
    Command,
    EntityCommands,
  },
  prelude::*,
};
use futures::{
  executor::block_on,
  future::poll_immediate,
  ready,
  FutureExt,
};
use tokio::{
  runtime::Handle,
  task::JoinHandle,
};

#[derive(Default)]
pub struct TokioPlugin(Option<Handle>);

impl TokioPlugin {
  pub fn new(handle: Option<Handle>) -> Self {
    Self(handle)
  }
}

#[derive(Resource, Deref, DerefMut)]
pub struct TokioRuntime(Handle);

struct BoxCommand(Box<dyn FnOnce(&mut World) + Send + 'static>);

impl Command for BoxCommand {
  fn apply(self, world: &mut World) {
    (self.0)(world)
  }
}

impl BoxCommand {
  fn new<C: Command + Send + 'static>(value: C) -> Self {
    BoxCommand(Box::new(|world| value.apply(world)))
  }
}

type ErrCb = Box<dyn FnOnce(Error, Entity, &mut World) + Send + Sync + 'static>;

#[derive(Component)]
pub struct Callback(Task<BoxCommand>, Option<ErrCb>);

fn run_callbacks(
  frame: Res<FrameCount>,
  cmds: ParallelCommands,
  mut query: Query<(Entity, &mut Callback)>,
) {
  query.iter_mut().for_each(|(entity, mut cb)| {
    let Callback(task, err_cb) = &mut *cb;
    let task_result = try_opt!(check_task(task), return);
    let err_cb = err_cb.take().expect("Callback can only complete once");

    let frame = frame.0;

    cmds.command_scope(move |mut cmds| {
      if let Some(mut entity_cmds) = cmds.get_entity(entity) {
        entity_cmds.remove::<Callback>();
        trace!(frame, ?entity, "running async task callback");
        match task_result {
          Ok(cb) => cmds.add(move |world: &mut World| (cb.0)(world)),
          Err(error) => {
            cmds.add(move |world: &mut World| err_cb(error, entity, world));
          }
        };
      } else {
        warn!(?entity, "entity despawned before callbacks could run");
      }
    });
  })
}

pub struct Task<T> {
  fut: JoinHandle<Result<T, Error>>,
}

impl<T> Future for Task<T> {
  type Output = Result<T, Error>;

  fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
    let res = ready!(self.fut.poll_unpin(cx))?;
    Poll::Ready(res)
  }
}

impl<T> Task<T> {
  #[allow(dead_code)]
  pub fn check(&mut self) -> Option<Result<T, Error>> {
    check_task(self)
  }
}

impl TokioRuntime {
  pub fn block_on<F>(&self, fut: F) -> F::Output
  where
    F: Future,
  {
    self.0.block_on(fut)
  }
  pub fn handle(&self) -> Handle {
    self.0.clone()
  }
  pub fn spawn<F, T>(&self, fut: F) -> Task<T>
  where
    F: Future<Output = Result<T, Error>> + Send + 'static,
    T: Send + 'static,
  {
    Task {
      fut: self.0.spawn(fut),
    }
  }
}

impl Plugin for TokioPlugin {
  fn build(&self, app: &mut App) {
    let handle = self.0.clone().unwrap_or_else(|| {
      tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap()
        .handle()
        .clone()
    });
    app.insert_resource(TokioRuntime(handle)).add_systems(
      PreUpdate,
      run_callbacks.run_if(any_with_component::<Callback>()),
    );
  }
}

pub fn check_task<F, T>(fut: &mut F) -> Option<T>
where
  F: Future<Output = T> + Unpin,
{
  block_on(poll_immediate(fut))
}

pub trait EntityCommandsExt {
  fn spawn_callback<T, F, O, E>(&mut self, fut: F, cb: O, on_err: E) -> &mut Self
  where
    T: Debug + Send + 'static,
    F: Future<Output = anyhow::Result<T>> + Send + 'static,
    O: FnOnce(T, Entity, &mut World) + Send + Sync + 'static,
    E: FnOnce(anyhow::Error, Entity, &mut World) + Send + Sync + 'static;
}

impl EntityCommandsExt for EntityCommands<'_, '_, '_> {
  fn spawn_callback<T, F, O, E>(&mut self, fut: F, cb: O, on_err: E) -> &mut Self
  where
    T: Debug + Send + 'static,
    F: Future<Output = anyhow::Result<T>> + Send + 'static,
    O: FnOnce(T, Entity, &mut World) + Send + Sync + 'static,
    E: FnOnce(anyhow::Error, Entity, &mut World) + Send + Sync + 'static,
  {
    self.add(move |entity: Entity, world: &mut World| {
      let rt = world.resource::<TokioRuntime>();
      let frame = world.resource::<FrameCount>().0;
      trace!(frame, "spawning async task");
      let task = rt.spawn(async move {
        let task_result = fut.await?;
        trace!(?task_result, ?entity, "async task completed");
        Ok(BoxCommand::new(move |world: &mut World| {
          cb(task_result, entity, world)
        }))
      });
      let err_cb = Box::new(on_err);
      world
        .entity_mut(entity)
        .insert(Callback(task, Some(err_cb)));
    })
  }
}
