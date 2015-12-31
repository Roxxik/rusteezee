use glium_text::{ self, TextSystem, FontTexture, TextDisplay };
use glium::{ Display, Surface };

use std::fmt;
use std::fs::File;
use std::path::Path;
use std::io::Error as IOError;
use std::error::Error;

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

#[derive(Debug)]
pub enum TextCreationError {
    ReadFileError(IOError),
    BuildFontTextureError,
}

impl fmt::Display for TextCreationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::TextCreationError::*;
        match *self {
            ReadFileError(ref s) =>
                write!(fmt, "{}: {}", self.description(), s),
            BuildFontTextureError =>
                write!(fmt, "{}", self.description()),
        }
    }
}

impl Error for TextCreationError {
    fn description(&self) -> &str {
        use self::TextCreationError::*;
        match *self {
            ReadFileError(_) =>
                "Could not read file",
            BuildFontTextureError =>
                "Error while building the font texture",
        }
    }

    #[inline]
    fn cause(&self) -> Option<&Error> {
        use self::TextCreationError::*;
        match *self {
            ReadFileError(ref s) => Some(s),
            BuildFontTextureError => None,
        }
    }
}

impl From<IOError> for TextCreationError {
    fn from(err: IOError) -> Self {
        use self::TextCreationError::*;
        ReadFileError(err)
    }
}

impl From<()> for TextCreationError {
    fn from(_err: ()) -> Self {
        use self::TextCreationError::*;
        BuildFontTextureError
    }
}


impl Text {
    pub fn new(display: &Display, font_path: &str) -> Result<Text, TextCreationError> {
        let file = try!(File::open(&Path::new(font_path)));
        let font = try!(FontTexture::new(display, file, 24));

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
