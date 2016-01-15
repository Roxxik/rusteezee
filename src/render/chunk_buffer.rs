use std::collections::HashMap;

use glium::{ Display, VertexBuffer };
use cgmath::{ Point, Point3 };

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
        let mut new_buf = HashMap::new();
        let surroundings = Chunks::around(self.view_dist, center);
        for pos in surroundings {
            let vb = self.buffer.remove(&pos);
            if vb.is_none() || game.chunk(pos).is_dirty() {
                new_buf.insert(pos, VertexBuffer::new(display, &game.chunk(pos).as_faces()).unwrap());
            } else if let Some(vb) = vb {
                new_buf.insert(pos, vb);
            }
        }
        self.buffer = new_buf;
        self.center = center;
    }

    pub fn iter<'a>(&'a self) -> Vec<(ChunkPos, &'a VertexBuffer<FaceVertex>)> {
        self.buffer.iter().map(|(pos, vb)| (Point3::from_vec(*pos - self.center), vb)).collect()
    }

    pub fn get_view_dist(&self) -> u8 {
        self.view_dist
    }

    pub fn set_view_dist(&mut self, view_dist: u8) {
        self.view_dist = ::std::cmp::max(2, view_dist);
    }
}
