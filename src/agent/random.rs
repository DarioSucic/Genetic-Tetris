use crate::*;

pub struct RandomAgent {
    rng: ThreadRng,
}

impl RandomAgent {
    #[allow(dead_code)]
    pub fn new() -> RandomAgent {
        RandomAgent { rng: thread_rng() }
    }
}

impl Agent for RandomAgent {
    fn get_action(&mut self, _state: &mut TetrisState) -> Option<KeyCode> {
        let possible_keys = &[KeyCode::Left, KeyCode::Right, KeyCode::Up, KeyCode::Down];
        let key = *possible_keys.choose(&mut self.rng).unwrap();
        Some(key)
    }
}
