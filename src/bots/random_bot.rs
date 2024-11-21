use rand::seq::SliceRandom;

use super::*;

pub struct RandomBot {
    rng: rand::rngs::ThreadRng,
}

impl RandomBot {
    pub fn new() -> RandomBot {
        RandomBot { rng: rand::thread_rng() }
    }
}

impl Bot for RandomBot {
    fn id() -> &'static str {
        "Random"
    }

    fn play(&mut self, instance: &super::GameInstance) -> Option<Move> {
        get_available_moves(instance)
            .choose(&mut self.rng)
            .cloned()
    }
}
