use std::ops::{ Index, IndexMut };

use cgmath::Point3;

use super::block::Block;
use ::render::FaceVertex;

pub type BlockPos = Point3<u8>;

#[derive(Copy, Clone, Debug)]
pub struct Chunk {
    blocks: [[[Block; 16]; 16]; 16],
    dirty: bool,
}

impl Chunk {
    pub fn new() -> Chunk {
        Chunk::new_with(Block::Air)
    }

    pub fn new_with(block: Block) -> Chunk {
        Chunk {
            blocks: [[[block; 16]; 16]; 16],
            dirty: false,
        }
    }

    pub fn clear_dirty(&mut self) {
        self.dirty = false;
    }

    pub fn is_dirty(&self) -> bool {
        self.dirty
    }

    pub fn as_faces(&self) -> Vec<FaceVertex> {
        let mut faces = Vec::new();
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let pos = Point3::new(x, y, z);
                    faces.extend(self[pos].as_faces().iter().map(|&(f, c)|
                        FaceVertex {
                            corner: c.into(),
                            face: f as u8,
                            pos: pos.into()
                        }
                    ));
                }
            }
        }
        faces
    }
}

impl Index<BlockPos> for Chunk {
    type Output = Block;
    fn index(&self, index: BlockPos) -> &Block {
        &self.blocks[index.x as usize][index.y as usize][index.z as usize]
    }
}

impl IndexMut<BlockPos> for Chunk {
    fn index_mut(&mut self, index: BlockPos) -> &mut Block {
        self.dirty = true;
        &mut self.blocks[index.x as usize][index.y as usize][index.z as usize]
    }
}
