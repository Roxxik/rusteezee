use std::ops::{ Index, IndexMut };

use super::block::Block;

pub type BlockPos = [u8; 3];

#[derive(Copy, Clone, Debug)]
pub struct Chunk {
    blocks: [[[Block; 16]; 16]; 16],
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            blocks: [[[Block::Air; 16]; 16]; 16]
        }
    }
}

impl Index<BlockPos> for Chunk {
    type Output = Block;
    fn index(&self, index: BlockPos) -> &Block {
        &self.blocks[index[0] as usize][index[1] as usize][index[2] as usize]
    }
}

impl IndexMut<BlockPos> for Chunk {
    fn index_mut(&mut self, index: BlockPos) -> &mut Block {
        &mut self.blocks[index[0] as usize][index[1] as usize][index[2] as usize]
    }
}
