use std::{
  future::Future,
  ops::{
    Deref,
    DerefMut,
  },
  pin::Pin,
  task::{
    Context,
    Poll,
  },
};

use anyhow::Error;
use bevy::prelude::*;
use futures::{
  executor::block_on,
  future::poll_immediate,
  ready,
  FutureExt,
};
use tokio::task::JoinHandle;

pub struct TokioPlugin;

#[derive(Resource)]
pub struct TokioRuntime(tokio::runtime::Runtime);

impl Deref for TokioRuntime {
  type Target = tokio::runtime::Runtime;
  fn deref(&self) -> &Self::Target {
    &self.0
  }
}

impl DerefMut for TokioRuntime {
  fn deref_mut(&mut self) -> &mut Self::Target {
    &mut self.0
  }
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
    app.insert_resource(TokioRuntime(
      tokio::runtime::Builder::new_multi_thread()
        .enable_all()
        .build()
        .unwrap(),
    ));
  }
}

pub fn check_task<F, T>(fut: &mut F) -> Option<T>
where
  F: Future<Output = T> + Unpin,
{
  block_on(poll_immediate(fut))
}
