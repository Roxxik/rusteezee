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
}
