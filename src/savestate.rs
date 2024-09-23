use bevy::{
  app::AppExit,
  prelude::*,
};

use self::{
  assets::{
    SavedEntity,
    SavedEntityLoader,
  },
  components::Save,
};
use crate::{
  core::MudStartup,
  util::DebugLifecycle,
};

pub mod assets;
pub mod components;
pub mod events;
pub mod helpers;
pub mod observers;
pub mod resources;
pub mod systems;
pub mod traits;

use traits::AppWorldExt;

#[derive(Debug, Clone, Copy)]
pub struct SaveStatePlugin(f32);

impl Default for SaveStatePlugin {
  fn default() -> Self {
    Self::with_interval(SAVE_INTERVAL)
  }
}

impl SaveStatePlugin {
  pub fn with_interval(secs: f32) -> Self {
    Self(secs)
  }
}

const SAVE_INTERVAL: f32 = 30.0;

impl Plugin for SaveStatePlugin {
  fn build(&self, app: &mut App) {
    app
      .init_resource::<resources::PersistentComponents>()
      .insert_resource(resources::SaveInterval(self.0))
      .register_type::<components::Persistent>()
      .add_event::<events::LoadFailed>()
      .debug_lifecycle::<components::Persistent>("Persistent")
      .add_systems(Startup, systems::load_system.in_set(MudStartup::World))
      .add_systems(
        Last,
        systems::final_save_system
          .run_if(on_event::<AppExit>.and(not(on_event::<events::LoadFailed>))),
      )
      .add_systems(Last, systems::save_system.run_if(not(on_event::<AppExit>)));

    app
      .persist::<Save>()
      .init_asset::<SavedEntity>()
      .init_asset_loader::<SavedEntityLoader>()
      .insert_resource(resources::SavedEntityStates::default())
      .add_systems(PreUpdate, systems::handle_asset_events)
      .debug_lifecycle::<Save>("Save")
      .debug_lifecycle::<Handle<SavedEntity>>("Handle<SavedEntity>")
      .observe(observers::saved_entity_added)
      .observe(observers::save_added);
  }
}
