use std::{
  cmp::Ordering,
  collections::{
    HashMap,
    HashSet,
  },
  f32::consts::SQRT_2,
};

use bevy::prelude::*;
use bitflags::bitflags;
use hexx::{
  hex,
  EdgeDirection,
  Hex,
  HexLayout,
};
use ratatui::{
  buffer::Cell,
  prelude::{
    Color as TuiColor,
    Rect,
    Style as TuiStyle,
    *,
  },
  widgets::Widget,
};
use serde::{
  Deserialize,
  Serialize,
};

pub const EDGES: [(&str, Option<&str>); 8] = [
  (".", Some(")")),
  ("-", None),
  (".", Some("(")),
  ("(", None),
  (")", None),
  ("'", Some(")")),
  ("-", None),
  ("'", Some("(")),
];

pub const EDGE_OFFSETS: [(i32, i32); 8] = [
  (-1, -1),
  (0, -1),
  (1, -1),
  (-2, 0),
  (2, 0),
  (-1, 1),
  (0, 1),
  (1, 1),
];

pub const EDGE_NEIGHBORS: [&[Hex]; 8] = [
  &[
    EdgeDirection::FLAT_TOP_LEFT.into_hex(),
    EdgeDirection::FLAT_TOP.into_hex(),
  ],
  &[EdgeDirection::FLAT_TOP.into_hex()],
  &[
    EdgeDirection::FLAT_TOP.into_hex(),
    EdgeDirection::FLAT_TOP_RIGHT.into_hex(),
  ],
  &[
    EdgeDirection::FLAT_TOP_LEFT.into_hex(),
    EdgeDirection::FLAT_BOTTOM_LEFT.into_hex(),
  ],
  &[
    EdgeDirection::FLAT_TOP_RIGHT.into_hex(),
    EdgeDirection::FLAT_BOTTOM_RIGHT.into_hex(),
  ],
  &[
    EdgeDirection::FLAT_BOTTOM_LEFT.into_hex(),
    EdgeDirection::FLAT_BOTTOM.into_hex(),
  ],
  &[EdgeDirection::FLAT_BOTTOM.into_hex()],
  &[
    EdgeDirection::FLAT_BOTTOM.into_hex(),
    EdgeDirection::FLAT_BOTTOM_RIGHT.into_hex(),
  ],
];

#[derive(
  Reflect, Default, Clone, Copy, Eq, PartialEq, Hash, Serialize, Deserialize, Deref, Debug,
)]
#[reflect(Hash, Debug)]
pub struct Modifier(u16);

bitflags! {
  impl Modifier: u16 {
      const BOLD              = 0b0000_0000_0001;
      const DIM               = 0b0000_0000_0010;
      const ITALIC            = 0b0000_0000_0100;
      const UNDERLINED        = 0b0000_0000_1000;
      const SLOW_BLINK        = 0b0000_0001_0000;
      const RAPID_BLINK       = 0b0000_0010_0000;
      const REVERSED          = 0b0000_0100_0000;
      const HIDDEN            = 0b0000_1000_0000;
      const CROSSED_OUT       = 0b0001_0000_0000;
  }
}

#[derive(Copy, Clone, Reflect, Default, Eq, PartialEq, Hash, Debug)]
#[reflect(Debug, Hash)]
pub enum Color {
  #[default]
  Reset,
  Black,
  Red,
  Green,
  Yellow,
  Blue,
  Magenta,
  Cyan,
  Gray,
  DarkGray,
  LightRed,
  LightGreen,
  LightYellow,
  LightBlue,
  LightMagenta,
  LightCyan,
  White,
  Rgb(u8, u8, u8),
  Indexed(u8),
}

impl From<Color> for TuiColor {
  fn from(color: Color) -> TuiColor {
    match color {
      Color::Reset => TuiColor::Reset,
      Color::Black => TuiColor::Black,
      Color::Red => TuiColor::Red,
      Color::Green => TuiColor::Green,
      Color::Yellow => TuiColor::Yellow,
      Color::Blue => TuiColor::Blue,
      Color::Magenta => TuiColor::Magenta,
      Color::Cyan => TuiColor::Cyan,
      Color::Gray => TuiColor::Gray,
      Color::DarkGray => TuiColor::DarkGray,
      Color::LightRed => TuiColor::LightRed,
      Color::LightGreen => TuiColor::LightGreen,
      Color::LightYellow => TuiColor::LightYellow,
      Color::LightBlue => TuiColor::LightBlue,
      Color::LightMagenta => TuiColor::LightMagenta,
      Color::LightCyan => TuiColor::LightCyan,
      Color::White => TuiColor::White,
      Color::Rgb(r, g, b) => TuiColor::Rgb(r, g, b),
      Color::Indexed(i) => TuiColor::Indexed(i),
    }
  }
}

