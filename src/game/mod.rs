pub mod chunk;
pub mod chunks;
pub mod block;

use cgmath::{ Point, Point3 };

use ::render::Face;
use self::chunks::{ Chunks, ChunkPos };
use self::chunk::{ Chunk, BlockPos };
use self::block::Block;

pub struct GameState {
    chunks: Chunks,
    selected_block: Option<(ChunkPos, BlockPos, Face)>,
}

impl GameState {
    pub fn new() -> GameState {
        let mut game = GameState {
            chunks: Chunks::new(),
            selected_block: None,
        };
        for x in -10..10 {
            for z in -10..10 {
                game.chunks[Point3::new(x, 0, z)] = Chunk::new_with(Block::Dirt);
            }
        }
        game
    }

    fn normalize(mut chunk: ChunkPos, mut block: Point3<i8>) -> (ChunkPos, BlockPos) {
        for i in 0..3 {
            while block[i] < 0 {
                block[i] = block[i] + 16;
                chunk[i] = chunk[i] - 1;
            }
            while block[i] >= 16 {
                block[i] = block[i] - 16;
                chunk[i] = chunk[i] + 1;
            }
        }
        (chunk, Point3::from_vec(block.to_vec().cast()))
    }

    pub fn set_selected_block(&mut self, block: Option<(ChunkPos, BlockPos, Face)>) {
        self.selected_block = block;
    }

    pub fn get_selected_block(&self) -> Option<(ChunkPos, BlockPos, Face)> {
        self.selected_block
    }

    pub fn attack(&mut self) {
        self.selected_block.map(|(c, b, _)| self.chunks[c][b] = Block::Air);
    }

    pub fn place(&mut self) {
        self.selected_block.map(|(c, b, f)| {
            let b = Point3::from_vec((
                  b.to_vec().cast()
                + f.to_vec()
            ));

            let (c, b) = GameState::normalize(c, b);
            self.chunks[c][b] = Block::Dirt
        });
    }

    pub fn chunk(&self, pos: ChunkPos) -> Chunk {
        self.chunks[pos]
    }
}
