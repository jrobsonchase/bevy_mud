//! # The Canton Map System
//!
//! Who knows if this is anywhere near correct/final.
//!
//! Maps are hierarchical. Map -> Tile -> Errthang else.

use bevy::{
  ecs::{
    query::WorldQuery,
    system::SystemParam,
  },
  prelude::*,
  utils::HashMap,
};
use ratatui::{
  prelude::Rect,
  widgets::Widget,
};

use crate::{
  ascii_map::{
    render::Ansi,
    widget::HexMap,
  },
  character::Player,
  coords::Cubic,
  net::TelnetOut,
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
    app.register_type::<Tiles>();
    app.register_type::<MapWidget>();

    app.insert_resource(Maps::default());

    app.add_systems(
      PreUpdate,
      (track_maps_system, apply_deferred, track_tiles_system).chain(),
    );
    app.add_systems(PostUpdate, player_map_system);
  }
}

#[derive(Resource, Default)]
pub struct Maps {
  pub by_name: HashMap<MapName, Entity>,
  pub by_id: HashMap<Entity, MapName>,
}

/// Quick reference for all of the tiles that are children of a particular map.
#[derive(Component, Default, Reflect, Eq, PartialEq)]
#[reflect(Component)]
pub struct Tiles {
  pub by_coords: HashMap<MapCoords, Entity>,
  pub by_id: HashMap<Entity, MapCoords>,
}

#[derive(SystemParam)]
pub struct MapTilesMut<'w, 's> {
  pub maps: Res<'w, Maps>,
  pub tiles: Query<'w, 's, &'static mut Tiles>,
}

#[derive(SystemParam)]
pub struct MapTiles<'w, 's> {
  pub maps: Res<'w, Maps>,
  pub tiles: Query<'w, 's, &'static Tiles>,
}

impl<'w, 's> MapTilesMut<'w, 's> {
  pub fn by_name_mut(&mut self, name: &MapName) -> Option<(Entity, Mut<Tiles>)> {
    let id = self.maps.by_name.get(name).copied()?;
    let tiles = self.tiles.get_mut(id).ok()?;
    Some((id, tiles))
  }
  pub fn by_name(&self, name: &MapName) -> Option<(Entity, &Tiles)> {
    let id = self.maps.by_name.get(name).copied()?;
    let tiles = self.tiles.get(id).ok()?;
    Some((id, tiles))
  }
  pub fn by_id_mut(&mut self, id: Entity) -> Option<Mut<Tiles>> {
    self.tiles.get_mut(id).ok()
  }
  pub fn by_id(&self, id: Entity) -> Option<&Tiles> {
    self.tiles.get(id).ok()
  }
}

impl<'w, 's> MapTiles<'w, 's> {
  pub fn by_name(&self, name: &MapName) -> Option<(Entity, &Tiles)> {
    let id = self.maps.by_name.get(name).copied()?;
    let tiles = self.tiles.get(id).ok()?;
    Some((id, tiles))
  }
  pub fn by_id(&self, id: Entity) -> Option<&Tiles> {
    self.tiles.get(id).ok()
  }
}

fn track_tiles_system(
  mut cmd: Commands,
  mut map_tiles: MapTilesMut,
  parent_query: Query<&Parent>,
  tile_query: Query<
    (Entity, &MapName, &MapCoords),
    (Or<(Changed<MapName>, Changed<MapCoords>)>, With<Tile>),
  >,
  mut tiles_removed: RemovedComponents<Tile>,
) {
  for (ent, map, loc) in tile_query.iter() {
    // Walk up the hierarchy to find the parent map, and remove this tile by id.
    for p in parent_query.iter_ancestors(ent) {
      if let Some(mut tiles) = map_tiles.by_id_mut(p) {
        if let Some(coords) = tiles.by_id.remove(&ent) {
          tiles.by_coords.remove(&coords);
        }
        break;
      }
    }
    // Insert the tile into the new map and set its parent.
    if let Some((map_ent, mut tiles)) = map_tiles.by_name_mut(map) {
      tiles.by_id.insert(ent, *loc);
      tiles.by_coords.insert(*loc, ent);
      cmd.entity(ent).set_parent(map_ent);
    }
  }

  // Handle tiles that have gone away entirely.
  // Since this should alawys be via a despawn (since tiles should never become
  // non-tiles), we won't have any more information and need to simply iterate
  // over all maps to find who owns it.
  for ent in tiles_removed.iter() {
    for mut tiles in map_tiles.tiles.iter_mut() {
      if let Some(coords) = tiles.by_id.remove(&ent) {
        tiles.by_coords.remove(&coords);
      }
    }
  }
}

fn track_maps_system(
  mut cmd: Commands,
  mut maps: ResMut<Maps>,
  map_added: Query<(Entity, &MapName), Added<Map>>,
  mut removed: RemovedComponents<Map>,
) {
  for (ent, name) in map_added.iter() {
    maps.by_name.insert(name.clone(), ent);
    maps.by_id.insert(ent, name.clone());
    cmd.entity(ent).insert(Tiles::default());
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
#[derive(
  Component, Reflect, Clone, Deref, Default, Eq, PartialEq, Ord, PartialOrd, Hash, Debug,
)]
#[reflect(Component)]
pub struct MapName(pub String);

/// The location of an entity within its current map.
#[derive(Component, Reflect, Clone, Copy, Deref, Default, Eq, PartialEq, Hash, Debug)]
#[reflect(Component, Hash)]
pub struct MapCoords(pub Cubic);

#[derive(WorldQuery, Debug)]
#[world_query(mutable)]
#[world_query(derive(Debug))]
pub struct PuppetLocation<'a> {
  entity: Entity,
  coords: &'a MapCoords,
  map: &'a MapName,
  puppet: &'a Player,
  widget: Option<&'a mut MapWidget>,
}

#[derive(Component, Reflect, Default, Debug, Deref, DerefMut)]
#[reflect(Component)]
pub struct MapWidget(#[reflect(ignore)] HexMap);

type LocationChanged = Or<(Changed<MapCoords>, Changed<MapName>)>;

pub fn player_map_system(
  mut cmd: Commands,
  map_tiles: MapTiles,
  mut puppet_query: Query<PuppetLocation, LocationChanged>,
  player_query: Query<&TelnetOut>,
) {
  for puppet in puppet_query.iter_mut() {
    let out = player_query.get(**puppet.puppet).unwrap();
    let (_, tiles) = map_tiles.by_name(puppet.map).unwrap();
    let tile = tiles.by_coords.get(puppet.coords).copied();
    if let Some(tile) = tile {
      cmd.entity(puppet.entity).set_parent(tile);
    } else {
      warn!(?puppet, "moved to an invalid tile");
      cmd.entity(puppet.entity).remove_parent();
      out.line("You are off the map!");
      continue;
    }

    let maybe_widget = puppet.widget;

    let mut widget = try_opt!(maybe_widget, continue);

    widget.clear();
    widget.center(**puppet.coords);
    for coord in puppet.coords.spiral(5) {
      if tiles.by_coords.get(&MapCoords(coord)).is_some() {
        widget.insert(coord, Default::default());
      };
    }
    widget.tile(**puppet.coords).center().symbol("O");
    let mut renderer = Ansi::default();
    renderer.resize(Rect {
      width: 40,
      height: 18,
      x: 0,
      y: 0,
    });
    widget.render(*renderer.area(), &mut renderer);
    out.line(format!("Map:\n{}", renderer));
  }
}