#[derive(Copy, Clone, Reflect, Default, Eq, PartialEq, Hash, Debug)]
#[reflect(Debug, Hash)]
pub struct Style {
  pub fg: Option<Color>,
  pub bg: Option<Color>,
  pub add_modifier: Modifier,
  pub sub_modifier: Modifier,
}

impl From<Style> for TuiStyle {
  fn from(value: Style) -> Self {
    let mut style = TuiStyle::default();
    if let Some(fg) = value.fg {
      style.fg = Some(fg.into());
    }
    if let Some(bg) = value.bg {
      style.bg = Some(bg.into());
    }
    style.add_modifier =
      ratatui::style::Modifier::from_bits(*value.add_modifier).unwrap_or_default();
    style.sub_modifier =
      ratatui::style::Modifier::from_bits(*value.sub_modifier).unwrap_or_default();

    style
  }
}

#[derive(Clone, Debug, Default)]
pub struct Tile {
  // Edge cells with weights
  // These will necessarily be shared by one or two other tiles, and the final
  // state will be determined based on relative weights.
  // Useful for representing edges that are un-traversable.
  edges: [Option<(Cell, f32)>; 8],
  // The background of this tile, with weight.
  // The weight determines how "full" to make a tile, as well as "smudging"
  // between adjacent tiles if solid edges are disabled.
  background: Option<(Cell, f32)>,
  // The center of the tile. Usually the tile's primary occupant.
  center: Option<Cell>,
}

impl Tile {
  pub fn edge(&mut self, id: usize) -> WeightedCellBuilder {
    WeightedCellBuilder {
      cell: &mut self.edges[id],
    }
  }

  pub fn background(&mut self) -> WeightedCellBuilder {
    WeightedCellBuilder {
      cell: &mut self.background,
    }
  }

  pub fn center(&mut self) -> CellBuilder {
    CellBuilder {
      cell: &mut self.center,
    }
  }
}
pub struct CellBuilder<'a> {
  cell: &'a mut Option<Cell>,
}

impl<'a> CellBuilder<'a> {
  pub fn clear(&mut self) {
    self.cell.take();
  }

  pub fn style(&mut self, style: TuiStyle) -> &mut Self {
    self
      .cell
      .get_or_insert_with(Default::default)
      .set_style(style);
    self
  }
  pub fn symbol(&mut self, symbol: &str) -> &mut Self {
    self
      .cell
      .get_or_insert_with(Default::default)
      .set_symbol(symbol);
    self
  }
}

pub struct WeightedCellBuilder<'a> {
  cell: &'a mut Option<(Cell, f32)>,
}

impl<'a> WeightedCellBuilder<'a> {
  pub fn clear(&mut self) {
    self.cell.take();
  }

  pub fn weight(&mut self, weight: f32) -> &mut Self {
    self.cell.get_or_insert_with(|| (Cell::default(), 1.0)).1 = weight;
    self
  }

  pub fn style(&mut self, style: TuiStyle) -> &mut Self {
    self
      .cell
      .get_or_insert_with(|| (Cell::default(), 1.0))
      .0
      .set_style(style);
    self
  }
  pub fn symbol(&mut self, symbol: &str) -> &mut Self {
    self
      .cell
      .get_or_insert_with(|| (Cell::default(), 1.0))
      .0
      .set_symbol(symbol);
    self
  }
}

impl Tile {
  pub fn with_character(c: char) -> Tile {
    let mut cell = Cell::default();
    cell.set_char(c);
    Tile {
      center: Some(cell),
      ..Default::default()
    }
  }
}

#[derive(Clone, Debug, Reflect)]
#[reflect(from_reflect = false)]
pub struct HexMap {
  #[reflect(ignore)]
  tiles: HashMap<Hex, Tile>,
  center: Hex,
  up_direction: EdgeDirection,
  radius: u8,
  render_edges: bool,
  base_layout: HexLayout,
}

impl Default for HexMap {
  fn default() -> Self {
    const SQRT_3: f32 = 1.732_050_8;
    HexMap {
      tiles: HashMap::default(),
      center: hex(0, 0),
      up_direction: EdgeDirection::FLAT_NORTH,
      radius: 20,
      render_edges: true,
      base_layout: HexLayout {
        orientation: hexx::HexOrientation::Flat,
        hex_size: Vec2 {
          y: 2. / SQRT_3,
          x: 2.,
        },
        invert_y: true,
        ..Default::default()
      },
    }
  }
}

impl HexMap {
  /// Prune the stored tiles based on the center and radius
  /// O(mapsize) operation, only really needs to be called when the center
  /// moves.
  pub fn prune(&mut self) {
    let mut out = vec![];
    for c in self.tiles.keys().copied() {
      if self.center.distance_to(c) > 2 * self.radius as i32 {
        out.push(c);
      }
    }
    for c in out {
      self.tiles.remove(&c);
    }
  }

  pub fn clear(&mut self) {
    self.tiles.clear();
  }

