
mod utils;
mod checkers;
mod chess;
mod tic_tac_toe;
mod agents;
mod contest;

use std::env;
use agents::{RandomAgent, RolloutAgent};

fn main() {
  let random_agent = RandomAgent{};
  let rollout_broad = RolloutAgent{ iterations: 600, depth: 10 };
  let rollout_deep = RolloutAgent{ iterations: 300, depth: 20 };
  let rollout_weak = RolloutAgent{ iterations: 1, depth: 1 };
  let rollout_strong = RolloutAgent{ iterations: 600, depth: 20 };

  if let Some(arg) = env::args().nth(1) {
    match arg.as_str() {
      "contest" => {
        contest::run_contest(tic_tac_toe::TicTacToe::new(3, 3), rollout_broad, rollout_deep);
      }
      "tictactoe" => {
        tic_tac_toe::play_game(rollout_broad, rollout_weak);
      }
      "checkers" => {
        checkers::play_checkers(random_agent, rollout_weak);
      }
      s => println!("Argument not recognised: {}", s),
    }
  }
  else {
    checkers::play_checkers(rollout_strong, rollout_weak);
  }
}
