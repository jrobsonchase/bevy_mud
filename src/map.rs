//! # The Canton Map System
//!
//! Who knows if this is anywhere near correct/final.
//!
//! Maps are hierarchical. Map -> Tile -> Errthang else.

use bevy::{
  prelude::*,
  utils::HashMap,
};

use crate::{
  coords::Cubic,
  savestate::SaveExt,
};

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app
      .persist_component::<MapName>()
      .persist_component::<MapCoords>()
      .persist_component::<Map>()
      .persist_component::<Tile>();

    app.register_type::<Cubic>();

    app.insert_resource(Maps::default());

    app.add_systems(PreUpdate, track_maps_system);
  }
}

#[derive(Resource, Default)]
pub struct Maps {
  pub by_name: HashMap<MapName, Entity>,
  pub by_id: HashMap<Entity, MapName>,
}

/// Quick reference for all of the tiles that are children of a particular map.
#[derive(Component, Default)]
pub struct Tiles {
  pub by_coords: HashMap<MapCoords, Entity>,
  pub by_id: HashMap<Entity, MapCoords>,
}

fn track_maps_system(
  mut maps: ResMut<Maps>,
  map_added: Query<(Entity, &MapName), Added<Map>>,
  mut removed: RemovedComponents<Map>,
) {
  for (ent, name) in map_added.iter() {
    maps.by_name.insert(name.clone(), ent);
    maps.by_id.insert(ent, name.clone());
  }

  for ent in removed.iter() {
    if let Some(name) = maps.by_id.remove(&ent) {
      maps.by_name.remove(&name);
    }
  }
}

/// Marker component for maps.
#[derive(Component, Reflect, Copy, Clone, Default)]
#[reflect(Component)]
pub struct Map;

/// Marker component for map tiles.
/// Must also have a [MapName] and [MapCoords].
/// Must also have a [Parent] that is a [Map].
#[derive(Component, Reflect, Copy, Clone, Default)]
#[reflect(Component)]
pub struct Tile;

/// Either the name of this map (if the entity is tagged with [Map]), or the
/// name of the map the entity is in.
#[derive(Component, Reflect, Clone, Deref, Default, Eq, PartialEq, Ord, PartialOrd, Hash)]
#[reflect(Component)]
pub struct MapName(pub String);

/// The location of an entity within its current map.
#[derive(Component, Reflect, Clone, Deref, Default, Eq, PartialEq, Hash)]
#[reflect(Component)]
pub struct MapCoords(pub Cubic);
