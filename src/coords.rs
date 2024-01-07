use std::{
  iter,
  ops::{
    Add,
    AddAssign,
    Mul,
    Sub,
  },
};

use bevy::reflect::Reflect;

#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash, Default, Reflect)]
#[reflect(Hash)]
pub struct Cubic(pub i64, pub i64, pub i64);

impl Sub for Cubic {
  type Output = Cubic;
  fn sub(self, rhs: Self) -> Self::Output {
    Cubic(self.0 - rhs.0, self.1 - rhs.1, self.2 - rhs.2)
  }
}

impl Add for Cubic {
  type Output = Cubic;
  fn add(self, rhs: Self) -> Self::Output {
    Cubic(self.0 + rhs.0, self.1 + rhs.1, self.2 + rhs.2)
  }
}

impl AddAssign for Cubic {
  fn add_assign(&mut self, rhs: Self) {
    *self = *self + rhs;
  }
}

pub const DIRECTIONS: [Cubic; 6] = [
  Cubic(1, 0, -1),
  Cubic(1, -1, 0),
  Cubic(0, -1, 1),
  Cubic(-1, 0, 1),
  Cubic(-1, 1, 0),
  Cubic(0, 1, -1),
];

impl Cubic {
  pub fn rotate_left(self) -> Cubic {
    Cubic(self.2, self.0, self.1) * -1
  }
  pub fn rotate_right(self) -> Cubic {
    Cubic(self.1, self.2, self.0) * -1
  }
  pub fn rotate(mut self, mut d: i8) -> Cubic {
    let rotate_fn = if d < 0 {
      d *= -1;
      Cubic::rotate_left
    } else {
      Cubic::rotate_right
    };

    for _ in 0..d {
      self = rotate_fn(self);
    }

    self
  }
  pub fn distance(self, other: Cubic) -> i64 {
    (self.0 - other.0)
      .abs()
      .max((self.1 - other.1).abs())
      .max((self.2 - other.2).abs())
  }
  pub fn direction(self) -> f32 {
    let (x, y) = self.square();
    x.atan2(y)
  }
  pub fn neighbor(self, i: u8) -> Option<Cubic> {
    DIRECTIONS.get(i as usize).copied().map(move |d| d + self)
  }
  pub fn neighbors(self) -> impl Iterator<Item = Cubic> {
    DIRECTIONS.iter().copied().map(move |d| d + self)
  }

  pub fn ring(self, radius: u64) -> impl Iterator<Item = Cubic> {
    let radius = radius as i64;

    let mut start = Some(self + DIRECTIONS[4] * radius);

    let mut i = 0;
    let mut j = 0;

    iter::from_fn(move || {
      if i >= 6 || (i > 0 && radius == 0) {
        return None;
      }

      let ret = start;
      start = start.and_then(|s| s.neighbor(i));

      j += 1;
      if j >= radius {
        j = 0;
        i += 1;
      }

      ret
    })
  }
  pub fn spiral(self, radius: u64) -> impl Iterator<Item = Cubic> {
    (0..=radius).flat_map(move |r| self.ring(r))
  }

  fn square(self) -> (f32, f32) {
    let x = 3. / 2. * (self.0 as f32);
    let y = (3f32).sqrt() / 2. * (self.0 as f32) + (3f32).sqrt() * (self.1 as f32);
    (x, y)
  }
}

impl Mul<i64> for Cubic {
  type Output = Cubic;
  fn mul(self, rhs: i64) -> Self::Output {
    Cubic(self.0 * rhs, self.1 * rhs, self.2 * rhs)
  }
}

#[cfg(test)]
mod test {
  use super::*;

  #[test]
  fn test_direction() {
    for (i, d) in DIRECTIONS.iter().map(|d| d.direction()).enumerate() {
      println!("{}: {}", i, d);
    }
    // panic!()
  }

  #[test]
  fn test_ring() {
    let start = Cubic(0, 0, 0);

    let ring = start.ring(0).collect::<Vec<_>>();

    assert_eq!(ring, vec![Cubic(0, 0, 0)]);

    let ring = start.ring(1).collect::<Vec<_>>();

    assert_eq!(
      ring,
      vec![
        Cubic(-1, 1, 0),
        Cubic(0, 1, -1),
        Cubic(1, 0, -1),
        Cubic(1, -1, 0),
        Cubic(0, -1, 1),
        Cubic(-1, 0, 1)
      ]
    );

    let ring = start.ring(2).collect::<Vec<_>>();

    assert_eq!(
      ring,
      vec![
        Cubic(-2, 2, 0),
        Cubic(-1, 2, -1),
        Cubic(0, 2, -2),
        Cubic(1, 1, -2),
        Cubic(2, 0, -2),
        Cubic(2, -1, -1),
        Cubic(2, -2, 0),
        Cubic(1, -2, 1),
        Cubic(0, -2, 2),
        Cubic(-1, -1, 2),
        Cubic(-2, 0, 2),
        Cubic(-2, 1, 1)
      ],
    );
  }

  fn spiral_area(radius: usize) -> usize {
    1 + 3 * radius * (radius + 1)
  }

  #[test]
  fn test_spiral() {
    let center = Cubic::default();
    for radius in 0..10usize {
      let spiral = center.spiral(radius as _).collect::<Vec<_>>();

      assert_eq!(spiral_area(radius), spiral.len());
    }
  }
}
