//! # The Mud Map System
//!
//! Who knows if this is anywhere near correct/final.
//!
//! Maps are hierarchical. Map -> Tile -> Errthang else.

use std::io::Write;

use base64::Engine as _;
use bevy::{
  ecs::{
    entity::EntityHashSet,
    system::SystemParam,
  },
  prelude::*,
  utils::HashMap,
};
use hexx::{
  EdgeDirection,
  Hex,
  HexLayout,
};
use ratatui::{
  prelude::Rect,
  style::{
    Color as TuiColor,
    Style as TuiStyle,
  },
  widgets::Widget,
};
use serde::{
  Deserialize,
  Serialize,
};

use crate::{
  ascii_map::{
    render::Ansi,
    widget::{
      Color,
      HexMap,
      Modifier,
      Style,
    },
  },
  character::Player,
  core::{
    Live,
    LiveQuery,
    MudStartup,
  },
  net::{
    TelnetOut,
    GMCP,
  },
  savestate::traits::AppWorldExt,
  util::{
    debug_trigger,
    DebugLifecycle,
  },
};

const MAP_RADIUS: u32 = 9;

pub struct MapPlugin;

#[derive(Resource, Debug, Reflect, Clone, Copy, Eq, PartialEq)]
#[reflect(Resource)]
pub struct MapConfig {
  pub init_res_power: u32,
  pub extra_resolutions: usize,
}

impl Default for MapConfig {
  fn default() -> Self {
    Self {
      init_res_power: 4,
      extra_resolutions: 4,
    }
  }
}

impl MapConfig {
  pub fn radii(&self) -> impl Iterator<Item = (usize, u32)> {
    let base = self.init_res_power;
    (0..self.extra_resolutions + 1).map(move |i| {
      if i == 0 {
        (i, 0)
      } else {
        (i, base * 2u32.pow(i as u32 - 1))
      }
    })
  }
  pub fn hex_resolutions(&self, hex: Hex) -> impl Iterator<Item = (usize, Hex)> {
    self.radii().map(move |(i, r)| (i, hex.to_lower_res(r)))
  }
}

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<RenderRequest>().add_event::<Moved>();
    app
      .persist::<Name>()
      .persist::<Map>()
      .persist::<MapWidget>()
      .persist::<Transform>()
      .persist::<Render>()
      .persist::<Tile>();

    app
      .register_type::<Name>()
      .register_type::<Map>()
      .register_type::<MapWidget>()
      .register_type::<Transform>()
      .register_type::<Render>()
      .register_type::<Tile>()
      .register_type::<GlobalTransform>()
      .register_type::<Symbol>()
      .register_type::<Option<Symbol>>()
      .register_type::<Style>()
      .register_type::<Option<Style>>()
      .register_type::<Color>()
      .register_type::<Option<Color>>()
      .register_type::<Modifier>()
      .register_type::<Hex>()
      .register_type::<EdgeDirection>()
      .register_type::<Entities>()
      .register_type::<MapWidget>();

    app.insert_resource(Maps::default());

    app
      .add_systems(
        Startup,
        (|world: &mut World| {
          if !world.contains_resource::<MapConfig>() {
            world.insert_resource(MapConfig::default());
          }
        })
        .in_set(MudStartup::System),
      )
      .add_systems(PostUpdate, propagate_transforms)
      .debug_lifecycle::<Map>("Map")
      .observe(debug_trigger::<Moved>)
      .observe(debug_trigger::<RenderRequest>)
      .observe(render_map_system)
      .observe(moved_to_render)
      .observe(live_added)
      .observe(live_removed)
      .observe(map_added)
      .observe(map_removed);

    app
      .world_mut()
      .register_component_hooks::<Transform>()
      .on_add(|mut world, entity, _| {
        world
          .commands()
          .entity(entity)
          .insert(GlobalTransform::default());
      })
      .on_remove(|mut world, entity, _| {
        world.commands().entity(entity).remove::<GlobalTransform>();
      });
  }
}

#[derive(Resource, Default)]
pub struct Maps {
  pub by_name: HashMap<String, Entity>,
  pub by_id: HashMap<Entity, String>,
}

