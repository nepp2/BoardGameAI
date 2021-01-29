
use rayon::prelude::*;
use rand::{SeedableRng, rngs::StdRng};

use crate::agents::*;

pub fn run_contest<A, B, G>(g : G, a : A, b : B)
  where A : GameAgent<G> + Send + Sync,
    B : GameAgent<G>  + Send + Sync,
    G : Game + Send + Sync
{
  let total_games = 100;
  let mut games = vec![ g.clone() ; total_games ];  
  let winners = games.par_iter_mut()
    .map(move |g| {
      let mut rng = StdRng::from_entropy();
      let (mut a, mut b) = (a.clone(), b.clone());
      for _ in 0..400 {
        if !agent_action(&mut a, &mut b, g, &mut rng) {
          return g.winner();
        }
      }
      g.winner()
    }).collect::<Vec<_>>();
  let p1_wins = winners.iter().filter(|&&w| w == Some(0)).count();
  let p2_wins = winners.iter().filter(|&&w| w == Some(1)).count();
  println!("P1 wins: {}, P2 wins: {}, Draws: {}",
    p1_wins, p2_wins, total_games - (p1_wins + p2_wins));
}

