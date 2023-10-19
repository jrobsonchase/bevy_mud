use std::marker::PhantomData;

use bevy::{
  ecs::system::Command,
  prelude::*,
};

struct RunSystem<M, S> {
  system: S,
  marker: std::marker::PhantomData<fn(&M)>,
}

impl<M: 'static, S: IntoSystemConfigs<M> + Send + 'static> RunSystem<M, S> {
  fn new(sys: S) -> Self {
    Self {
      system: sys,
      marker: PhantomData,
    }
  }
}

impl<M: 'static, S: IntoSystemConfigs<M> + Send + 'static> Command for RunSystem<M, S> {
  fn apply(self, world: &mut World) {
    Schedule::default().add_systems(self.system).run(world);
  }
}

pub fn run_system<M: 'static>(sys: impl IntoSystemConfigs<M> + Send + 'static) -> impl Command {
  RunSystem::new(sys)
}