/// Quick reference for all of the entities that are on a particular map.
#[derive(Reflect, Component, Default, Eq, PartialEq)]
#[reflect(Component)]
pub struct Entities {
  pub config: MapConfig,
  #[reflect(ignore)]
  pub by_coords: Vec<HashMap<Hex, Vec<Entity>>>,
  #[reflect(ignore)]
  pub by_id: HashMap<Entity, GlobalTransform>,
}

impl Entities {
  fn new(config: MapConfig) -> Self {
    Self {
      config,
      by_coords: vec![Default::default(); config.extra_resolutions + 1],
      ..Default::default()
    }
  }
  fn iter_at(&self, coord: Hex, resolution: usize) -> impl Iterator<Item = Entity> + '_ {
    self.by_coords[resolution]
      .get(&coord)
      .into_iter()
      .flat_map(|entities| entities.iter().copied())
  }

  fn find_within(
    &self,
    coord: Hex,
    radius: u32,
  ) -> impl Iterator<Item = (Entity, &GlobalTransform)> + '_ {
    self
      .config
      .radii()
      .find(|(_, r)| *r >= radius/2)
      .into_iter()
      .flat_map(move |(i, r)| {
        let downscaled = coord.to_lower_res(r);
        debug!(resolution = i, coord = ?downscaled, resolution_radius = r, radius, "returning entities");
        self.iter_at(downscaled, i).chain(
          downscaled
            .all_neighbors()
            .into_iter()
            .flat_map(move |coord| self.iter_at(coord, i)),
        )
      })
      .filter_map(|ent| self.by_id.get(&ent).map(|xform| (ent, xform)))
  }

  fn add_entity_at_resolution(&mut self, entity: Entity, coords: Hex, resolution: usize) {
    let ents = self.by_coords[resolution].entry(coords).or_default();
    if !ents.contains(&entity) {
      ents.push(entity);
    }
  }
  fn add_entity(&mut self, entity: Entity, xform: &GlobalTransform) {
    for (i, coords) in self.config.hex_resolutions(xform.coords) {
      self.add_entity_at_resolution(entity, coords, i);
    }
    self.by_id.insert(entity, xform.clone());
  }
  fn remove_entity_at_resolution(&mut self, entity: Entity, coords: Hex, resolution: usize) {
    if let Some(ent_vec) = self.by_coords[resolution].get_mut(&coords) {
      ent_vec.retain(|e| *e != entity);
    }
  }
  fn remove_entity(&mut self, entity: Entity) -> Option<GlobalTransform> {
    if let Some(prev) = self.by_id.remove(&entity) {
      for (i, coords) in self.config.hex_resolutions(prev.coords) {
        self.remove_entity_at_resolution(entity, coords, i);
      }
      Some(prev)
    } else {
      None
    }
  }
  fn move_entity(&mut self, entity: Entity, new: &GlobalTransform) {
    let Some(prev) = self.by_id.insert(entity, new.clone()) else {
      for (i, coords) in self.config.hex_resolutions(new.coords) {
        self.add_entity_at_resolution(entity, coords, i);
      }
      return;
    };
    for (i, prev, new) in self
      .config
      .hex_resolutions(prev.coords)
      .zip(self.config.hex_resolutions(new.coords))
      .map(|((i, prev), (_, new))| (i, prev, new))
    {
      if prev != new {
        self.remove_entity_at_resolution(entity, prev, i);
        self.add_entity_at_resolution(entity, new, i);
      }
    }
  }
}

#[derive(SystemParam)]
struct MapEntitiesMut<'w, 's> {
  maps: Res<'w, Maps>,
  entities: Query<'w, 's, &'static mut Entities>,
}

#[derive(SystemParam)]
pub struct MapEntities<'w, 's> {
  maps: Res<'w, Maps>,
  entities: Query<'w, 's, &'static Entities>,
}

