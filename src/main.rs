
use std::ops::Add;
use std::fmt;

#[derive(Copy, Clone, Debug)]
struct Pos {
  x: i32,
  y: i32,
}

impl Add for Pos {
  type Output = Pos;

  fn add(self, other: Pos) -> Pos {
    Pos { x: self.x + other.x, y: self.y + other.y }
  }
}

#[derive(Copy, Clone, PartialEq)]
enum Tile {
  White,
  Black,
  WhiteKing,
  BlackKing,
  Empty,
}

#[derive(Copy, Clone, PartialEq)]
enum Player {
  WhitePlayer,
  BlackPlayer,
}

use Tile::*;
use Player::*;

impl Tile {
  fn player(self) -> Option<Player> {
    match self {
      White | WhiteKing => Some(WhitePlayer),
      Black | BlackKing => Some(BlackPlayer),
      Empty => None,
    }
  }
}

const BOARD_SIZE : i32 = 8;

#[derive(Clone)]
struct Board {
  tiles : [Tile ; (BOARD_SIZE * BOARD_SIZE) as usize]
}

impl Board {
  fn new() -> Board {
    Board { tiles : [Empty ; (BOARD_SIZE * BOARD_SIZE) as usize] }
  }

  fn index(&self, x : i32, y : i32) -> usize {
    (y * BOARD_SIZE + x) as usize
  }

  fn try_get(&self, p : Pos) -> Option<Tile> {
    if p.x < 0 || p.x >= BOARD_SIZE || p.y < 0 || p.y >= BOARD_SIZE {
      None
    }
    else {
      Some(self.get(p.x, p.y))
    }
  }

  fn get(&self, x : i32, y : i32) -> Tile {
    self.tiles[self.index(x, y)]
  }

  fn get_mut(&mut self, x : i32, y : i32) -> &mut Tile {
    &mut self.tiles[self.index(x, y)]
  }
}

#[derive(Debug)]
enum Move {
  Simple { from : Pos, to : Pos },
  Capture { from : Pos, capture : Pos, then : Box<Move> },
}

#[derive(Clone)]
struct GameState {
  board : Board,
  turn : Player,
}

impl GameState {

  fn new() -> GameState {
    let mut board = Board::new();
    for y in 0..3 {
      for i in (0..BOARD_SIZE).step_by(2) {
        let x = i + (y % 2);
        *board.get_mut(x, y) = White;
      }
    }
    for y in (BOARD_SIZE-3)..BOARD_SIZE {
      for i in (0..BOARD_SIZE).step_by(2) {
        let x = i + (y % 2);
        *board.get_mut(x, y) = Black;
      }
    }
    GameState { board, turn : WhitePlayer }
  }

  fn moves_from_pos(&self, start : Pos, moves : &mut Vec<Move>) {
    let start_tile = self.board.get(start.x, start.y);
    let possible_moves : &[Pos] = match start_tile {
      White => &[Pos {x: -1, y: 1}, Pos {x: 1, y: 1}],
      Black => &[Pos {x: -1, y: -1}, Pos {x: 1, y: -1}],
      WhiteKing | BlackKing =>
        &[Pos {x: -1, y: 1}, Pos {x: 1, y: 1},
          Pos {x: -1, y: -1}, Pos {x: 1, y: -1}],
      Empty => &[],
    };
    for m in possible_moves {
      let pos = start + *m;
      if let Some(t) = self.board.try_get(pos) {
        match t.player() {
          Some(p) => {
            if p != start_tile.player().unwrap() {
              let hop = pos + *m;
              if let Some(Tile::Empty) = self.board.try_get(hop) {

              }
            }
          }
          None => moves.push(Move::Simple{ from: start, to: pos}),
        }
      }
    }
  }

  fn possible_actions(&self) -> Vec<Move> {
    let mut moves = vec![];
    for (i, p) in self.board.tiles.iter().enumerate() {
      if p.player() == Some(self.turn) {
        let x = (i as i32) % BOARD_SIZE;
        let y = (i as i32) / BOARD_SIZE;
        let p = Pos{ x, y };
        self.moves_from_pos(p, &mut moves);
      }
    }
    moves
  }
}

impl fmt::Display for GameState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for y in 0..BOARD_SIZE {
      write!(f, "|")?;
      for x in 0..BOARD_SIZE {
        let s = match self.board.get(x, y) {
          Black => "b", BlackKing => "B",
          White => "w", WhiteKing => "W",
          Empty => " ",
        };
        write!(f, " {}", s)?;
      }
      writeln!(f, " |")?;
    }
    Ok(())
  }
}

fn main() {
  println!("Checkers!");
  let mut game = GameState::new();
  for a in game.possible_actions() {
    if let Move::Simple{ from, to } = a {
      *game.board.get_mut(from.x, from.y) = WhiteKing;
      *game.board.get_mut(to.x, to.y) = BlackKing;
    }
  }
  println!("{}", game);
}
