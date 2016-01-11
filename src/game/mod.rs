pub mod chunk;
pub mod chunks;
pub mod block;

use self::chunks::{ Chunks, ChunkPos };
use self::chunk::{ Chunk, BlockPos };
use self::block::Block;

pub struct GameState {
    chunks: Chunks,
    selected_block: Option<(ChunkPos, BlockPos)>,
}

impl GameState {
    pub fn new() -> GameState {
        let mut game = GameState {
            chunks: Chunks::new(),
            selected_block: None,
        };
        for x in 0..40 {
            let (cx, bx) = (x / 16, x % 16);
            game.place_block(([cx, 0, 0], [bx as u8, 0, 0]));
        }
        game
    }

    pub fn set_selected_block(&mut self, block: Option<(ChunkPos, BlockPos)>) {
        self.selected_block = block;
    }

    pub fn get_selected_block(&self) -> Option<(ChunkPos, BlockPos)> {
        self.selected_block
    }

    pub fn attack(&mut self) {
        self.selected_block.map(|(c, b)| self.chunks[c][b] = Block::Air);
    }

    pub fn place_block(&mut self, pos: (ChunkPos, BlockPos)) {
        self.chunks[pos.0][pos.1] = Block::Dirt;
    }

    pub fn chunk(&self, pos: ChunkPos) -> Chunk {
        self.chunks[pos]
    }
}
