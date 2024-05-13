//! # The Mud Map System
//!
//! Who knows if this is anywhere near correct/final.
//!
//! Maps are hierarchical. Map -> Tile -> Errthang else.

use std::io::Write;

use base64::Engine as _;
use bevy::{
  ecs::system::{
    CommandQueue,
    SystemParam,
  },
  prelude::*,
  reflect::GetTypeRegistration,
  utils::{
    hashbrown::HashSet,
    HashMap,
  },
};
use bevy_async_util::CommandsExt;
use bevy_sqlite::*;
use futures::TryStreamExt;
use hexx::{
  EdgeDirection,
  Hex,
  HexLayout,
};
use ratatui::{
  prelude::Rect,
  widgets::Widget,
};

use crate::{
  ascii_map::{
    render::Ansi,
    widget::{
      Color,
      HexMap,
      Modifier,
      Style,
      Tile as TuiTile,
    },
  },
  character::{
    Character,
    NonPlayer,
    Player,
  },
  net::{
    TelnetOut,
    GMCP,
  },
};

const MAP_RADIUS: u64 = 9;

pub struct MapPlugin;

impl Plugin for MapPlugin {
  fn build(&self, app: &mut App) {
    app.add_event::<RenderRequest>().add_event::<Moved>();
    app
      .persist_component::<Name>()
      .persist_component::<Map>()
      .persist_component::<MapWidget>()
      .persist_component::<Transform>()
      .persist_component::<Render>()
      .persist_component::<Tile>();

    app
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
        PreUpdate,
        (
          (track_maps_system, add_global),
          propagate_transforms,
          emit_change_events,
          render_map_system,
        )
          .chain()
          .after(DbSystem),
      )
      .add_systems(Startup, spawn_maps.after(DbSystem));
  }
}

#[derive(Resource, Default)]
pub struct Maps {
  pub by_name: HashMap<String, Entity>,
  pub by_id: HashMap<Entity, String>,
}

/// Quick reference for all of the entities that are on a particular map.
#[derive(Reflect, Component, Default, Eq, PartialEq)]
pub struct Entities {
  #[reflect(ignore)]
  pub by_coords: HashMap<Hex, Vec<Entity>>,
  #[reflect(ignore)]
  pub by_id: HashMap<Entity, Transform>,
}

impl Entities {
  pub fn iter_at(&self, coord: &Hex) -> impl Iterator<Item = Entity> + '_ {
    self
      .by_coords
      .get(coord)
      .into_iter()
      .flat_map(|entities| entities.iter().copied())
  }
}

#[derive(SystemParam)]
pub struct MapEntitiesMut<'w, 's> {
  pub maps: Res<'w, Maps>,
  pub tiles: Query<'w, 's, &'static mut Entities>,
}

#[derive(SystemParam)]
pub struct MapEntities<'w, 's> {
  pub maps: Res<'w, Maps>,
  pub tiles: Query<'w, 's, &'static Entities>,
}

impl<'w, 's> MapEntitiesMut<'w, 's> {
  pub fn by_name_mut(&mut self, name: &str) -> Option<(Entity, Mut<Entities>)> {
    let id = self.maps.by_name.get(name).copied()?;
    let tiles = self.tiles.get_mut(id).ok()?;
    Some((id, tiles))
  }
  pub fn by_name(&self, name: &str) -> Option<(Entity, &Entities)> {
    let id = self.maps.by_name.get(name).copied()?;
    let tiles = self.tiles.get(id).ok()?;
    Some((id, tiles))
  }
  pub fn by_id_mut(&mut self, id: Entity) -> Option<Mut<Entities>> {
    self.tiles.get_mut(id).ok()
  }
  pub fn by_id(&self, id: Entity) -> Option<&Entities> {
    self.tiles.get(id).ok()
  }
}

impl<'w, 's> MapEntities<'w, 's> {
  pub fn by_name(&self, name: &str) -> Option<(Entity, &Entities)> {
    let id = self.maps.by_name.get(name).copied()?;
    let tiles = self.tiles.get(id).ok()?;
    Some((id, tiles))
  }
  pub fn by_id(&self, id: Entity) -> Option<&Entities> {
    self.tiles.get(id).ok()
  }
}

/// Marker component for maps.
#[derive(Component, Reflect, Clone, Default, Deref, Hash, Eq, PartialEq)]
#[reflect(Component, Hash)]
pub struct Map(pub String);

#[derive(Component, Reflect, Clone, Default, Eq, PartialEq, Hash, Debug)]
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
#[derive(Component, Reflect, Copy, Clone, Default)]
#[reflect(Component)]
pub struct Tile;

#[derive(Reflect, Clone, Default, Eq, PartialEq, Hash, Debug)]
#[reflect(Hash, FromWorld)]
pub struct Symbol {
  pub text: String,
  pub style: Style,
}

