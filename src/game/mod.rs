use std::collections::HashSet;


pub struct GameState {
    pub stones: HashSet<[i32; 3]>,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            stones: HashSet::new(),
        }
    }

    pub fn flip_stone(&mut self, block: [i32; 3]) {
        if self.stones.contains(&block) {
            self.stones.remove(&block);
        } else {
            self.stones.insert(block);
        }

    }
}
