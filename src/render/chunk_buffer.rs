use std::collections::HashMap;

use glium::{ Display, VertexBuffer };
use cgmath::Point;

use render::FaceVertex;
use logic::game::GameState;
use logic::chunks::{ ChunkPos, Chunks };

#[derive(Debug)]
pub struct ChunkBuffer {
    center: ChunkPos,
    view_dist: u8,
    buffer: HashMap<ChunkPos, VertexBuffer<FaceVertex>>
}

impl ChunkBuffer {
    pub fn new(view_dist: u8) -> ChunkBuffer {
        ChunkBuffer {
            buffer: HashMap::new(),
            view_dist: view_dist,
            center: Point::origin(),
        }
    }

    pub fn update(&mut self, display: &Display, game: &GameState, center: ChunkPos) {
        self.buffer.clear();
        let surroundings = Chunks::around(self.view_dist, center);
        for (pos, rel) in surroundings {
            self.buffer.insert(rel, VertexBuffer::new(display, &game.chunk(pos).as_faces()).unwrap());
        }
    }

    pub fn iter(&self) -> ::std::collections::hash_map::Iter<ChunkPos, VertexBuffer<FaceVertex>> {
        self.buffer.iter()
    }
}
