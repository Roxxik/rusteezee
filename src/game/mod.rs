use std::collections::HashSet;


pub struct GameState {
    pub stones: HashSet<[i32; 3]>,
    selected_block: Option<[i32; 3]>,
}

impl GameState {
    pub fn new() -> GameState {
        let mut stones = HashSet::new();
        stones.insert([ 0,  0,  0]);
        stones.insert([ 1,  0,  0]);
        stones.insert([ 0,  1,  0]);
        stones.insert([ 0,  0,  1]);
        stones.insert([-1,  0,  0]);
        stones.insert([ 0,  0, -1]);

        GameState {
            stones: stones,
            selected_block: None,
        }
    }

    pub fn set_selected_block(&mut self, block: Option<[i32; 3]>) {
        self.selected_block = block;
    }

    pub fn get_selected_block(&self) -> Option<[i32; 3]> {
        self.selected_block
    }

    pub fn attack(&mut self) {
        self.selected_block.map(|b| self.stones.remove(&b));
    }
}
