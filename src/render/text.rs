use std::fs::File;
use std::path::Path;

use glium_text::{ self, TextSystem, FontTexture, TextDisplay };
use glium::{ Display, Surface };

use super::error::TextCreationError;


pub struct Text {
    system: TextSystem,
    font: FontTexture,
}

const MATRIX: [[f32; 4]; 4] = [
    [0.025, 0.0,   0.0,   0.0],
    [0.0,   0.025, 0.0,   0.0],
    [0.0,   0.0,   0.025, 0.0],
    [-1.0,  0.96,  0.0,   1.0],
];

impl Text {
    pub fn new(display: &Display, font_path: &str, font_size: u32) -> Result<Text, TextCreationError> {
        let file = try!(File::open(&Path::new(font_path)));
        let font = try!(FontTexture::new(display, file, font_size));

        Ok(Text {
            system: TextSystem::new(display),
            font: font,
        })
    }

    pub fn draw<S: Surface>(&self, surface: &mut S, text: &str, color: (f32, f32, f32, f32)) {
        let text = TextDisplay::new(&self.system, &self.font, text);
        glium_text::draw(&text, &self.system, surface, MATRIX, color);
    }
}
