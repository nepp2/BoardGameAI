
use piston_window::*;
use rand::{SeedableRng, rngs::StdRng};

use crate::utils::*;
use crate::agents::*;

/// Holds the contents of a tile
#[derive(Copy, Clone, PartialEq)]
enum Tile {
  Occupied(Player, Piece),
  Empty,
}

/// The type of piece
#[derive(Copy, Clone, PartialEq)]
enum Piece {
  Pawn, // normal pawn
  Bishop,
  Knight,
  Rook,
  Queen,
  King,
  StartingPawn, // can move two spaces
  HoppedPawn, // just moved two spaces
  StartingKing, // can castle
  StartingRook, // can castle
}

/// The two competing players
#[derive(Copy, Clone, PartialEq)]
enum Player {
  White,
  Black,
}

/// The mode that the game is in.
#[derive(Copy, Clone, PartialEq)]
enum Mode {
  /// The active player can choose any move
  Turn,

  /// One of the players won
  Victory(Player),
}

use Tile::*;
use Player::*;
use Piece::*;

impl Tile {
  /// Indicates which player the tile belongs to (if either)
  fn player(self) -> Option<Player> {
    match self {
      Occupied(p, _) => Some(p),
      Empty => None,
    }
  }
}

/// The size of the board (which is assumed to be square)
const BOARD_SIZE : i32 = 8;

type Board = crate::utils::Board<Tile>;

#[derive(Debug, Copy, Clone)]
pub enum Action {
  Move { from : Pos, to : Pos },
  Castle { king : Pos, king_to : Pos, rook : Pos, rook_to : Pos },
}

#[derive(Clone)]
pub struct Chess {
  board : Board,
  active_player : Player,
  mode : Mode,
}

impl Chess {

  pub fn new() -> Chess {
    let mut board = Board::new(Tile::Empty, BOARD_SIZE);
    for y in 0..3 {
      for i in (0..BOARD_SIZE).step_by(2) {
        let x = i + (y % 2);
        board.set(Pos {x, y}, Occupied(White, Pawn));
      }
    }
    for y in (BOARD_SIZE-3)..BOARD_SIZE {
      for i in (0..BOARD_SIZE).step_by(2) {
        let x = i + (y % 2);
        board.set(Pos {x, y}, Occupied(Black, Pawn));
      }
    }
    Chess { board, active_player : White, mode: Mode::Turn }
  }

  fn visit_player_pieces(&self, player : Player, mut f : impl FnMut(Pos)) {
    for (i, t) in self.board.iter().enumerate() {
      if t.player() == Some(player) {
        let p = Pos {
          x: (i as i32) % BOARD_SIZE,
          y: (i as i32) / BOARD_SIZE,
        };
        f(p);
      }
    }
  }

  fn find_moves_at_position(&self, pos : Pos, actions : &mut Vec<Action>) {
    panic!()
  }

