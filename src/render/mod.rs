pub mod camera;
pub mod error;
pub mod event;
pub mod renderer;
pub mod shader;
pub mod picking;
pub mod text;
pub mod texture;

use cgmath::Vector3;

#[derive(Clone, Copy, Debug)]
pub struct FaceVertex {
    pub face: u8,
    pub pos: [u8; 3],
    pub corner: [f32; 3],

}
implement_vertex!(FaceVertex, face, pos, corner);

#[derive(Clone, Copy, Debug)]
pub struct WireVertex {
    pub corner: [f32; 3],
}
implement_vertex!(WireVertex, corner);


#[derive(Clone, Copy, Debug)]
pub enum Face {
    Top,
    Bottom,
    North,
    East,
    South,
    West,
}

impl Face {
    pub fn values() -> Vec<Face> {
        use self::Face::*;
        vec![
            Top,
            Bottom,
            North,
            East,
            South,
            West,
        ]
    }

    pub fn to_vec(self) -> Vector3<i8> {
        use self::Face::*;
        match self {
            Top    => Vector3::new( 0,  1,  0),
            Bottom => Vector3::new( 0, -1,  0),
            North  => Vector3::new( 0,  0, -1),
            East   => Vector3::new( 1,  0,  0),
            South  => Vector3::new( 0,  0,  1),
            West   => Vector3::new(-1,  0,  0),
        }
    }
}

impl From<u32> for Face {
    fn from(x: u32) -> Face {
        use self::Face::*;
        assert!(Top as u32 <= x && x <= West as u32);
        unsafe { ::std::mem::transmute(x as u8) }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum HDirection {
    Forth,
    Back,
    Left,
    Right,
}

#[derive(Clone, Copy, Debug)]
pub enum VDirection {
    Up,
    Down,
}