impl<'w, 's> MapEntitiesMut<'w, 's> {
  fn by_name_mut(&mut self, name: &str) -> Option<(Entity, Mut<Entities>)> {
    let id = self.maps.by_name.get(name).copied()?;
    let entities = self.entities.get_mut(id).ok()?;
    Some((id, entities))
  }
  fn move_entity(
    &mut self,
    id: Entity,
    from_xform: &GlobalTransform,
    to_xform: &GlobalTransform,
  ) -> bool {
    if from_xform == to_xform {
      return false;
    } else if from_xform.map == to_xform.map {
      if let Some((_, mut ents)) = self.by_name_mut(&to_xform.map) {
        ents.move_entity(id, to_xform);
      }
    } else {
      self.remove_entity(id, from_xform);
      self.add_entity(id, to_xform);
    }
    true
  }
  fn remove_entity(&mut self, id: Entity, xform: &GlobalTransform) {
    if let Some((_, mut ents)) = self.by_name_mut(&xform.map) {
      ents.remove_entity(id);
    }
  }
  fn add_entity(&mut self, id: Entity, xform: &GlobalTransform) {
    if let Some((_, mut ents)) = self.by_name_mut(&xform.map) {
      ents.add_entity(id, xform);
    };
  }
}

impl<'w, 's> MapEntities<'w, 's> {
  pub fn by_name(&self, name: &str) -> Option<(Entity, &Entities)> {
    let id = self.maps.by_name.get(name).copied()?;
    let entities = self.entities.get(id).ok()?;
    Some((id, entities))
  }
  pub fn by_id(&self, id: Entity) -> Option<&Entities> {
    self.entities.get(id).ok()
  }
}

/// Marker component for maps.
#[derive(
  Component, Reflect, Clone, Default, Deref, Hash, Eq, PartialEq, Serialize, Deserialize,
)]
#[reflect(Component, Hash)]
pub struct Map(pub String);

#[derive(
  Component, Reflect, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
#[reflect(Component, Hash)]
pub struct Transform {
  pub map: String,
  pub coords: Hex,
  pub facing: EdgeDirection,
}

#[derive(Component, Reflect, Clone, Deref, Default, Eq, PartialEq, Hash, Debug)]
#[reflect(Component, Hash)]
pub struct GlobalTransform(Transform);

/// Marker component for map tiles.
#[derive(Component, Reflect, Copy, Clone, Default, Serialize, Deserialize)]
#[reflect(Component)]
pub struct Tile;

#[derive(Reflect, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize)]
#[reflect(Hash, FromWorld)]
pub struct Symbol {
  pub text: String,
  #[serde(default)]
  pub style: Style,
}

#[derive(
  Component, Reflect, Clone, Default, Eq, PartialEq, Hash, Debug, Serialize, Deserialize,
)]
#[reflect(Component, Hash, FromWorld)]
pub struct Render {
  pub icon: Option<Symbol>,
  pub background: Option<Symbol>,
}

