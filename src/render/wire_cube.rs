#[derive(Clone, Copy, Debug)]
pub struct WireVertex {
    pos: (f32, f32, f32),
    color: (f32, f32, f32),
}
implement_vertex!(WireVertex, pos, color);

const SCALE: f32 = 1.01;

const COLOR: (f32, f32, f32) = (0.0, 0.0, 0.0);

use glium::index::PrimitiveType;

pub const PRIMITIVE_TYPE:  PrimitiveType = PrimitiveType::LinesList;

pub const VERTICES: [WireVertex; 8] = [
    WireVertex { pos: ( 0.5 * SCALE,  0.5 * SCALE,  0.5 * SCALE), color: COLOR },
    WireVertex { pos: ( 0.5 * SCALE,  0.5 * SCALE, -0.5 * SCALE), color: COLOR },
    WireVertex { pos: ( 0.5 * SCALE, -0.5 * SCALE,  0.5 * SCALE), color: COLOR },
    WireVertex { pos: ( 0.5 * SCALE, -0.5 * SCALE, -0.5 * SCALE), color: COLOR },
    WireVertex { pos: (-0.5 * SCALE,  0.5 * SCALE,  0.5 * SCALE), color: COLOR },
    WireVertex { pos: (-0.5 * SCALE,  0.5 * SCALE, -0.5 * SCALE), color: COLOR },
    WireVertex { pos: (-0.5 * SCALE, -0.5 * SCALE,  0.5 * SCALE), color: COLOR },
    WireVertex { pos: (-0.5 * SCALE, -0.5 * SCALE, -0.5 * SCALE), color: COLOR },
];

pub const INDICES: [u16; 24] = [
    //Top
    0, 1,
    1, 5,
    5, 4,
    4, 0,
    //Bottom
    2, 3,
    3, 7,
    7, 6,
    6, 2,
    //Sides
    0, 2,
    1, 3,
    5, 7,
    4, 6,
];
