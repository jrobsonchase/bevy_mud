use bevy::prelude::*;

use super::{
  assets::SavedEntity,
  components::{
    Save,
    SavedEntityState,
  },
  helpers::write_saved_entity,
  resources::SavedEntityStates,
};

pub fn save_added(
  trigger: Trigger<OnAdd, Save>,
  query: Query<&Save, Without<Handle<SavedEntity>>>,
  asset_server: Res<AssetServer>,
  mut cmd: Commands,
) {
  let entity = trigger.entity();
  let Ok(data) = query.get(entity) else {
    return;
  };

  let handle = asset_server.load::<SavedEntity>(data);

  cmd
    .entity(entity)
    .insert((handle, SavedEntityState::default()));
}

pub fn saved_entity_added(
  trigger: Trigger<OnAdd, Handle<SavedEntity>>,
  mut states: ResMut<SavedEntityStates>,
  assets: Res<Assets<SavedEntity>>,
  query: Query<&Handle<SavedEntity>>,
  mut cmd: Commands,
) {
  let entity = trigger.entity();
  let handle = query.get(entity).unwrap();

  states
    .handle_entities
    .entry(handle.id())
    .or_default()
    .insert(entity);

  let Some(asset) = assets.get(handle) else {
    // warn!("no saved entity found for {handle:?}");
    return;
  };

  cmd.queue(write_saved_entity(entity, asset));
}
