pub mod camera;
pub mod error;
pub mod renderer;
pub mod shader;
pub mod picking;
pub mod text;
pub mod texture;

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
