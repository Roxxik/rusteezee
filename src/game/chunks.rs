use std::ops::{ Index, IndexMut };
use std::collections::HashMap;

use super::chunk::Chunk;


pub type ChunkPos = [i32; 3];

#[derive(Debug)]
pub struct Chunks {
    chunks: HashMap<ChunkPos, Chunk>,
    empty: Chunk,
}

impl Chunks {
    pub fn new() -> Chunks {
        Chunks {
            chunks: HashMap::new(),
            empty: Chunk::new(),
        }
    }
}

impl Index<ChunkPos> for Chunks {
    type Output = Chunk;
    fn index(&self, index: ChunkPos) -> &Chunk {
        self.chunks.get(&index).unwrap_or(&self.empty)
    }
}

impl IndexMut<ChunkPos> for Chunks {
    fn index_mut(&mut self, index: ChunkPos) -> &mut Chunk {
        self.chunks.entry(index).or_insert_with(Chunk::new)
    }
}