  /// Turn the tile at `pos` into a queen if it is
  /// currently a pawn, and just reached the
  /// final row at the other end of the board
  fn pawn_promotion(&mut self, p : Pos) {
    let tile_value = self.board.get(p);
    match tile_value {
      Occupied(Black, Pawn) => {
        if p.y == 0 {
          self.board.set(p, Occupied(Black, Queen));
        }
      }
      Occupied(White, Pawn) => {
        if p.y == BOARD_SIZE-1 {
          self.board.set(p, Occupied(White, Queen));
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
    for tile in self.board.iter() {
      match tile {
        Occupied(Black, _) => black += 1,
        Occupied(White, _) => white += 1,
        Empty => (),
      }
    }
    (white, black)
  }

  fn active_player_swap(&mut self) {
    match self.active_player {
      White => self.active_player = Black,
      Black => self.active_player = White,
    }
  }
}

impl Game for Chess {
  type Action = Action;

  fn possible_actions(&self, actions : &mut Vec<Action>) {
    match self.mode {
      Mode::Turn => {
        let p = self.active_player;
        self.visit_player_pieces(p, |pos| {
          self.find_moves_at_position(pos, actions);
        });
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
      Action::Move { from, to } => {
        let tile_value = self.board.get(from);
        self.board.set(from, Tile::Empty);
        self.board.set(to, tile_value);
        self.pawn_promotion(to);
        self.active_player_swap();
        if self.victory_check() {
          self.mode = Mode::Victory(tile_value.player().unwrap());
        }
        else {
          self.mode = Mode::Turn;
        }
      }
      Action::Castle { king, king_to, rook, rook_to } => {
        panic!()
      }
    }
  }

  fn player_score(&self, player : i64) -> f64 {
    let (white, black) = self.piece_count();
    match player {
      0 => (white - black) as f64,
      1 => (black - white) as f64,
      _ => panic!("chess is a two-player game"),
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

fn draw_chess(game : &Chess, player_actions : &[Action], context : &Context, graphics : &mut G2d) {
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
        Occupied(Black, _) => Some([1.0, 0.0, 0.0, 1.0]),
        Occupied(White, _) => Some([0.0, 1.0, 0.0, 1.0]),
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
      Action::Move { from, to } => {
        draw_border(from, [0.0, 0.0, 1.0, 1.0], &context, graphics);
        draw_border(to, [0.0, 0.0, 1.0, 1.0], &context, graphics);
      }
      Action::Castle { king, king_to, rook, rook_to } => {
        draw_border(king, [0.0, 0.0, 1.0, 1.0], &context, graphics);
        draw_border(king_to, [0.0, 0.0, 1.0, 1.0], &context, graphics);
        draw_border(rook, [0.0, 0.0, 1.0, 1.0], &context, graphics);
        draw_border(rook_to, [0.0, 0.0, 1.0, 1.0], &context, graphics);
      }
    }
  }
}

fn handle_click(
  game : &mut Chess,
  player_actions : &mut Vec<Action>,
  pos : Pos,
) -> Option<Action>
{
  let tile = game.board.get(pos);
  for a in player_actions.iter().cloned() {
    let to = match a {
      Action::Move { to, ..} => to,
      Action::Castle { king_to, ..} => king_to,
    };
    if to == pos {
      return Some(a);
    }
  }
  if let Occupied(player, _) = tile {
    if game.active_player == player {
      player_actions.clear();
      game.possible_actions(player_actions);
      player_actions.retain(|a| match a {
        Action::Move{from, ..} => *from == pos,
        Action::Castle{king, ..} => *king == pos,
      });
    }
  }
  None
}

pub fn play_chess<A, B>(mut agent_a : A, mut agent_b : B)
  where A : GameAgent<Chess>, B : GameAgent<Chess>
{
  println!("Chess!");
  let mut game = Chess::new();
  let mut rng = StdRng::from_entropy(); //StdRng::seed_from_u64(0);

  let mut window: PistonWindow =
    WindowSettings::new("Chess", [480, 480])
    .exit_on_esc(true).build().unwrap();

  let mut mouse_pos = [0.0, 0.0];
  let mut player_actions = vec![];
  
  while let Some(event) = window.next() {
    if let Some(Button::Keyboard(key)) = event.press_args() {
      if key == Key::Space {
        agent_action(&mut agent_a, &mut agent_b, &mut game, &mut rng);
      }
      if key == Key::Return {
        game = Chess::new();
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
      if let Some(action) = handle_click(&mut game, &mut player_actions, pos) {
        game.apply_action(&action);
        player_actions.clear();
        // AI response
        if game.mode == Mode::Turn {
          agent_action(&mut agent_a, &mut agent_b, &mut game, &mut rng);
        }
      }
    }
    // Handle draw events
    window.draw_2d(&event, |context, graphics, _device| {
      draw_chess(&game, &player_actions, &context, graphics)
    });
  }
}

