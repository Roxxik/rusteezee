#[macro_use]
extern crate glium;
extern crate glium_text;
extern crate cgmath;
extern crate image;
extern crate bit_set;



use cgmath::Angle;
use cgmath::Point;
use cgmath::Point3;

mod render;
mod game;

use render::Renderer;

//#[derive(Copy, Clone)]
//enum Face {
//    North, // -z
//    South, // +z
//    West,  // -x
//    East,  // +x
//    Top,   // +y
//    Bottom,// -y
//}
//
//const FACES: [Face; 6] = [
//    Face::North,
//    Face::South,
//    Face::West,
//    Face::East,
//    Face::Top,
//    Face::Bottom,
//];


//"/usr/share/fonts/TTF/NotoSans-Regular.ttf"
fn main() {
    match Renderer::new() {
        Ok(r)  => { r.game_loop(); },
        Err(r) => { println!("{}", r.description()); },
    }


/*
        for ev in display.poll_events() {
            use camera::{ CAM_POS_STEP, CAM_DIR_STEP };
            match ev {
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Space)) => {
                    camera.pos.y += CAM_POS_STEP;
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::LShift)) => {
                    camera.pos.y -= CAM_POS_STEP;
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::A)) => {
                    camera.pos.x += (camera.phi + cgmath::deg(270.0)).sin() * CAM_POS_STEP;
                    camera.pos.z -= (camera.phi + cgmath::deg(270.0)).cos() * CAM_POS_STEP;
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::D)) => {
                    camera.pos.x += (camera.phi + cgmath::deg(90.0)).sin() * CAM_POS_STEP;
                    camera.pos.z -= (camera.phi + cgmath::deg(90.0)).cos() * CAM_POS_STEP;
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::W)) => {
                    camera.pos.x += (camera.phi + cgmath::deg(0.0)).sin() * CAM_POS_STEP;
                    camera.pos.z -= (camera.phi + cgmath::deg(0.0)).cos() * CAM_POS_STEP;
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::S)) => {
                    camera.pos.x += (camera.phi + cgmath::deg(180.0)).sin() * CAM_POS_STEP;
                    camera.pos.z -= (camera.phi + cgmath::deg(180.0)).cos() * CAM_POS_STEP;
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Left)) => {
                    camera.phi = camera.phi - CAM_DIR_STEP;
                    camera = camera.norm_phi();
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Right)) => {
                    camera.phi = camera.phi + CAM_DIR_STEP;
                    camera = camera.norm_phi();
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Up)) => {
                    camera.theta = camera.theta + CAM_DIR_STEP;
                    camera = camera.norm_theta();
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Down)) => {
                    camera.theta = camera.theta - CAM_DIR_STEP;
                    camera = camera.norm_theta();
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Key1)) => {
                    flip(&mut stones, 1);
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Key2)) => {
                    flip(&mut stones, 2);
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Key3)) => {
                    flip(&mut stones, 3);
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Key4)) => {
                    flip(&mut stones, 4);
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Key5)) => {
                    flip(&mut stones, 5);
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Key6)) => {
                    flip(&mut stones, 6);
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Key7)) => {
                    flip(&mut stones, 7);
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Key8)) => {
                    flip(&mut stones, 8);
                },
                glium::glutin::Event::KeyboardInput(glium::glutin::ElementState::Pressed, _, Some(glium::glutin::VirtualKeyCode::Key9)) => {
                    flip(&mut stones, 9);
                },

                _ => {}
            }
        }
        */
}

//fn flip(set: &mut BitSet, value: usize) {
//    if set.contains(&value) { set.remove(&value); } else { set.insert(value); }
//}
