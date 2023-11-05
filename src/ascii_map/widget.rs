use std::{
  cmp::Ordering,
  collections::{
    HashMap,
    HashSet,
  },
};

use bevy::prelude::*;
use ratatui::{
  buffer::Cell,
  prelude::{
    Color,
    Rect,
    Style,
    *,
  },
  widgets::Widget,
};

use crate::coords::Cubic;

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

pub const EDGE_NEIGHBORS: [&[Cubic]; 8] = [
  &[Cubic(-1, 0, 1), Cubic(0, -1, 1)],
  &[Cubic(0, -1, 1)],
  &[Cubic(1, -1, 0), Cubic(0, -1, 1)],
  &[Cubic(-1, 0, 1), Cubic(-1, 1, 0)],
  &[Cubic(1, 0, -1), Cubic(1, -1, 0)],
  &[Cubic(0, 1, -1), Cubic(-1, 1, 0)],
  &[Cubic(0, 1, -1)],
  &[Cubic(0, 1, -1), Cubic(1, 0, -1)],
];

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

  pub fn style(&mut self, style: Style) -> &mut Self {
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

  pub fn style(&mut self, style: Style) -> &mut Self {
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
  tiles: HashMap<Cubic, Tile>,
  center: Cubic,
  radius: u8,
  render_edges: bool,
}

impl Default for HexMap {
  fn default() -> Self {
    HexMap {
      tiles: HashMap::default(),
      center: Cubic(0, 0, 0),
      radius: 20,
      render_edges: true,
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
      if self.center.distance(c) > 2 * self.radius as i64 {
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

  pub fn tile(&mut self, coords: Cubic) -> &mut Tile {
    self.tiles.entry(coords).or_default()
  }

  // Insert a tile.
  pub fn insert(&mut self, coords: Cubic, tile: Tile) {
    if self.center.distance(coords) > 2 * self.radius as i64 {
      return;
    }
    self.tiles.insert(coords, tile);
  }

  pub fn center(&mut self, coords: Cubic) -> &mut Self {
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

  fn rect_coords(&self, rect: Rect, coords: Cubic) -> Option<(u16, u16)> {
    if self.center.distance(coords) >= self.radius as _ {
      return None;
    }
    let coords = coords - self.center;
    let (x, y) = self.center_coords(rect);
    let mut x = x as i32;
    let mut y = y as i32;
    x += 3 * coords.0 as i32;
    y += 2 * coords.1 as i32 + coords.0 as i32;
    visible(rect, x, y)
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
          let (x, y) = if let Some((x, y)) = visible(area, x as i32 + x_off, y as i32 + y_off) {
            (x, y)
          } else {
            continue;
          };
          if !visited.contains(&(x, y)) {
            visited.insert((x, y));
            if self.render_edges {
              let has_neighbor = EDGE_NEIGHBORS[i]
                .iter()
                .map(|n| hex + *n)
                .any(|h| self.rect_coords(area, h).is_some() && self.tiles.contains_key(&h));
              let c = if has_neighbor {
                EDGES[i].1.unwrap_or(EDGES[i].0)
              } else {
                EDGES[i].0
              };
              buf.set_string(x, y, c, Style::reset().fg(Color::DarkGray));
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
