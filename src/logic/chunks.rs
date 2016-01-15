use std::ops::{ Index, IndexMut };
use std::collections::HashMap;

use cgmath::{ Point, Point3 };

use super::chunk::Chunk;


pub type ChunkPos = Point3<i32>;

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

    pub fn around(dist: u8, center: ChunkPos) -> Vec<ChunkPos> {
        let mut res = Vec::new();
        let dist = dist as i32;
        for x in -dist + 1..dist {
            for y in -dist + 1..dist {
                for z in -dist + 1..dist {
                    let rel = Point3::new(x, y, z);
                    res.push(center + rel.to_vec());
                }
            }
        }
        res
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
