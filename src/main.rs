
mod checkers;
mod agents;

use agents::{RandomAgent, RolloutAgent};

fn main() {
  let mut random_agent = RandomAgent{};
  let mut rollout_broad = RolloutAgent{ iterations: 600, depth: 10 };
  let mut rollout_deep = RolloutAgent{ iterations: 300, depth: 20 };

  checkers::play_checkers([&mut rollout_broad, &mut rollout_deep]);
}
