
use std::fmt;
use piston_window::*;
use rand::{SeedableRng, rngs::StdRng};

use crate::utils::*;
use crate::agents::*;

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
#[derive(Copy, Clone, PartialEq)]
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

type Board = crate::utils::Board<Tile>;

#[derive(Debug, Copy, Clone)]
pub enum Action {
  Step { from : Pos, to : Pos },
  Jump { from : Pos, capture : Pos, to : Pos },
}

#[derive(Clone)]
pub struct Checkers {
  board : Board,
  active_player : Player,
  mode : Mode,
}

impl Checkers {

  fn new() -> Checkers {
    let mut board = Board::new(Tile::Empty, BOARD_SIZE);
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
    Checkers { board, active_player : WhitePlayer, mode: Mode::StartOfTurn }
  }

  fn actions_from_pos(&self, start : Pos, actions : &mut Vec<Action>, find_steps : &mut bool) {
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
                if *find_steps {
                  actions.clear();
                  *find_steps = false;
                }
                actions.push(Action::Jump{ from: start, capture: pos, to: jump});
              }
            }
          }
          None => {
            if *find_steps {
              actions.push(Action::Step{ from: start, to: pos});
            }
          }
        }
      }
    }
  }

  /// Return true if the piece at pos can capture a piece
  /// in its next move
  fn can_capture_a_piece(&mut self, p : Pos) -> bool {
    let mut actions = vec!();
    self.actions_from_pos(p, &mut actions, &mut false);
    actions.len() > 0
  }

  /// Turn the tile at `pos` into a king if it is
  /// currently a normal piece, and just reached the
  /// final row at the other end of the board
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
    for tile in self.board.iter() {
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
}

impl Game for Checkers {
  type Action = Action;

  fn possible_actions(&self, actions : &mut Vec<Action>) {
    match self.mode {
      Mode::StartOfTurn => {
        let mut find_steps = true;
        for (i, p) in self.board.iter().enumerate() {
          if p.player() == Some(self.active_player) {
            let x = (i as i32) % BOARD_SIZE;
            let y = (i as i32) / BOARD_SIZE;
            let p = Pos{ x, y };
            self.actions_from_pos(p, actions, &mut find_steps);
          }
        }
      }
      Mode::Chain(p) => {
        self.actions_from_pos(p, actions, &mut false);
      }
      Mode::Victory(_) => (),
    }
  }

  fn active_player(&self) -> i64 {
    match self.active_player {
      WhitePlayer => 0, BlackPlayer => 1
    }
  }

  fn apply_action(&mut self, a : &Action) {
    match *a {
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
        if self.can_capture_a_piece(to) {
          self.mode = Mode::Chain(to);
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
}

impl fmt::Display for Checkers {
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

pub fn play_checkers(agents : [&mut dyn GameAgent<Checkers> ; 2]){

  println!("Checkers!");
  let mut game = Checkers::new();
  let mut rng = StdRng::from_entropy(); //StdRng::seed_from_u64(0);

  let mut window: PistonWindow =
    WindowSettings::new("Checkers", [480, 480])
    .exit_on_esc(true).build().unwrap();

  let mut mouse_pos = [0.0, 0.0];
  let mut player_actions = vec![];
  
  while let Some(event) = window.next() {
    if let Some(Button::Keyboard(key)) = event.press_args() {
      if key == Key::Space {
        let player = game.active_player() as usize;
        if let Some(a) = agents[player].choose_action(&mut game, &mut rng) {
          game.apply_action(&a);
        }
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
      match game.board.get(pos) {
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
                let player = game.active_player();
                loop {
                  // loop to complete chains, if needed
                  if let Some(a) = agents[player as usize].choose_action(&mut game, &mut rng) {
                    game.apply_action(&a);
                    if let Mode::Chain(_) = game.mode {
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

