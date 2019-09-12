
use rand::{Rng, rngs::StdRng};
use std::fmt::Debug;

pub trait Game : Clone {
  type Action : Debug;

  fn possible_actions(&self, actions : &mut Vec<Self::Action>);
  fn active_player(&self) -> i64;
  fn apply_action(&mut self, a : &Self::Action);
  fn player_score(&self, player : i64) -> f64;
  fn winner(&self) -> Option<i64>;
}

pub trait GameAgent<G : Game> : Clone {
  fn choose_action(&mut self, game: &G, rng: &mut StdRng) -> Option<G::Action>;
}

#[derive(Clone)]
pub struct RandomAgent {}

impl <G : Game> GameAgent<G> for RandomAgent {
  fn choose_action(&mut self, game : &G, rng : &mut StdRng) -> Option<G::Action> {
    let mut actions = vec![];
    game.possible_actions(&mut actions);
    if actions.len() > 0 {
      let i: usize = rng.gen_range(0, actions.len());
      Some(actions.remove(i))
    }
    else {
      None
    }
  }
}

/// Takes one action for whichever player has the next turn.
/// Returns true if an action was taken.
pub fn agent_action<A, B, G>(a : &mut A, b : &mut B, g : &mut G, rng: &mut StdRng) -> bool
  where A : GameAgent<G>, B : GameAgent<G>, G : Game
{
  let a = match g.active_player() {
    0 => a.choose_action(g, rng),
    1 => b.choose_action(g, rng),
    _ => panic!("no agent found for player"),
  };
  if let Some(a) = a {
    g.apply_action(&a);
    true
  }
  else {
    false
  }
}

#[derive(Clone)]
pub struct RolloutAgent {
  pub iterations : i64,
  pub depth : i64,
}

fn rollout<G : Game>(game : &mut G, rng : &mut StdRng, max_depth : i64) {
  let mut actions = vec![];
  for _ in 0..max_depth {
    actions.clear();
    game.possible_actions(&mut actions);
    if actions.len() > 0 {
      let i: usize = rng.gen_range(0, actions.len());
      game.apply_action(&actions[i]);
    }
    else {
      break;
    }
  }
}

impl <G : Game> GameAgent<G> for RolloutAgent {

  fn choose_action(&mut self, game : &G, rng : &mut StdRng) -> Option<G::Action> {
    let player = game.active_player();
    let mut actions = vec![];
    game.possible_actions(&mut actions);
    let mut best_score = -99999999999999.0;
    let mut best_action = None;
    for a in actions {
      let mut score = 0.0;
      for _ in 0..self.iterations {
        let mut game = game.clone();
        game.apply_action(&a);
        rollout(&mut game, rng, self.depth);
        score += game.player_score(player);
      }
      if score > best_score {
        best_score = score;
        best_action = Some(a);
      }
    }
    best_action
  }
}