#[derive(Component, Reflect, Default, Debug, Deref, DerefMut)]
#[reflect(Component)]
pub struct MapWidget(#[reflect(ignore)] HexMap);

impl Serialize for MapWidget {
  fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
  where
    S: serde::Serializer,
  {
    serializer.serialize_unit()
  }
}

impl<'de> Deserialize<'de> for MapWidget {
  fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
  where
    D: serde::Deserializer<'de>,
  {
    struct WidgetVisitor;
    impl<'de> serde::de::Visitor<'de> for WidgetVisitor {
      type Value = MapWidget;

      fn expecting(&self, formatter: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(formatter, "the unit type")
      }

      fn visit_unit<E: serde::de::Error>(self) -> Result<Self::Value, E> {
        Ok(Default::default())
      }
    }

    deserializer.deserialize_any(WidgetVisitor)
  }
}

fn map_added(
  trigger: Trigger<OnAdd, Map>,
  cfg: Res<MapConfig>,
  mut cmd: Commands,
  mut maps: ResMut<Maps>,
  map_query: Query<&Map>,
) {
  let ent = trigger.entity();
  let Ok(name) = map_query.get(ent) else {
    warn!(entity = %ent.to_bits(), "map added, but not found");
    return;
  };
  debug!(entity = %ent.to_bits(), name = **name, "map added");
  maps.by_name.insert((**name).clone(), ent);
  maps.by_id.insert(ent, (**name).clone());
  cmd.entity(ent).insert(Entities::new(*cfg));
}

fn map_removed(trigger: Trigger<OnRemove, Map>, mut maps: ResMut<Maps>) {
  let ent = trigger.entity();
  if let Some(name) = maps.by_id.remove(&ent) {
    maps.by_name.remove(&name);
  }
}

fn live_removed(
  trigger: Trigger<OnRemove, Live>,
  mut cmd: Commands,
  mut map_ents: MapEntitiesMut,
  query: Query<&GlobalTransform>,
) {
  let entity = trigger.entity();
  let Ok(xform) = query.get(entity) else {
    return;
  };
  map_ents.remove_entity(entity, xform);
  cmd.trigger_targets(
    Moved {
      prev: xform.clone().into(),
      new: None,
    },
    entity,
  );
}

fn live_added(
  trigger: Trigger<OnAdd, Live>,
  mut cmd: Commands,
  mut map_ents: MapEntitiesMut,
  query: Query<&GlobalTransform>,
) {
  let entity = trigger.entity();
  let Ok(xform) = query.get(entity) else {
    return;
  };
  map_ents.add_entity(entity, xform);
  cmd.trigger_targets(
    Moved {
      prev: None,
      new: xform.clone().into(),
    },
    entity,
  );
}

fn propagate_transforms(
  mut cmd: Commands,
  mut map_ents: MapEntitiesMut,
  changed_query: Query<(Entity, Option<&Parent>), Or<(Added<Transform>, Changed<Transform>)>>,
  mut data_query: Query<(&Transform, &mut GlobalTransform, Has<Live>)>,
  children: Query<&Children>,
) {
  for (ent, parent) in changed_query.iter() {
    let parent_xform = parent
      .and_then(|p| data_query.get(p.get()).ok())
      .map(|t| t.1)
      .cloned();
    propagate_transform(
      &mut cmd,
      &mut map_ents,
      ent,
      parent_xform.as_ref(),
      &mut data_query,
      &children,
    );
  }
}

#[derive(Event, Debug)]
pub struct Moved {
  pub prev: Option<GlobalTransform>,
  pub new: Option<GlobalTransform>,
}

fn propagate_transform(
  cmd: &mut Commands,
  map_ents: &mut MapEntitiesMut,
  entity: Entity,
  parent_xform: Option<&GlobalTransform>,
  data: &mut Query<(&Transform, &mut GlobalTransform, Has<Live>)>,
  children: &Query<&Children>,
) {
  let my_xform = if let Ok((xform, mut global, live)) = data.get_mut(entity) {
    let xform = if let Some(GlobalTransform(parent)) = parent_xform {
      Transform {
        map: parent.map.clone(),
        coords: parent.coords + xform.coords.rotate_cw(parent.facing.index() as _),
        facing: xform.facing.rotate_cw(parent.facing.index() as _),
      }
    } else {
      xform.clone()
    };

    let prev = std::mem::replace(&mut *global, GlobalTransform(xform));

    if live && prev != *global {
      map_ents.move_entity(entity, &prev, &global);
      cmd.trigger_targets(
        Moved {
          prev: prev.into(),
          new: Some(global.clone()),
        },
        entity,
      );
    }

    Some(global.clone())
  } else {
    parent_xform.cloned()
  };

  for child in children.get(entity).iter().flat_map(|c| c.iter()) {
    propagate_transform(cmd, map_ents, *child, my_xform.as_ref(), data, children);
  }
}

fn moved_to_render(
  trigger: Trigger<Moved>,
  mut cmd: Commands,
  mut targets: Local<EntityHashSet>,
  map_entities: MapEntities,
  widgets: LiveQuery<(), With<MapWidget>>,
  rendered: LiveQuery<(), With<Render>>,
) {
  targets.clear();
  let entity = trigger.entity();
  let Moved { prev, new } = trigger.event();
  if widgets.contains(entity) {
    targets.insert(entity);
  }
  if rendered.contains(entity) {
    for xform in prev.iter().chain(new) {
      let (_, map) = try_opt!(map_entities.by_name(&xform.map), continue);
      for coords in xform.coords.spiral_range(0..MAP_RADIUS) {
        for other in map.by_coords[0]
          .get(&coords)
          .into_iter()
          .flat_map(|ents| ents.iter())
        {
          if targets.contains(other) {
            continue;
          }

          if widgets.contains(*other) {
            targets.insert(*other);
          }
        }
      }
    }
  }
  cmd.trigger_targets(RenderRequest, targets.drain().collect::<Vec<Entity>>());
}

#[derive(Event, Debug)]
pub struct RenderRequest;

fn render_map_system(
  trigger: Trigger<RenderRequest>,
  map_entities: MapEntities,
  mut puppet_query: LiveQuery<(&GlobalTransform, &Player, &mut MapWidget)>,
  player_query: Query<&TelnetOut, With<GMCP>>,
  render_query: LiveQuery<&Render>,
) {
  debug!(entity = trigger.entity().to_bits(), "got render request");
  let Ok((xform, player, mut widget)) = puppet_query.get_mut(trigger.entity()) else {
    debug!("couldn't find puppet, returning");
    return;
  };

  let Ok(out) = player_query.get(player.0) else {
    debug!("player doesn't have gmcp enabled, returning");
    return;
  };
  let Some((_, map)) = map_entities.by_name(&xform.map) else {
    debug!(map_name = xform.map, "map not found");
    return;
  };

  let center = xform.coords + xform.facing.into_hex() * 3;

  widget.clear();
  widget.center(center);
  widget.up_direction(xform.facing);
  for coord in xform.coords.spiral_range(0..MAP_RADIUS) {
    if !is_visible(xform, coord, MAP_RADIUS) && coord != center {
      continue;
    }
    let tile = widget.tile(coord);
    for render in map
      .iter_at(coord, 0)
      .filter_map(|e| render_query.get(e).ok())
    {
      if let Some(bg) = render.background.as_ref() {
        tile.background().symbol(&bg.text).style(bg.style.into());
      }
      if let Some(fg) = render.icon.as_ref() {
        let mut style: TuiStyle = fg.style.into();
        if coord == xform.coords {
          style = style.fg(TuiColor::Blue);
        }
        tile.center().symbol(&fg.text).style(style);
      }
    }
  }
  for (entity, location) in map.find_within(xform.coords, MAP_RADIUS) {
    if !is_visible(xform, location.coords, MAP_RADIUS) {
      continue;
    }

    let Some(render) = render_query.get(entity).ok() else {
      continue;
    };

    let tile = widget.tile(location.coords);

    if let Some(bg) = render.background.as_ref() {
      tile.background().symbol(&bg.text).style(bg.style.into());
    }
    if let Some(fg) = render.icon.as_ref() {
      let mut style: TuiStyle = fg.style.into();
      if location.coords == xform.coords {
        style = style.fg(TuiColor::Blue);
      }
      tile.center().symbol(&fg.text).style(style);
    }
  }
  let mut renderer = Ansi::default();
  let map_area = Rect {
    width: 55,
    height: 23,
    x: 0,
    y: 0,
  };
  renderer.resize(map_area);
  widget.render(map_area, &mut renderer);
  let out = out.clone();
  bevy::tasks::AsyncComputeTaskPool::get()
    .spawn(async move {
      let mut compressor = zstd::Encoder::new(Vec::new(), 0).unwrap();
      write!(compressor, "{}", renderer).unwrap();
      let bytes = compressor.finish().unwrap();
      let encoded = base64::prelude::BASE64_STANDARD.encode(bytes);
      negotiate!(
        out,
        sub,
        GMCP,
        format!("map {:?}", encoded).as_bytes().into()
      );
    })
    .detach();
}

fn is_visible(xform: &GlobalTransform, coord: Hex, radius: u32) -> bool {
  let layout = HexLayout::default();
  let diff = coord - xform.coords;
  let facing = xform.facing;

  // orient to flat north
  let rotated = diff.rotate_ccw(2 + facing.index() as u32);

  let Vec2 { x, y } = layout.hex_to_world_pos(rotated);

  let angle = x.atan2(y);

  let dist = xform.coords.distance_to(coord);

  if angle.abs() > std::f32::consts::FRAC_PI_2 {
    dist < (radius / 4) as _
  } else {
    dist < radius as _
  }
}
