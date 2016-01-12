mod camera;
mod cube;
mod error;
mod event;
mod shader;
mod picking;
mod text;
mod texture;
mod wire_cube;

use std::f32::consts::PI;
use std::collections::HashMap;

use image;
use cgmath::{ Point, Point3, Matrix4, Vector3 };
use glium::{ self, glutin, DisplayBuild, Surface, Display, VertexBuffer };
use glium::program::Program;
use glium::glutin::Event as GlEvent;
use glium::backend::glutin_backend::WinRef;
use glium::draw_parameters::DrawParameters;
use glium::index::{ NoIndices, PrimitiveType };

use self::camera::Camera;
use self::text::Text;
use self::error::RendererCreationError;
use self::event::Event;
use self::picking::Picker;
use game::GameState;
use game::chunks::{ Chunks, ChunkPos };

const MOUSE_SENSIVITY: f32 = 0.1;

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

pub struct Renderer {
    display: Display,
    picker: Picker,
    cube_program: Program,
    wire_program: Program,
    camera: Camera,
    fov: f32, //in radians
    text: Text,
    stats: bool,
    fill: bool,
    game: GameState,
}

impl Renderer {
    pub fn new() -> Result<Renderer, RendererCreationError<glutin::CreationError>> {
        let display = try!(glutin::WindowBuilder::new()
            .with_depth_buffer(24)
            .with_vsync()
            .build_glium());
        {
            let window = display.get_window().unwrap();
            window.set_cursor(glium::glutin::MouseCursor::Crosshair);
        }

        Ok(Renderer {
            picker: try!(Picker::new(&display)),
            text: try!(Text::new(&display, "/usr/share/fonts/TTF/NotoSans-Regular.ttf", 24)),
            cube_program: try!(Program::from_source(
                &display,
                shader::cube::VERTEX,
                shader::cube::FRAGMENT,
                Some(shader::cube::GEOMETRY),
            )),
            wire_program: try!(Program::from_source(
                &display,
                shader::wire::VERTEX,
                shader::wire::FRAGMENT,
                Some(shader::wire::GEOMETRY),
            )),
            camera: Camera::at(Point3::new(5.0, 5.0, 5.0), Point::origin()),
            fov: PI / 3.0,
            stats: false,
            fill: true,
            game: GameState::new(),
            display: display,
        })
    }

    pub fn game_loop(mut self) {

        let image = image::load(::std::io::Cursor::new(&include_bytes!(
            "../../assets/textures/dirt.png"
        )[..]),image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(&self.display, image).unwrap();
        let texture_sampler = glium::uniforms::Sampler::new(&texture)
            .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
            .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest);

        let wires_buffer: VertexBuffer<WireVertex> = VertexBuffer::immutable(&self.display, &[
            WireVertex { corner: [1.0, 1.0, 1.0] },
            WireVertex { corner: [0.0, 0.0, 0.0] },
        ]).unwrap();

        let mut chunks: HashMap<ChunkPos, VertexBuffer<FaceVertex>> = HashMap::new();
        let view_dist = 2;
        loop {
            //create chunk buffers
            chunks.clear();//TODO this needs some optimization
            let center = self.camera.get_chunk_pos();
            let surroundings = Chunks::around(view_dist, center);
            for (pos, rel) in surroundings {
                chunks.insert(rel, VertexBuffer::new(&self.display, &self.game.chunk(pos).as_faces()).unwrap());
            }

            {//pick from previous frame
                let pick_res = self.picker.pick().map(|(c, b, f)| {
                    (
                        self.camera.get_chunk_pos() + c.to_vec(),
                        b,
                        f
                    )
                });
                self.game.set_selected_block(pick_res);
            }

            // draw
            let mut target = self.display.draw();
            self.picker.resize(&self.display, target.get_dimensions());

            target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
            self.picker.clear(&self.display);

            let perspective = self.get_perspective(target.get_dimensions());
            let view = self.camera.view_matrix();

            let vp: [[f32; 4]; 4]  = (perspective * view).into();

            let params = self.get_params();

            for (pos, vb) in chunks.iter() {
                let pos = (pos[0], pos[1], pos[2]);
                self.picker.draw(
                    &self.display,
                    vb,
                    &NoIndices(PrimitiveType::LinesList),
                    &uniform! { vp: vp, chunk: pos },
                    &params
                );

                target.draw(
                    vb,
                    &NoIndices(PrimitiveType::LinesList),
                    &self.cube_program,
                    &uniform! {
                        vp : vp,
                        chunk: pos,
                        tex: texture_sampler,
                    },
                    &params
                ).unwrap();
            }
            if let Some((chunk, pos, _)) = self.game.get_selected_block() {
                let pos: [u32; 3] = pos.to_vec().cast().into();
                let chunk: [i32; 3] = (chunk - self.camera.get_chunk_pos()).into();
                target.draw(
                    &wires_buffer,
                    &NoIndices(PrimitiveType::LinesList),
                    &self.wire_program,
                    &uniform! { vp: vp, pos: pos, chunk: chunk, color: [0.0, 0.0, 0.0, 1.0f32] },
                    &params
                ).unwrap();
            }

            if self.stats {
                self.text.draw(&mut target, &format!("{}", self.camera), (1.0, 1.0, 0.0, 1.0));
            }

            target.finish().unwrap();


            if !self.handle_events() {
                return;
            }
            self.camera.update();
        }
    }

