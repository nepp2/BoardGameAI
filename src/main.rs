
use std::ops::Add;
use std::fmt;
use rand::{Rng, SeedableRng, rngs::StdRng};

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

/// Holds the contents of a tile
#[derive(Copy, Clone, PartialEq)]
enum Tile {
  White,
  Black,
  WhiteKing,
  BlackKing,
  Empty,
}

/// The two competing players
#[derive(Copy, Clone, PartialEq)]
enum Player {
  WhitePlayer,
  BlackPlayer,
}

/// The mode that the game is in.
#[derive(Copy, Clone)]
enum Mode {
  /// Active player can choose any move
  StartOfTurn,

  /// This exists to cope with the fact that players must
  /// sometimes take multiple actions in one turn; if they
  /// capture a piece, they must continue using the same
  /// piece to capture any other pieces available.
  Chain(Pos),

  /// One of the players won
  Victory(Player),
}

use Tile::*;
use Player::*;

impl Tile {
  /// Indicates which player the tile belongs to (if either)
  fn player(self) -> Option<Player> {
    match self {
      White | WhiteKing => Some(WhitePlayer),
      Black | BlackKing => Some(BlackPlayer),
      Empty => None,
    }
  }
}

/// The size of the board (which is assumed to be square)
const BOARD_SIZE : i32 = 8;

/// Stores the tiles. This is separated out to add convenient
/// access methods, and so it can be optimised later.
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
      Some(self.get(p))
    }
  }

  fn get(&self, p : Pos) -> Tile {
    self.tiles[self.index(p.x, p.y)]
  }

  fn set(&mut self, p : Pos, t : Tile) {
    self.tiles[self.index(p.x, p.y)] = t;
  }
}

#[derive(Debug, Copy, Clone)]
enum Action {
  Step { from : Pos, to : Pos },
  Jump { from : Pos, capture : Pos, to : Pos },
}

#[derive(Clone)]
struct GameState {
  board : Board,
  active_player : Player,
  mode : Mode,
}

impl GameState {

  fn new() -> GameState {
    let mut board = Board::new();
    for y in 0..3 {
      for i in (0..BOARD_SIZE).step_by(2) {
        let x = i + (y % 2);
        board.set(Pos {x, y}, White);
      }
    }
    for y in (BOARD_SIZE-3)..BOARD_SIZE {
      for i in (0..BOARD_SIZE).step_by(2) {
        let x = i + (y % 2);
        board.set(Pos{x, y}, Black);
      }
    }
    GameState { board, active_player : WhitePlayer, mode: Mode::StartOfTurn }
  }

  fn moves_from_pos(&self, start : Pos, steps : &mut Vec<Action>, jumps : &mut Vec<Action>) {
    let start_tile = self.board.get(start);
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
              let jump = pos + *m;
              if let Some(Tile::Empty) = self.board.try_get(jump) {
                jumps.push(Action::Jump{ from: start, capture: pos, to: jump});
              }
            }
          }
          None => steps.push(Action::Step{ from: start, to: pos}),
        }
      }
    }
  }

  fn possible_actions(&self) -> Vec<Action> {
    let mut steps = vec![];
    let mut jumps = vec![];
    match self.mode {
      Mode::StartOfTurn => {
        for (i, p) in self.board.tiles.iter().enumerate() {
          if p.player() == Some(self.active_player) {
            let x = (i as i32) % BOARD_SIZE;
            let y = (i as i32) / BOARD_SIZE;
            let p = Pos{ x, y };
            self.moves_from_pos(p, &mut steps, &mut jumps);
          }
        }
      }
      Mode::Chain(p) => {
        self.moves_from_pos(p, &mut steps, &mut jumps);
      }
      Mode::Victory(_) => (),
    }
    if jumps.len() > 0 {
      jumps
    }
    else {
      steps
    }
  }

  fn chain_check(&mut self, p : Pos) {
    let mut steps = vec!();
    let mut jumps = vec!();
    self.moves_from_pos(p, &mut steps, &mut jumps);
    if jumps.len() > 0 {
      self.mode = Mode::Chain(p);
    }
    else {
      self.active_player_swap();
      self.mode = Mode::StartOfTurn;
    }
  }

  fn king_check(&mut self, p : Pos) {
    let tile_value = self.board.get(p);
    match tile_value {
      Tile::Black => {
        if p.y == 0 {
          self.board.set(p, Tile::BlackKing);
        }
      }
      Tile::White => {
        if p.y == BOARD_SIZE-1 {
          self.board.set(p, Tile::WhiteKing);
        }
      }
      _ => (),
    }
  }

  fn victory_check(&self) -> bool {
    let (white, black) = self.piece_count();
    white == 0 || black == 0
  }

  fn piece_count(&self) -> (i32, i32) {
    let mut white = 0;
    let mut black = 0;
    for tile in self.board.tiles.iter() {
      match tile {
        Tile::Black => black += 1,
        Tile::BlackKing => black += 2,
        Tile::White => white += 1,
        Tile::WhiteKing => white += 2,
        Tile::Empty => (),
      }
    }
    (white, black)
  }

  fn active_player_swap(&mut self) {
    match self.active_player {
      Player::WhitePlayer => self.active_player = Player::BlackPlayer,
      Player::BlackPlayer => self.active_player = Player::WhitePlayer,
    }
  }

  fn apply_action(&mut self, a : Action) {
    match a {
      Action::Step { from, to } => {
        let tile_value = self.board.get(from);
        self.board.set(from, Tile::Empty);
        self.board.set(to, tile_value);
        self.king_check(to);
        self.active_player_swap();
        self.mode = Mode::StartOfTurn;
      }
      Action::Jump { from, capture, to } => {
        let tile_value = self.board.get(from);
        self.board.set(from, Tile::Empty);
        self.board.set(capture, Tile::Empty);
        self.board.set(to, tile_value);
        self.king_check(to);
        self.chain_check(to);
        if self.victory_check() {
          self.mode = Mode::Victory(tile_value.player().unwrap());
        }
      }
    }
  }
}

