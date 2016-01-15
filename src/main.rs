#![allow(dead_code)]

#[macro_use]
extern crate glium;
extern crate glium_text;
extern crate cgmath;
extern crate image;
extern crate bit_set;

mod event;
mod logic;
mod render;
mod types;

use render::renderer::Renderer;

fn main() {
    match Renderer::new() {
        Ok(r)  => { r.game_loop(); },
        Err(r) => { println!("{}", r); },
    }
}
