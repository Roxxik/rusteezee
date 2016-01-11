use std::ops::{ Index, IndexMut };

use super::block::Block;
use ::render::FaceVertex;

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

    pub fn as_faces(&self) -> Vec<FaceVertex> {
        let mut faces = Vec::new();
        for x in 0..16 {
            for y in 0..16 {
                for z in 0..16 {
                    let pos = [x,y,z];
                    if self[pos] != Block::Air {
                        faces.push(FaceVertex { corner: [x as f32 + 0.0, y as f32 + 1.0, z as f32 + 0.0] });
                        faces.push(FaceVertex { corner: [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0] }); //Top
                        faces.push(FaceVertex { corner: [x as f32 + 0.0, y as f32 + 0.0, z as f32 + 1.0] });
                        faces.push(FaceVertex { corner: [x as f32 + 1.0, y as f32 + 0.0, z as f32 + 0.0] }); //Bottom
                        faces.push(FaceVertex { corner: [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 0.0] });
                        faces.push(FaceVertex { corner: [x as f32 + 0.0, y as f32 + 0.0, z as f32 + 0.0] }); //North
                        faces.push(FaceVertex { corner: [x as f32 + 1.0, y as f32 + 1.0, z as f32 + 1.0] });
                        faces.push(FaceVertex { corner: [x as f32 + 1.0, y as f32 + 0.0, z as f32 + 0.0] }); //East
                        faces.push(FaceVertex { corner: [x as f32 + 0.0, y as f32 + 1.0, z as f32 + 1.0] });
                        faces.push(FaceVertex { corner: [x as f32 + 1.0, y as f32 + 0.0, z as f32 + 1.0] }); //South
                        faces.push(FaceVertex { corner: [x as f32 + 0.0, y as f32 + 1.0, z as f32 + 0.0] });
                        faces.push(FaceVertex { corner: [x as f32 + 0.0, y as f32 + 0.0, z as f32 + 1.0] }); //West
                    }
                }
            }
        }
        faces
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
