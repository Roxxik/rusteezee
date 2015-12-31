use bit_set::BitSet;

pub struct GameState {
    pub stones: BitSet,
}

impl GameState {
    pub fn new() -> GameState {
        GameState {
            stones: BitSet::new(),
        }
    }

    pub fn flip_stone(&mut self, value: usize) {
        if self.stones.contains(&value) { self.stones.remove(&value); } else { self.stones.insert(value); }
    }
}