    fn get_perspective(&self, dimensions: (u32, u32)) -> Matrix4<f32> {
        let (width, height) = dimensions;
        let aspect_ratio = height as f32 / width as f32;

        let zfar = 1024.0;
        let znear = 0.1;

        let f = 1.0 / (self.fov / 2.0).tan();

        Matrix4::new(
            f * aspect_ratio, 0.0,              0.0              , 0.0,
                   0.0      ,  f ,              0.0              , 0.0,
                   0.0      , 0.0,      (zfar+znear)/(zfar-znear), 1.0,
                   0.0      , 0.0, -(2.0*zfar*znear)/(zfar-znear), 0.0,
        )
    }

    fn get_params<'b, 'c>(&'b self) -> DrawParameters<'c> {
        use glium::draw_parameters::PolygonMode::{ Fill, Line };
        use glium::draw_parameters::DepthTest;

        DrawParameters {
            depth: glium::Depth {
                test: DepthTest::IfLess,
                write: true,
                .. Default::default()
            },
            //backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            polygon_mode: if self.fill { Fill } else { Line },
            .. Default::default()
        }
    }


    fn convert(ev: GlEvent) -> Option<Event> {
        use self::event::Event::*;
        use self::HDirection::*;
        use self::VDirection::*;
        use glium::glutin::Event as E;
        use glium::glutin::ElementState::Pressed;
        use glium::glutin::VirtualKeyCode as V;
        use glium::glutin::MouseButton as M;
        match ev {
            E::MouseInput(Pressed, M::Left ) => Some(Attack),
            E::MouseInput(Pressed, M::Right) => Some(UseItem),
            E::KeyboardInput(state, _, Some(key)) => {
                let t = state == Pressed;
                match (state, key) {
                    (_      , V::W)      => Some(Move { dir: Forth, toogle: t }),
                    (_      , V::A)      => Some(Move { dir: Left , toogle: t }),
                    (_      , V::S)      => Some(Move { dir: Back , toogle: t }),
                    (_      , V::D)      => Some(Move { dir: Right, toogle: t }),
                    (_      , V::Up)     => Some(Turn { dir: Forth, toogle: t }),
                    (_      , V::Left)   => Some(Turn { dir: Left , toogle: t }),
                    (_      , V::Down)   => Some(Turn { dir: Back , toogle: t }),
                    (_      , V::Right)  => Some(Turn { dir: Right, toogle: t }),
                    (_      , V::Space)  => Some(Fly  { dir: Up   , toogle: t }),
                    (_      , V::LShift) => Some(Fly  { dir: Down , toogle: t }),
                    _ => None,
                }

            },
            _ => None,
        }
    }

    fn handle_events(&mut self) -> bool {
        for ev in self.display.poll_events() {
            use glium::glutin::Event as E;
            use glium::glutin::ElementState::Pressed;
            use glium::glutin::VirtualKeyCode::*;
            match ev {
                E::Closed => return false,
                E::MouseMoved((mouse_x, mouse_y)) => {
                    let window = self.display.get_window().unwrap();
                    let (mid_x, mid_y) = Renderer::fix_mouse(window);
                    // screen coordinates increase to the right, just like phi
                    self.camera.add_phi((mouse_x - mid_x as i32) as f32 * MOUSE_SENSIVITY);
                    // screen coordinates decrease to the top, unlike theta
                    self.camera.add_theta((mid_y - mouse_y as i32) as f32 * MOUSE_SENSIVITY);
                },
                E::KeyboardInput(Pressed, _, Some(F1)) => self.fill = !self.fill,
                E::KeyboardInput(Pressed, _, Some(F3)) => self.stats = ! self.stats,
                E::KeyboardInput(Pressed, _, Some(Escape)) => return false,
                _ => if let Some(ev) = Renderer::convert(ev) {
                    use self::event::Event::*;
                    match ev {
                        Move { dir: d, toogle: t } => self.camera.mov (d, t),
                        Turn { dir: d, toogle: t } => self.camera.turn(d, t),
                        Fly  { dir: d, toogle: t } => self.camera.fly (d, t),
                        Attack                     => self.game.attack(),
                        UseItem                    => self.game.place(),
                        _ => {}
                    }
                },
            }
        }
        return true;
    }

    //returns window mid
    fn fix_mouse(window: WinRef) -> (i32, i32) {
        let (x, y) = window.get_inner_size_points().unwrap();
        let (mid_x, mid_y) = (x as i32 / 2, y as i32 / 2);
        window.set_cursor_position(mid_x, mid_y).unwrap();
        (mid_x, mid_y)
    }
}