#[derive(Component, Reflect, Clone, Default, Eq, PartialEq, Hash, Debug)]
#[reflect(Component, Hash, FromWorld)]
pub struct Render {
  pub icon: Option<Symbol>,
  pub background: Option<Symbol>,
}

#[derive(Component, Reflect, Default, Debug, Deref, DerefMut)]
#[reflect(Component)]
pub struct MapWidget(#[reflect(ignore)] HexMap);

fn spawn_maps(mut cmd: Commands, db: SaveDbOwned) {
  let conn = db.db.clone();
  cmd.spawn_callback(
    async move {
      let map = Map::get_type_registration().type_info().type_path();
      sqlx::query!(
        r#"
          select e.id
          from entity e
          where exists (
            select *
            from entity_component ec, component c
            where ec.entity = e.id
            and c.id = ec.component
            and c.name = ?
          )
        "#,
        map,
      )
      .fetch(&*conn)
      .map_ok(|res| res.id)
      .try_collect::<Vec<i64>>()
      .await
    },
    move |res, world| {
      load_entities(res, world);
      let conn = world.resource::<Db>().clone();
      let mut queue = CommandQueue::default();
      let mut cmds = Commands::new(&mut queue, world);
      cmds.spawn_callback(
        async move {
          let character = Character::get_type_registration().type_info().type_path();
          let map = Map::get_type_registration().type_info().type_path();
          let non_player = NonPlayer::get_type_registration().type_info().type_path();
          sqlx::query!(
            r#"
              select e.id
              from entity e
              where ((not exists (
                select *
                from entity_component ec, component c
                where ec.entity = e.id
                and c.id = ec.component
                and c.name = ?
              )) or exists (
                select *
                from entity_component ec, component c
                where ec.entity = e.id
                and c.id = ec.component
                and c.name = ?
              )) and (not exists (
                select *
                from entity_component ec, component c
                where ec.entity = e.id
                and c.id = ec.component
                and c.name = ?
              ))
            "#,
            character,
            non_player,
            map,
          )
          .fetch(&*conn)
          .map_ok(|res| res.id)
          .try_collect::<Vec<i64>>()
          .await
        },
        load_entities,
      );
      queue.apply(world);
    },
  );
}

fn load_entities(res: sqlx::Result<Vec<i64>>, world: &mut World) {
  let db: SaveDbOwned = SaveDb::from_world(world).into();
  match res {
    Ok(entities) => {
      debug!(?entities, "got {} entities to load", entities.len());
      let db = db.borrowed();

      for index in entities {
        let db_entity = DbEntity::from_index(index);
        let Some(world_entity) = db.map.read().world_entity(db_entity) else {
          warn!(index, "missing world entity for db entity");
          continue;
        };
        let Some(mut entity) = world.get_or_spawn(world_entity) else {
          warn!(?world_entity, "entity is no longer valid");
          continue;
        };
        let Some(task) = db.load_db_entity(db_entity) else {
          continue;
        };
        entity.insert(task);
      }
    }
    Err(error) => {
      warn!(%error, "error looking up entities to load");
    }
  }
}

fn track_maps_system(
  mut cmd: Commands,
  mut maps: ResMut<Maps>,
  map_added: Query<(Entity, &Map), Added<Map>>,
  mut removed: RemovedComponents<Map>,
) {
  for (ent, name) in map_added.iter() {
    maps.by_name.insert((**name).clone(), ent);
    maps.by_id.insert(ent, (**name).clone());
    cmd.entity(ent).insert(Entities::default());
  }

  for ent in removed.read() {
    if let Some(name) = maps.by_id.remove(&ent) {
      maps.by_name.remove(&name);
    }
  }
}

fn add_global(
  mut cmd: Commands,
  query: Query<Entity, (With<Transform>, Without<GlobalTransform>)>,
) {
  for entity in query.iter() {
    cmd.entity(entity).insert(GlobalTransform::default());
  }
}

fn propagate_transforms(
  mut writer: EventWriter<Moved>,
  mut map_ents: MapEntitiesMut,
  changed: Query<(Entity, Option<&Parent>), Or<(Added<Transform>, Changed<Transform>)>>,
  mut data_query: Query<(&Transform, &mut GlobalTransform)>,
  children: Query<&Children>,
) {
  for (ent, parent) in changed.iter() {
    let parent_xform = parent.and_then(|p| data_query.get(**p).map(|(_, g)| g.clone()).ok());
    propagate_transform(
      &mut writer,
      &mut map_ents,
      ent,
      parent_xform.as_ref(),
      &mut data_query,
      &children,
    );
  }
}

#[derive(Event)]
pub struct Moved {
  pub entity: Entity,
  pub prev: GlobalTransform,
  pub new: GlobalTransform,
}

fn propagate_transform(
  writer: &mut EventWriter<Moved>,
  map_ents: &mut MapEntitiesMut,
  entity: Entity,
  parent_xform: Option<&GlobalTransform>,
  data: &mut Query<(&Transform, &mut GlobalTransform)>,
  children: &Query<&Children>,
) {
  let my_xform = if let Ok((xform, mut global)) = data.get_mut(entity) {
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

    if prev.map != global.map {
      if let Some((_, mut ents)) = map_ents.by_name_mut(&prev.map) {
        if let Some(ent_vec) = ents
          .by_id
          .remove(&entity)
          .and_then(|prev| ents.by_coords.get_mut(&prev.coords))
        {
          ent_vec.retain(|e| *e != entity);
        }
      }
      if let Some((_, mut ents)) = map_ents.by_name_mut(&global.map) {
        ents.by_id.insert(entity, (**global).clone());
        ents
          .by_coords
          .entry(global.coords)
          .or_default()
          .push(entity);
      };
    } else if prev.coords != global.coords {
      if let Some((_, mut ents)) = map_ents.by_name_mut(&prev.map) {
        if let Some(ent_vec) = ents
          .by_id
          .get(&entity)
          .map(|t| t.coords)
          .and_then(|prev| ents.by_coords.get_mut(&prev))
        {
          ent_vec.retain(|e| *e != entity);
        }
      }
      if let Some((_, mut ents)) = map_ents.by_name_mut(&global.map) {
        ents.by_id.insert(entity, (**global).clone());
        ents
          .by_coords
          .entry(global.coords)
          .or_default()
          .push(entity);
      };
    }

    if prev != *global {
      writer.send(Moved {
        entity,
        prev,
        new: global.clone(),
      });
    }

    Some(global.clone())
  } else {
    None
  };

  for child in children.get(entity).iter().flat_map(|c| c.iter()) {
    propagate_transform(writer, map_ents, *child, my_xform.as_ref(), data, children);
  }
}

fn emit_change_events(
  map_entities: MapEntities,
  mut writer: EventWriter<RenderRequest>,
  mut moved: EventReader<Moved>,
  widgets: Query<(), With<MapWidget>>,
  rendered: Query<(), With<Render>>,
) {
  for Moved { entity, prev, new } in moved.read() {
    if widgets.contains(*entity) {
      writer.send(RenderRequest(*entity));
    }
    if rendered.contains(*entity) {
      for xform in [prev, new] {
        let (_, map) = try_opt!(map_entities.by_name(&xform.map), continue);
        for coords in xform.coords.spiral_range(0..(MAP_RADIUS as u32)) {
          for other in map
            .by_coords
            .get(&coords)
            .into_iter()
            .flat_map(|ents| ents.iter())
          {
            if widgets.contains(*other) {
              writer.send(RenderRequest(*other));
            }
          }
        }
      }
    }
  }
}

#[derive(Event, Debug, Deref)]
pub struct RenderRequest(pub Entity);

fn render_map_system(
  map_entities: MapEntities,
  mut puppet_query: Query<(Entity, &GlobalTransform, &Player, &mut MapWidget)>,
  player_query: Query<&TelnetOut, With<GMCP>>,
  render_query: Query<&Render>,
  mut render_requests: EventReader<RenderRequest>,
) {
  let to_render = render_requests.read().map(|e| e.0).collect::<HashSet<_>>();

  if bevy::tasks::ComputeTaskPool::get().thread_num() <= 1 {
    warn!(
      par = bevy::tasks::available_parallelism(),
      "only one thread??"
    );
  }

  puppet_query
    .par_iter_mut()
    .for_each(|(entity, xform, player, mut widget)| {
      if !to_render.contains(&entity) {
        return;
      }

      let out = try_opt!(player_query.get(player.0).ok(), return);
      let (_, map) = try_opt!(map_entities.by_name(&xform.map), return);

      let center = xform.coords + xform.facing.into_hex() * 3;

      widget.clear();
      widget.center(center);
      widget.up_direction(xform.facing);
      for coord in xform.coords.spiral_range(0..(MAP_RADIUS as u32)) {
        if !is_visible(xform, coord) && coord != center {
          continue;
        }
        let mut tile = TuiTile::default();
        for render in map.iter_at(&coord).filter_map(|e| render_query.get(e).ok()) {
          if let Some(bg) = render.background.as_ref() {
            tile.background().symbol(&bg.text).style(bg.style.into());
          }
          if let Some(fg) = render.icon.as_ref() {
            tile.center().symbol(&fg.text).style(fg.style.into());
          }
        }

        widget.insert(coord, tile);
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
    });
}

fn is_visible(xform: &GlobalTransform, coord: Hex) -> bool {
  let layout = HexLayout::default();
  let diff = coord - xform.coords;
  let facing = xform.facing;

  // orient to flat north
  let rotated = diff.rotate_ccw(2 + facing.index() as u32);

  let Vec2 { x, y } = layout.hex_to_world_pos(rotated);

  let angle = x.atan2(y);

  let dist = xform.coords.distance_to(coord);

  if angle.abs() > std::f32::consts::FRAC_PI_2 {
    dist <= (MAP_RADIUS / 4) as _
  } else {
    dist <= MAP_RADIUS as _
  }
}
