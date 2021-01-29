
use piston_window::*;
use rand::{SeedableRng, rngs::StdRng};

use crate::utils::Pos;

/// Represents the state of a checkers game
#[derive(Clone)]
pub struct Checkers {
  pub tiles : [Tile ; 64],
  pub active_player : Player,
  pub mode : Mode,
}

/// Holds the contents of a tile
#[derive(Copy, Clone, PartialEq)]
pub enum Tile {
  Occupied(Player, Piece),
  Empty,
}

/// Describes the type of a piece
#[derive(Copy, Clone, PartialEq)]
pub enum Piece {
  Pawn,
  King,
}

/// The two competing players
#[derive(Copy, Clone, PartialEq)]
pub enum Player {
  White,
  Black,
}

/// The mode that the game is in.
#[derive(Copy, Clone, PartialEq)]
pub enum Mode {
  /// The active player can choose any move
  StartOfTurn,

  /// The active player has captured a piece, but must
  /// continue capturing with the same piece until no
  /// more pieces can be captured.
  ChainCapture(Pos),

  /// One of the players won
  Victory(Player),
}

use Tile::*;
use Player::*;
use Piece::*;

impl Tile {
  /// Indicates which player the tile belongs to (if either)
  fn player(self) -> Option<Player> {
    if let Occupied(p, _) = self {
      Some(p)
    }
    else {
      None
    }
  }
}

/// The size of the board (which is assumed to be square)
const BOARD_SIZE : i32 = 8;

#[derive(Debug, Copy, Clone)]
pub enum Action {
  Step { from : Pos, to : Pos },
  Jump { from : Pos, capture : Pos, to : Pos },
}

fn possible_moves(tile : Tile) -> &'static [Pos] {
  match tile {
    Occupied(White, Pawn) => &[Pos {x: -1, y: 1}, Pos {x: 1, y: 1}],
    Occupied(Black, Pawn) => &[Pos {x: -1, y: -1}, Pos {x: 1, y: -1}],
    Occupied(_, King) =>
      &[Pos {x: -1, y: 1}, Pos {x: 1, y: 1},
        Pos {x: -1, y: -1}, Pos {x: 1, y: -1}],
    Empty => &[],
  }
}

fn coord_index(x : i32, y : i32) -> usize {
  (y * BOARD_SIZE + x) as usize
}

fn pos_index(p : Pos) -> usize {
  coord_index(p.x, p.y)
}

impl Checkers {

  pub fn new() -> Checkers {
    let mut board = [Tile::Empty ; 64];
    for y in 0..3 {
      for i in (0..BOARD_SIZE).step_by(2) {
        let x = i + (y % 2);
        board[coord_index(x, y)] = Occupied(White, Pawn);
      }
    }
    for y in (BOARD_SIZE-3)..BOARD_SIZE {
      for i in (0..BOARD_SIZE).step_by(2) {
        let x = i + (y % 2);
        board[coord_index(x, y)] = Occupied(Black, Pawn);
      }
    }
    Checkers { tiles: board, active_player : White, mode: Mode::StartOfTurn }
  }

  fn set_tile(&mut self, p : Pos, tile : Tile) {
    self.tiles[pos_index(p)] = tile;
  }

  fn get_tile(&self, p : Pos) -> Tile {
    self.tiles[pos_index(p)]
  }

  fn try_get_tile(&self, p : Pos) -> Option<Tile> {
    if p.x >= 0 && p.x < BOARD_SIZE && p.y >= 0 && p.y < BOARD_SIZE {
      Some(self.get_tile(p))
    }
    else { None }
  }

  fn visit_jumps_from_pos(&self, start : Pos, mut f : impl FnMut(Action)) {
    let start_tile = self.get_tile(start);
    let player = start_tile.player().unwrap();
    for m in possible_moves(start_tile) {
      let pos = start + *m;
      if let Some(p) = self.try_get_tile(pos).and_then(|t| t.player()) {
        if p != player {
          let jump = pos + *m;
          if let Some(Empty) = self.try_get_tile(jump) {
            let a = Action::Jump{ from: start, capture: pos, to: jump};
            f(a);
          }
        }
      }
    }
  }

