use std::future::Future;
use std::pin::Pin;
use std::task::Context;
use std::task::Poll;

use bevy::tasks::IoTaskPool;
use bevy::tasks::Task;

pub struct NoHup<T>(Option<Task<T>>);

impl<T> NoHup<T> {
    pub fn new(task: Task<T>) -> Self {
        NoHup(Some(task))
    }
}

impl<T> From<Task<T>> for NoHup<T> {
    fn from(value: Task<T>) -> Self {
        Self::new(value)
    }
}

impl<T> Future for NoHup<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        Pin::new(&mut self.0.as_mut().expect("poll after drop")).poll(cx)
    }
}

impl<T> Drop for NoHup<T> {
    fn drop(&mut self) {
        self.0.take().expect("nohup drop once").detach()
    }
}

pub fn nohup<T: Send + 'static>(f: impl Future<Output = T> + Send + 'static) -> NoHup<T> {
    NoHup::new(IoTaskPool::get().spawn(f))
}
