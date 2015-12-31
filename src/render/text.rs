use glium_text::{ TextSystem, FontTexture };

#[derive(Debug)]
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
    pub fn new(display: &glium::DisplayBuild::Facade, font_path: &str) -> Option<Text> {
        let opt_file = std::fs::File::open(&std::path::Path::new(font_path));
        let opt_font = opt_file.map(|f| FontTexture::new(display, f, 24);
        opt_font.map(|f| Text {
            system: glium_text::TextSystem::new(display),
            font: f,
        }
    }

    pub fn draw(surface: &mut glium::Surface, text: &str, color: (f32, f32, f32)) {
        let text = glium_text::TextDisplay::new(&self.system, &self.font, text);
        glium_text::draw(&text, &system, &mut target, MATRIX, color);
    }
}