  pub fn edges(&mut self, render: bool) -> &mut Self {
    self.render_edges = render;
    self
  }

  pub fn tile(&mut self, coords: Hex) -> &mut Tile {
    self.tiles.entry(coords).or_default()
  }

  // Insert a tile.
  pub fn insert(&mut self, coords: Hex, tile: Tile) {
    if self.center.distance_to(coords) > 2 * self.radius as i32 {
      return;
    }
    self.tiles.insert(coords, tile);
  }

  pub fn up_direction(&mut self, up_direction: EdgeDirection) -> &mut Self {
    self.up_direction = up_direction;
    self
  }

  pub fn center(&mut self, coords: Hex) -> &mut Self {
    self.center = coords;
    self
  }

  pub fn radius(&mut self, radius: u8) -> &mut Self {
    self.radius = radius;
    self
  }

  fn center_coords(&self, rect: Rect) -> (u16, u16) {
    let off_h = rect.height / 2;
    let off_w = rect.width / 2;
    (rect.x + off_w, rect.y + off_h)
  }

  fn rect_coords(&self, rect: Rect, coords: Hex) -> Option<(u16, u16)> {
    if self.center.unsigned_distance_to(coords) >= self.radius as _ {
      return None;
    }
    let (x, y) = self.center_coords(rect);

    let layout = HexLayout {
      origin: Vec2 {
        x: x as _,
        y: y as _,
      },
      ..self.base_layout
    };
    let coords = (coords - self.center).rotate_ccw(2 + self.up_direction.index() as u32);
    let pos = layout.hex_to_world_pos(coords);

    visible(rect, pos.x.round() as _, pos.y.round() as _)
  }
}

fn visible(rect: Rect, x: i32, y: i32) -> Option<(u16, u16)> {
  if x < 0 || y < 0 || x >= (rect.x + rect.width) as i32 || y >= (rect.y + rect.height) as i32 {
    None
  } else {
    Some((x as _, y as _))
  }
}

impl<'a> Widget for &'a HexMap {
  fn render(self, area: Rect, buf: &mut Buffer) {
    let mut visited: HashSet<(u16, u16)> = Default::default();

    for ring in 0..self.radius {
      let mut any_visible = false;
      for hex in self.center.ring(ring as _) {
        let (x, y) = if let Some((x, y)) = self.rect_coords(area, hex) {
          any_visible = true;
          (x, y)
        } else {
          continue;
        };

        let tile = if let Some(t) = self.tiles.get(&hex) {
          t
        } else {
          continue;
        };

        if let Some(bg) = tile.background.as_ref() {
          let bg_pattern = 7;

          for bg_off in -1..=1i32 {
            let bit = (bg_pattern >> (bg_off + 1) as u8 & 0x01) == 1;
            if !bit {
              continue;
            }
            if let Some((x, y)) = visible(area, x as i32 + bg_off, y as i32) {
              buf.set_string(x, y, &bg.0.symbol, bg.0.style());
            }
          }
        }

        if let Some(cell) = tile.center.as_ref() {
          buf.set_string(x, y, &cell.symbol, cell.style());
        }

        for (i, (x_off, y_off)) in EDGE_OFFSETS.into_iter().enumerate() {
          let Some((x, y)) = visible(area, x as i32 + x_off, y as i32 + y_off) else {
            continue;
          };
          if !visited.contains(&(x, y)) {
            visited.insert((x, y));
            if self.render_edges {
              let has_neighbor = EDGE_NEIGHBORS[i]
                .iter()
                .map(|n| hex + n.rotate_cw(2 + self.up_direction.index() as u32))
                .any(|h| self.rect_coords(area, h).is_some() && self.tiles.contains_key(&h));
              let c = if has_neighbor {
                EDGES[i].1.unwrap_or(EDGES[i].0)
              } else {
                EDGES[i].0
              };
              buf.set_string(x, y, c, TuiStyle::reset().fg(TuiColor::DarkGray));
            } else {
              let mut neighbors = tile.background.iter().collect::<Vec<_>>();
              for bg in EDGE_NEIGHBORS[i]
                .iter()
                .filter_map(|n| self.tiles.get(&(hex + *n)))
                .filter_map(|t| t.background.as_ref())
              {
                neighbors.push(bg);
              }
              if neighbors.is_empty() {
                continue;
              }
              let sums =
                neighbors
                  .iter()
                  .fold(HashMap::<&Cell, f32>::new(), |mut acc, (cell, weight)| {
                    *acc.entry(cell).or_default() += weight;
                    acc
                  });
              let bg = sums
                .iter()
                .max_by(|a, b| a.1.partial_cmp(b.1).unwrap_or(Ordering::Equal))
                .unwrap()
                .0;
              buf.set_string(x, y, &bg.symbol, bg.style());
            }
          }
        }
      }
      if !any_visible {
        break;
      }
    }
  }
}
