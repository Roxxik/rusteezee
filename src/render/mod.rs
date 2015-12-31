mod shader;
mod camera;
mod cube;
mod texture;
mod block;
mod wire_cube;

mod position {
    #[derive(Clone, Copy, Debug)]
    pub struct Position {
        pub cube_pos: (i32, i32, i32),
    }
    implement_vertex!(Position, cube_pos);
}

use std::error::Error;
use std::fmt;

use image;
use cgmath::{ Point, Point3, Matrix4 };
use glium::{ self, glutin, DisplayBuild, GliumCreationError };

use self::camera::Camera;
use self::position::Position;
use game::GameState;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum RenderCreationError {
    ContextCreationError(GliumCreationError<glutin::CreationError>),
    ProgramCreationError(glium::program::ProgramCreationError),
}

impl fmt::Display for RenderCreationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        match self {
            &RenderCreationError::GliumCreationError(ref s) =>
                write!(fmt, "{}: {}", self.description(), s),
            &RenderCreationError::ProgramCreationError(ref s) =>
                write!(fmt, "{}: {}", self.description(), s),
        }
    }
}

impl Error for RenderCreationError {
    fn description(&self) -> &str {
        match self {
            &RenderCreationError::GliumCreationError(_) =>
                "Error while creating the Render Context",
            &RenderCreationError::ProgramCreationError(_) =>
                "Error while compiling the Shader",
        }
    }

    #[inline]
    fn cause(&self) -> Option<&Error> {
        match self {
            &RenderCreationError::GliumCreationError(ref s) => s,
            &RenderCreationError::ProgramCreationError(ref s) => s,
        }
    }
}


#[derive(Debug)]
pub struct Renderer {
    display: glium::Display,
    cube_program: glium::program::Program,
    wire_program: glium::program::Program,
    camera: Camera,
    //TODO make fov in degree
    fov: f32, //in radians
    stats: bool,
    fill: bool,
    game: GameState,
}

impl Renderer {
    pub fn new() -> Result<Renderer, RenderError> {
        let display = try!(glutin::WindowBuilder::new()
            .with_depth_buffer(24)
            .build_glium());
        let cube_prog = try!(glium::program::Program::from_source(
            &display,
            shader::cube_shader_vertex_src,
            shader::cube_shader_fragment_src,
            None,
        ));
        let wire_prog = try!(glium::program::Program::from_source(
            &display,
            shader::wire_shader_vertex_src,
            shader::wire_shader_fragment_src,
            None,
        ));
        Renderer {
            display: display,
            cube_program: cube_prog,
            wire_program: wire_prog,
            camera: Camera::at(Point3::new(5.0, 5.0, 5.0), Point::origin()),
            fov: ::std::f32::consts::PI / 3.0,
            stats: false,
            fill: false,
            game: GameState::new(),
        }
    }

    pub fn game_loop(self) {
        let image = image::load(::std::io::Cursor::new(&include_bytes!(
            "../../assets/textures/dirt.png"
        )[..]),image::PNG).unwrap().to_rgba();
        let image_dimensions = image.dimensions();
        let image = glium::texture::RawImage2d::from_raw_rgba_reversed(image.into_raw(), image_dimensions);
        let texture = glium::texture::SrgbTexture2d::new(&self.display, image).unwrap();


        loop {
            let mut target = self.display.draw();
            target.clear_color_and_depth((0.0, 0.0, 1.0, 1.0), 1.0);

            let perspective = Renderer::get_pespective(&target);
            let view: [[f32; 4]; 4] = self.camera.view_matrix().into();
            let params = self.get_params();

            let cubes: Vec<_> = self.game.stones.into_iter().map(|x| Position { cube_pos: (0, 0, x as i32) }).collect();
            let cubes_buffer = glium::vertex::VertexBuffer::new(&self.display, &cubes).unwrap();

            let wires: Vec<Position> = self.game.stones.into_iter().min().map(|x| Position { cube_pos: (0, 0, x as i32) }).into_iter().collect();
            let wires_buffer = glium::vertex::VertexBuffer::dynamic(&self.display, &wires).unwrap();

            let vb_cube = glium::VertexBuffer::new(&self.display, &cube::VERTICES).unwrap();
            let ib_cube = glium::IndexBuffer::new(&self.display, glium::index::PrimitiveType::TrianglesList, &cube::INDICES).unwrap();

            let vb_wire = glium::VertexBuffer::new(&self.display, &wire_cube::VERTICES).unwrap();
            let ib_wire = glium::IndexBuffer::new(&self.display, glium::index::PrimitiveType::LinesList, &wire_cube::INDICES).unwrap();

            target.draw(
                (&vb_cube, cubes_buffer.per_instance().unwrap()),
                &ib_cube,
                &self.cube_prog,
                &uniform! {
                    model: self::block::MODEL,
                    view: view,
                    perspective: perspective,
                    tex: glium::uniforms::Sampler::new(&texture)
                        .magnify_filter(glium::uniforms::MagnifySamplerFilter::Nearest)
                        .minify_filter(glium::uniforms::MinifySamplerFilter::Nearest)
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
                self.text.draw(&mut target, &*format!(
                    "x: {}, y: {}, z: {}, phi: {}, theta: {}",
                    self.camera.pos.x,
                    self.camera.pos.y,
                    self.camera.pos.z,
                    self.camera.phi.s,
                    self.camera.theta.s,
                ), (1.0, 1.0, 0.0, 1.0));
            }
            target.finish().unwrap();

            self.handle_events();
        }
    }

    fn get_perspective(&self, surface: &glium::Surface) -> Matrix4<f32> {
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

    fn handle_events(&mut self) {
        for ev in self.display.poll_events() {
            use glium::glutin::Event::KeyboardInput;
            use glium::glutin::ElementState::{ Pressed, Released };
            use glium::glutin::VirtualKeyCode::*;
            match ev {
                glium::glutin::Event::Closed => return,
                KeyboardInput(Pressed, _, Some(F3)) => {
                    self.show_stats = !self.show_stats;
                },
                KeyboardInput(Pressed, _, Some(F1)) => {
                    self.fill = !self.fill;
                },
                _ => {},
            }
        }
    }
}
