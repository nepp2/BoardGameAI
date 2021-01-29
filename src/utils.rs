
use std::ops::{Add, AddAssign, Neg};

/// an integer (X, Y) coordinate
#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Pos {
  pub x: i32,
  pub y: i32,
}

impl Add for Pos {
  type Output = Pos;
  fn add(self, other: Pos) -> Pos {
    Pos { x: self.x + other.x, y: self.y + other.y }
  }
}

impl Neg for Pos {
  type Output = Pos;
  fn neg(self) -> Pos {
    Pos { x: -self.x, y: -self.y }
  }
}

impl AddAssign for Pos {
  fn add_assign(&mut self, other: Pos) {
    *self = *self + other;
  }
}

/// Stores the tiles.
#[derive(Clone)]
pub struct Board<Tile : Clone> {
  pub size : i32,
  tiles : Vec<Tile>,
}

impl <Tile : Clone> Board<Tile> {
  pub fn new(initial_val : Tile, size : i32) -> Board<Tile> {
    Board { tiles : vec![initial_val ; (size * size) as usize], size }
  }

  pub fn iter(&self) -> impl Iterator<Item=&Tile> {
    self.tiles.iter()
  }

  pub fn index(&self, x : i32, y : i32) -> usize {
    (y * self.size + x) as usize
  }

  pub fn try_get(&self, p : Pos) -> Option<Tile> {
    if p.x < 0 || p.x >= self.size || p.y < 0 || p.y >= self.size {
      None
    }
    else {
      Some(self.get(p))
    }
  }

  pub fn get(&self, p : Pos) -> Tile {
    self.tiles[self.index(p.x, p.y)].clone()
  }

  pub fn set(&mut self, p : Pos, t : Tile) {
      let i = self.index(p.x, p.y);
    self.tiles[i] = t;
  }
}
