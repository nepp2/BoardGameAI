
use crate::checkers::{
  Checkers,
  Tile,
  Player,
  Piece,
};

use Tile::*;
use Player::*;
use Piece::*;

// TODO: Exercise to create a score function for checkers

fn piece_value(piece : Piece) -> i64 {
  match piece {
    Pawn => 1,
    King => 2,
  }
}

fn piece_count(checkers : &Checkers) -> (i32, i32) {
  let mut white = 0;
  let mut black = 0;
  for tile in checkers.tiles.iter() {
    match tile {
      Occupied(Black, Pawn) => black += 1,
      Occupied(Black, King) => black += 2,
      Occupied(White, Pawn) => white += 1,
      Occupied(White, King) => white += 2,
      Tile::Empty => (),
    }
  }
  (white, black)
}

fn player_score(checkers : &Checkers, player : Player) -> f64 {
  let (white, black) = piece_count(checkers);
  match player {
    White => (white - black) as f64,
    Black => (black - white) as f64,
  }
}

// TODO: Exercise to complete chess implementation