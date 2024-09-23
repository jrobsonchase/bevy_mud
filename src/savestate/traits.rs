use std::any::TypeId;

use bevy::{
  prelude::*,
  reflect::GetTypeRegistration,
};

pub trait AppWorldExt {
  fn persist<C: Component + GetTypeRegistration>(&mut self) -> &mut Self;
}

impl AppWorldExt for World {
  fn persist<C: Component + GetTypeRegistration>(&mut self) -> &mut Self {
    self
      .resource_mut::<super::resources::PersistentComponents>()
      .components
      .write()
      .unwrap()
      .insert(TypeId::of::<C>());
    self
  }
}

impl AppWorldExt for App {
  fn persist<C: Component + GetTypeRegistration>(&mut self) -> &mut Self {
    self.register_type::<C>();
    self.world_mut().persist::<C>();
    self
  }
}
