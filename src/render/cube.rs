#[derive(Clone, Copy, Debug)]
pub struct CubeVertex {
    pos: (f32, f32, f32),
    tex_pos: (f32, f32),
}
implement_vertex!(CubeVertex, pos, tex_pos);

use glium::index::PrimitiveType;

pub const PRIMITIVE_TYPE:  PrimitiveType = PrimitiveType::TrianglesList;

pub const VERTICES: [CubeVertex; 24] = [
    //North
    CubeVertex { pos: ( 0.5,  0.5, -0.5), tex_pos: (1.0, 1.0) },
    CubeVertex { pos: ( 0.5, -0.5, -0.5), tex_pos: (1.0, 0.0) },
    CubeVertex { pos: (-0.5, -0.5, -0.5), tex_pos: (0.0, 0.0) },
    CubeVertex { pos: (-0.5,  0.5, -0.5), tex_pos: (0.0, 1.0) },
    //South
    CubeVertex { pos: ( 0.5,  0.5,  0.5), tex_pos: (1.0, 1.0) },
    CubeVertex { pos: ( 0.5, -0.5,  0.5), tex_pos: (1.0, 0.0) },
    CubeVertex { pos: (-0.5, -0.5,  0.5), tex_pos: (0.0, 0.0) },
    CubeVertex { pos: (-0.5,  0.5,  0.5), tex_pos: (0.0, 1.0) },
    //West
    CubeVertex { pos: (-0.5,  0.5,  0.5), tex_pos: (0.0, 1.0) },
    CubeVertex { pos: (-0.5,  0.5, -0.5), tex_pos: (1.0, 1.0) },
    CubeVertex { pos: (-0.5, -0.5, -0.5), tex_pos: (1.0, 0.0) },
    CubeVertex { pos: (-0.5, -0.5,  0.5), tex_pos: (0.0, 0.0) },
    //East
    CubeVertex { pos: ( 0.5,  0.5,  0.5), tex_pos: (0.0, 1.0) },
    CubeVertex { pos: ( 0.5,  0.5, -0.5), tex_pos: (1.0, 1.0) },
    CubeVertex { pos: ( 0.5, -0.5, -0.5), tex_pos: (1.0, 0.0) },
    CubeVertex { pos: ( 0.5, -0.5,  0.5), tex_pos: (0.0, 0.0) },
    //Top
    CubeVertex { pos: ( 0.5,  0.5,  0.5), tex_pos: (0.0, 0.0) },
    CubeVertex { pos: ( 0.5,  0.5, -0.5), tex_pos: (0.0, 1.0) },
    CubeVertex { pos: (-0.5,  0.5, -0.5), tex_pos: (1.0, 1.0) },
    CubeVertex { pos: (-0.5,  0.5,  0.5), tex_pos: (1.0, 0.0) },
    //Bottom
    CubeVertex { pos: ( 0.5, -0.5,  0.5), tex_pos: (1.0, 1.0) },
    CubeVertex { pos: ( 0.5, -0.5, -0.5), tex_pos: (1.0, 0.0) },
    CubeVertex { pos: (-0.5, -0.5, -0.5), tex_pos: (0.0, 0.0) },
    CubeVertex { pos: (-0.5, -0.5,  0.5), tex_pos: (0.0, 1.0) },
];

pub const INDICES: [u16; 36] = [
    //North
     0,  1,  2,
     2,  3,  0,
    //South
     4,  5,  6,
     6,  7,  4,
    //West
     8,  9, 10,
    10, 11,  8,
    //East
    12, 13, 14,
    14, 15, 12,
    //Top
    16, 17, 18,
    18, 19, 16,
    //Bottom
    20, 21, 22,
    22, 23, 20,
];