impl fmt::Display for GameState {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    for y in 0..BOARD_SIZE {
      write!(f, "|")?;
      for x in 0..BOARD_SIZE {
        let s = match self.board.get(Pos{x, y}) {
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

use piston_window::*;

fn rollout(game : &mut GameState, rng : &mut StdRng, max_depth : i32) -> f64 {
  for _ in 0..max_depth {
    let actions = game.possible_actions();
    if actions.len() > 0 {
      let i: usize = rng.gen_range(0, actions.len());
      game.apply_action(actions[i]);
    }
    else {
      break;
    }
  }
  let (white, black) = game.piece_count();
  (white - black) as f64
}

fn choose_white_action(game : &GameState, rng : &mut StdRng) -> Option<Action> {
  let actions = game.possible_actions();
  let mut best_score = 0.0;
  let mut best_action = None;
  for a in actions {
    let mut score = 0.0;
    for _ in 0..10 {
      let mut g = game.clone();
      g.apply_action(a);
      score += rollout(&mut g, rng, 10);
    }
    if score > best_score {
      best_score = score;
      best_action = Some(a);
    }
  }
  best_action
}

fn main() {

  println!("Checkers!");
  let mut game = GameState::new();
  let mut rng = StdRng::from_entropy(); //StdRng::seed_from_u64(0);

  let mut window: PistonWindow =
    WindowSettings::new("Checkers", [480, 480])
    .exit_on_esc(true).build().unwrap();
  while let Some(event) = window.next() {
    if let Some(Button::Keyboard(key)) = event.press_args() {
      if key == Key::Space {
        let a =
          if game.active_player == WhitePlayer {
            choose_white_action(&mut game, &mut rng)
          }
          else {
            let actions = game.possible_actions();
            if actions.len() > 0 {
              let i: usize = rng.gen_range(0, actions.len());
              Some(actions[i])
            }
            else {
              None
            }
          };
        if let Some(a) = a {
          game.apply_action(a);
        }
      }
    }
    window.draw_2d(&event, |context, graphics, _device| {
      clear([1.0; 4], graphics);
      for y in 0..BOARD_SIZE {
        for i in (0..BOARD_SIZE).step_by(2) {
          let x = i + (y % 2);
          rectangle(
            [0.0, 0.0, 0.0, 1.0], // black
            [x as f64 * 60.0, y as f64 * 60.0, 60.0, 60.0],
            context.transform,
            graphics);

          let tile = game.board.get(Pos{x, y});
          let colour = match tile {
            Tile::Black => Some([1.0, 0.0, 0.0, 1.0]),
            Tile::White => Some([0.0, 1.0, 0.0, 1.0]),
            Tile::BlackKing => Some([0.5, 0.0, 0.0, 1.0]),
            Tile::WhiteKing => Some([0.0, 0.5, 0.0, 1.0]),
            Empty => None,
          };
          if let Some(c) = colour {
            ellipse(
              c, [x as f64 * 60.0 + 5.0, y as f64 * 60.0 + 5.0, 50.0, 50.0],
              context.transform,
              graphics);
          }
        }
      }
    });
  }
}

