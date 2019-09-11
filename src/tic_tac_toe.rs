
use piston_window::*;
use rand::{SeedableRng, rngs::StdRng};

use crate::utils::*;
use crate::agents::*;

/// The contents of a tile
type Tile = Option<Player>;

/// The two competing players
#[derive(Copy, Clone, PartialEq, Debug)]
enum Player {
  WhitePlayer,
  BlackPlayer,
}

use Player::*;

type Board = crate::utils::Board<Tile>;

#[derive(Debug, Copy, Clone)]
pub struct Action {
  pos : Pos,
  player : Player,
}

#[derive(Clone)]
pub struct TicTacToe {
  board : Board,
  active_player : Player,
  length_to_win : i32,
  victory : Option<Player>,
}

impl TicTacToe {

  fn new(size : i32, length_to_win : i32) -> TicTacToe {
    let board = Board::new(None, size);
    TicTacToe { board, active_player : WhitePlayer, length_to_win, victory: None }
  }

  fn active_player_swap(&mut self) {
    match self.active_player {
      Player::WhitePlayer => self.active_player = Player::BlackPlayer,
      Player::BlackPlayer => self.active_player = Player::WhitePlayer,
    }
  }
}

impl Game for TicTacToe {
  type Action = Action;

  fn possible_actions(&self, actions : &mut Vec<Action>) {
    if self.victory.is_some() {
      return;
    }
    for y in 0..self.board.size {
      for x in 0..self.board.size {
        let pos = Pos{x, y};
        if self.board.get(pos) == None {
          actions.push(Action { pos, player: self.active_player });
        }
      }
    }
  }

  fn active_player(&self) -> i64 {
    match self.active_player {
      WhitePlayer => 0, BlackPlayer => 1
    }
  }

  fn apply_action(&mut self, a : &Action) {
    self.board.set(a.pos, Some(a.player));
    // victory check
    let dirs = &[ Pos { x: 1, y : 0}, Pos { x: 0, y : 1}, Pos { x: 1, y : 1}, Pos { x: 1, y : -1} ];
    for &d in dirs {
      let forwards_and_back = &[ d, -d ];
      let mut count = 1;
      for &d in forwards_and_back {
        let mut p = a.pos + d;
        while self.board.try_get(p) == Some(Some(a.player)) {
          p += d;
          count += 1;
        }
      }
      if count >= self.length_to_win {
        self.victory = Some(a.player);
        break;
      }
    }
    // Swap active player
    self.active_player_swap();
  }

  fn player_score(&self, player : i64) -> f64 {
    match (self.victory, player) {
      (Some(WhitePlayer), 0) => 0.0,
      (Some(WhitePlayer), 1) => -1.0,
      (Some(BlackPlayer), 1) => 0.0,
      (Some(BlackPlayer), 0) => -1.0,
      _ => 0.0,
    }
  }
}

fn draw_tic_tac_toe(game : &TicTacToe, context : &Context, graphics : &mut G2d) {
  clear([1.0; 4], graphics);
  for y in 0..game.board.size {
    for x in 0..game.board.size {
      let is_black = (x + (y % 2)) % 2 == 0;
      if is_black {
        rectangle(
          [0.0, 0.0, 0.0, 1.0], // black
          [x as f64 * 60.0, y as f64 * 60.0, 60.0, 60.0],
          context.transform,
          graphics);
      }
      let tile = game.board.get(Pos{x, y});
      let colour = match tile {
        Some(BlackPlayer) => Some([1.0, 0.0, 0.0, 1.0]),
        Some(WhitePlayer) => Some([0.0, 1.0, 0.0, 1.0]),
        None => None,
      };
      if let Some(c) = colour {
        ellipse(
          c, [x as f64 * 60.0 + 5.0, y as f64 * 60.0 + 5.0, 50.0, 50.0],
          context.transform, graphics);
      }
    }
  }
}

pub fn play_game(agents : [&mut dyn GameAgent<TicTacToe> ; 2]){

  println!("Checkers!");
  let mut game = TicTacToe::new(3, 3);
  let mut rng = StdRng::from_entropy(); //StdRng::seed_from_u64(0);

  let mut window: PistonWindow =
    WindowSettings::new("Checkers", [480, 480])
    .exit_on_esc(true).build().unwrap();

  let mut mouse_pos = [0.0, 0.0];
  
  while let Some(event) = window.next() {
    if let Some(Button::Keyboard(key)) = event.press_args() {
      if key == Key::Space {
        let player = game.active_player() as usize;
        if let Some(a) = agents[player].choose_action(&mut game, &mut rng) {
          game.apply_action(&a);
        }
      }
      if key == Key::Return {
        game = TicTacToe::new(game.board.size, game.length_to_win);
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
      if game.board.get(pos).is_none() {
        let a = Action { pos, player: game.active_player };
        game.apply_action(&a);
        // AI response
        let player = game.active_player as usize;
        if let Some(a) = agents[player].choose_action(&mut game, &mut rng) {
          game.apply_action(&a);
        }
      }
    }
    // Handle draw events
    window.draw_2d(&event, |context, graphics, _device| {
      draw_tic_tac_toe(&game, &context, graphics)
    });
  }
}

