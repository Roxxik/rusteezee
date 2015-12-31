mod block;
mod camera;
mod cube;
mod error;
mod event;
mod shader;
mod text;
mod texture;
mod wire_cube;


#[derive(Clone, Copy, Debug)]
pub struct Position {
    pub cube_pos: (i32, i32, i32),
}
implement_vertex!(Position, cube_pos);


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

use image;
use cgmath::{ Point, Point3, Matrix4 };
use glium::{ self, glutin, DisplayBuild, Surface };
use glium::program::Program;
use glium::glutin::Event as GlEvent;

use self::camera::Camera;
use self::text::Text;
use self::error::RenderCreationError;
use self::event::Event;
use game::GameState;

pub struct Renderer {
    display: glium::Display,
    cube_program: Program,
    wire_program: Program,
    camera: Camera,
    //TODO make fov in degree
    fov: f32, //in radians
    text: Text,
    stats: bool,
    fill: bool,
    game: GameState,
}

impl Renderer {
    pub fn new() -> Result<Renderer, RenderCreationError<glutin::CreationError>> {
        let display = try!(glutin::WindowBuilder::new()
            .with_depth_buffer(24)
            .build_glium());
        let cube_prog = try!(Program::from_source(
            &display,
            shader::CUBE_VERTEX,
            shader::CUBE_FRAGMENT,
            None,
        ));
        let wire_prog = try!(Program::from_source(
            &display,
            shader::WIRE_VERTEX,
            shader::WIRE_FRAGMENT,
            None,
        ));
        let text = try!(Text::new(&display, "/usr/share/fonts/TTF/NotoSans-Regular.ttf", 24));
        Ok(Renderer {
            display: display,
            cube_program: cube_prog,
            wire_program: wire_prog,
            camera: Camera::at(Point3::new(5.0, 5.0, 5.0), Point::origin()),
            fov: ::std::f32::consts::PI / 3.0,
            text: text,
            stats: false,
            fill: true,
            game: GameState::new(),
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

        let vb_cube = glium::VertexBuffer::new(&self.display, &cube::VERTICES).unwrap();
        let ib_cube = glium::IndexBuffer::new(&self.display, glium::index::PrimitiveType::TrianglesList, &cube::INDICES).unwrap();

        let vb_wire = glium::VertexBuffer::new(&self.display, &wire_cube::VERTICES).unwrap();
        let ib_wire = glium::IndexBuffer::new(&self.display, glium::index::PrimitiveType::LinesList, &wire_cube::INDICES).unwrap();

        loop {
            let mut target = self.display.draw();
            target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);
            //target.clear_depth(1.0);

            let perspective: [[f32; 4]; 4] = self.get_perspective(&target).into();
            let view: [[f32; 4]; 4] = self.camera.view_matrix().into();
            {
                let params = self.get_params();

                let cubes: Vec<_> = self.game.stones.into_iter().map(|x| Position { cube_pos: (0, 0, x as i32) }).collect();
                let cubes_buffer = glium::vertex::VertexBuffer::new(&self.display, &cubes).unwrap();

                let wires: Vec<Position> = self.game.stones.into_iter().min().map(|x| Position { cube_pos: (0, 0, x as i32) }).into_iter().collect();
                let wires_buffer = glium::vertex::VertexBuffer::new(&self.display, &wires).unwrap();


                target.draw(
                    (&vb_cube, cubes_buffer.per_instance().unwrap()),
                    &ib_cube,
                    &self.cube_program,
                    &uniform! {
                        model: block::MODEL,
                        view: view,
                        perspective: perspective,
                        tex: texture_sampler,
                    },
                    &params
                ).unwrap();
                target.draw(
                    (&vb_wire, wires_buffer.per_instance().unwrap()),
                    &ib_wire,
                    &self.wire_program,
                    &uniform! { model: block::MODEL, view: view, perspective: perspective },
                    &params
                ).unwrap();

                if self.stats {
                    self.text.draw(&mut target, &format!(
                        "x: {}, y: {}, z: {}, phi: {}, theta: {}",
                        self.camera.pos.x,
                        self.camera.pos.y,
                        self.camera.pos.z,
                        self.camera.phi.s,
                        self.camera.theta.s,
                    ), (1.0, 1.0, 0.0, 1.0));
                }
            }
            target.finish().unwrap();

            self.camera.update();
            self.handle_events();
        }
    }

    fn get_perspective<T: Surface>(&self, surface: &T) -> Matrix4<f32> {
        let (width, height) = surface.get_dimensions();
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

    fn get_params(&self) -> glium::DrawParameters {
        use glium::draw_parameters::PolygonMode::{ Fill, Line };
        use glium::draw_parameters::DepthTest;
        glium::DrawParameters {
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
        match ev {
            _ => None,
        }
    }

    fn handle_events(&mut self) {
        for ev in self.display.poll_events() {
            use glium::glutin::Event::{ KeyboardInput, Closed };
            use glium::glutin::ElementState::{ Pressed, Released };
            use glium::glutin::VirtualKeyCode;
            use glium::glutin::VirtualKeyCode::*;
            match ev {
                Closed => return,
                KeyboardInput(Pressed, _, Some(F3)) => {
                    self.stats = !self.stats;
                },
                KeyboardInput(Pressed, _, Some(F1)) => {
                    self.fill = !self.fill;
                },
                KeyboardInput(state, _, Some(key)) => {
                    use self::camera::Direction::*;
                    if state == Pressed {
                        if let Some(stone) = match key {
                            Key1 => Some(1),
                            Key2 => Some(2),
                            Key3 => Some(3),
                            Key4 => Some(4),
                            Key5 => Some(5),
                            Key6 => Some(6),
                            Key7 => Some(7),
                            Key8 => Some(8),
                            Key9 => Some(9),
                            _ => None,
                        } {
                            self.game.flip_stone(stone);
                        }
                    }
                    if let Some(dir) = match key {
                        W => Some(Forward),
                        S => Some(Backward),
                        Space => Some(Up),
                        LShift => Some(Down),
                        A => Some(Left),
                        D => Some(Right),
                        VirtualKeyCode::Left => Some(TurnLeft),
                        VirtualKeyCode::Right => Some(TurnRight),
                        VirtualKeyCode::Up => Some(TurnUp),
                        VirtualKeyCode::Down => Some(TurnDown),
                        _ => None,
                    } {
                        self.camera.set_dir(
                            match state {
                                Pressed => true,
                                Released => false,
                            },
                            dir,
                        );
                    }

                },
                _ => {},
            }
        }
    }
}
