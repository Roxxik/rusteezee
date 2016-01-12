use cgmath::Point3;

use ::render::Face;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Block {
    Air,
    Dirt,
}

impl Block {
    pub fn as_faces(self) -> Vec<(Face, Point3<f32>)> {
        if self != Block::Air {
            use ::render::Face::*;
            vec![
                (Top    , Point3::new(0.0, 1.0, 0.0)),
                (Top    , Point3::new(1.0, 1.0, 1.0)),
                (Bottom , Point3::new(0.0, 0.0, 1.0)),
                (Bottom , Point3::new(1.0, 0.0, 0.0)),
                (North  , Point3::new(1.0, 1.0, 0.0)),
                (North  , Point3::new(0.0, 0.0, 0.0)),
                (East   , Point3::new(1.0, 1.0, 1.0)),
                (East   , Point3::new(1.0, 0.0, 0.0)),
                (South  , Point3::new(0.0, 1.0, 1.0)),
                (South  , Point3::new(1.0, 0.0, 1.0)),
                (West   , Point3::new(0.0, 1.0, 0.0)),
                (West   , Point3::new(0.0, 0.0, 1.0)),
            ]
        } else {
            Vec::new()
        }
    }
}