  fn find_jumps_from_pos(&self, start : Pos, actions : &mut Vec<Action>) {
    self.visit_jumps_from_pos(start, |a| {
      actions.push(a);
    });
  }

  fn find_steps_from_pos(&self, start : Pos, actions : &mut Vec<Action>) {
    let start_tile = self.get_tile(start);
    for m in possible_moves(start_tile) {
      let pos = start + *m;
      if let Some(Empty) = self.try_get_tile(pos) {
        actions.push(Action::Step{ from: start, to: pos});
      }
    }
  }

  fn visit_player_pieces(&self, player : Player, mut f : impl FnMut(Pos)) {
    for (i, t) in self.tiles.iter().enumerate() {
      if t.player() == Some(player) {
        let p = Pos {
          x: (i as i32) % BOARD_SIZE,
          y: (i as i32) / BOARD_SIZE,
        };
        f(p);
      }
    }
  }

  /// Return true if the piece at pos can capture a piece
  /// in its next move
  fn can_capture_a_piece(&mut self, p : Pos) -> bool {
    let mut can_capture = false;
    self.visit_jumps_from_pos(p, |_a| {
      can_capture = true;
    });
    can_capture
  }

  /// Turn the tile at `pos` into a king if it is
  /// currently a normal piece, and just reached the
  /// final row at the other end of the board
  fn king_check(&mut self, p : Pos) {
    let tile_value = self.get_tile(p);
    match tile_value {
      Occupied(Black, Pawn) => {
        if p.y == 0 {
          self.set_tile(p, Occupied(Black, King));
        }
      }
      Occupied(White, Pawn) => {
        if p.y == BOARD_SIZE-1 {
          self.set_tile(p, Occupied(White, King));
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
    for tile in self.tiles.iter() {
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

  fn active_player_swap(&mut self) {
    let p = match self.active_player { White => Black, Black => White };
    self.active_player = p;
  }
}

// --------- Implement the generic boardgame trait to allow agents to play ---------

use crate::agents::{Game, GameAgent, agent_action};

impl Game for Checkers {
  type Action = Action;

  fn possible_actions(&self, actions : &mut Vec<Action>) {
    match self.mode {
      Mode::StartOfTurn => {
        let p = self.active_player;
        self.visit_player_pieces(p, |pos| {
          self.find_jumps_from_pos(pos, actions);
        });
        if actions.is_empty() {
          self.visit_player_pieces(p, |pos| {
            self.find_steps_from_pos(pos, actions);
          });
        }
      }
      Mode::ChainCapture(p) => {
        self.find_jumps_from_pos(p, actions);
      }
      Mode::Victory(_) => (),
    }
  }

  fn active_player(&self) -> i64 {
    match self.active_player {
      White => 0, Black => 1
    }
  }

  fn apply_action(&mut self, a : &Action) {
    match *a {
      Action::Step { from, to } => {
        let tile_value = self.get_tile(from);
        self.set_tile(from, Tile::Empty);
        self.set_tile(to, tile_value);
        self.king_check(to);
        self.active_player_swap();
        self.mode = Mode::StartOfTurn;
      }
      Action::Jump { from, capture, to } => {
        let tile_value = self.get_tile(from);
        self.set_tile(from, Tile::Empty);
        self.set_tile(capture, Tile::Empty);
        self.set_tile(to, tile_value);
        self.king_check(to);
        if self.can_capture_a_piece(to) {
          self.mode = Mode::ChainCapture(to);
        }
        else {
          self.active_player_swap();
          self.mode = Mode::StartOfTurn;
        }
        if self.victory_check() {
          self.mode = Mode::Victory(tile_value.player().unwrap());
        }
      }
    }
  }

  fn player_score(&self, player : i64) -> f64 {
    let (white, black) = self.piece_count();
    match player {
      0 => (white - black) as f64,
      1 => (black - white) as f64,
      _ => panic!("checkers is a two-player game"),
    }
  }

  fn winner(&self) -> Option<i64> {
    match self.mode {
      Mode::Victory(White) => Some(0),
      Mode::Victory(Black) => Some(1),
      _ => None,
    }
  }
}

fn draw_checkers(game : &Checkers, player_actions : &[Action], context : &Context, graphics : &mut G2d) {
  clear([1.0; 4], graphics);
  for y in 0..BOARD_SIZE {
    for i in (0..BOARD_SIZE).step_by(2) {
      let x = i + (y % 2);
      rectangle(
        [0.0, 0.0, 0.0, 1.0], // black
        [x as f64 * 60.0, y as f64 * 60.0, 60.0, 60.0],
        context.transform,
        graphics);

      let tile = game.get_tile(Pos{x, y});
      let colour = match tile {
        Occupied(Black, Pawn) => Some([1.0, 0.0, 0.0, 1.0]),
        Occupied(White, Pawn) => Some([0.0, 1.0, 0.0, 1.0]),
        Occupied(Black, King) => Some([0.5, 0.0, 0.0, 1.0]),
        Occupied(White, King) => Some([0.0, 0.5, 0.0, 1.0]),
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
  fn draw_border(p : Pos, c : [f32 ; 4], context : &Context, graphics : &mut G2d) {
    Rectangle::new_border(c, 2.0)
      .draw([p.x as f64 * 60.0, p.y as f64 * 60.0, 60.0, 60.0],
        &DrawState::default(), context.transform, graphics);        
  }
  for a in player_actions {
    match *a {
      Action::Jump { from, capture, to } => {
        draw_border(from, [0.0, 0.0, 1.0, 1.0], &context, graphics);
        draw_border(capture, [1.0, 0.0, 1.0, 1.0], &context, graphics);
        draw_border(to, [0.0, 0.0, 1.0, 1.0], &context, graphics);
      }
      Action::Step { from, to } => {
        draw_border(from, [0.0, 0.0, 1.0, 1.0], &context, graphics);
        draw_border(to, [0.0, 0.0, 1.0, 1.0], &context, graphics);
      }
    }
  }
}

/// Load a graphical, interactive checkers game
pub fn play_checkers<A, B>(mut agent_a : A, mut agent_b : B)
  where A : GameAgent<Checkers>, B : GameAgent<Checkers>
{
  println!("Checkers!");
  let mut game = Checkers::new();
  let mut rng = StdRng::from_entropy();

  let mut window: PistonWindow =
    WindowSettings::new("Checkers", [480, 480])
    .exit_on_esc(true).build().unwrap();

  let mut mouse_pos = [0.0, 0.0];
  let mut player_actions = vec![];
  
  while let Some(event) = window.next() {
    if let Some(Button::Keyboard(key)) = event.press_args() {
      if key == Key::Space {
        player_actions.clear();
        agent_action(&mut agent_a, &mut agent_b, &mut game, &mut rng);
      }
      if key == Key::Return {
        game = Checkers::new();
      }
    }
    if let Some(p) = event.mouse_cursor_args() {
      mouse_pos = p;
    }
    // Handle mouse clicks
    if let Some(Button::Mouse(MouseButton::Left)) = event.press_args() {
      let x = (mouse_pos[0] / 60.0) as i32;
      let y = (mouse_pos[1] / 60.0) as i32;
      let pos = Pos{x, y};
      match game.get_tile(pos) {
        // Player clicked an empty tile
        Tile::Empty => {
          for a in player_actions.iter().cloned() {
            let to = match a {
              Action::Step { to, ..} => to,
              Action::Jump { to, ..} => to,
            };
            if to == pos {
              game.apply_action(&a);
              player_actions.clear();
              // AI response
              if game.mode == Mode::StartOfTurn {
                loop {
                  if agent_action(&mut agent_a, &mut agent_b, &mut game, &mut rng) {
                    // loop to complete chains, if needed
                    if let Mode::ChainCapture(_) = game.mode {
                      continue;
                    }
                  }
                  break;
                }
              }
              break;
            }
          }
        }
        // Player clicked an occupied tile
        tile => {
          if Some(game.active_player) == tile.player() {
            player_actions.clear();
            game.possible_actions(&mut player_actions);
            player_actions.retain(|a| match a {
              Action::Jump{from, ..} => *from == pos,
              Action::Step{from, ..} => *from == pos,
            });
          }
        }
      }
    }
    // Handle draw events
    window.draw_2d(&event, |context, graphics, _device| {
      draw_checkers(&game, &player_actions, &context, graphics)
    });
  }
}

