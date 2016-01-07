use std::error::Error;
use std::io::Error as IOError;
use std::fmt;

use glium::GliumCreationError;
use glium::program::ProgramCreationError;
use glium::texture::TextureCreationError;
use glium::framebuffer::RenderBufferCreationError;
use glium::framebuffer::ValidationError;

#[derive(Debug)]
pub enum RendererCreationError<T> {
    ContextCreationError(GliumCreationError<T>),
    ProgramCreationError(ProgramCreationError),
    TextCreationError(TextCreationError),
    PickerCreationError(PickerCreationError),
}

impl<T: Error> fmt::Display for RendererCreationError<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::RendererCreationError::*;
        match *self {
            ContextCreationError(ref s) =>
                write!(fmt, "{}: {}", self.description(), s),
            ProgramCreationError(ref s) =>
                write!(fmt, "{}: {}", self.description(), s),
            TextCreationError(ref s) =>
                write!(fmt, "{}: {}", self.description(), s),
            PickerCreationError(ref s) =>
                write!(fmt, "{}: {}", self.description(), s),
        }
    }
}

impl<T: Error> Error for RendererCreationError<T> {
    fn description(&self) -> &str {
        use self::RendererCreationError::*;
        match *self {
            ContextCreationError(_) =>
                "Error while creating the Render Context",
            ProgramCreationError(_) =>
                "Error while compiling the Shader",
            TextCreationError(_) =>
                "Error while creating the Fontrenderer",
            PickerCreationError(_) =>
                "Error while creating the Picker",
        }
    }

    #[inline]
    fn cause(&self) -> Option<&Error> {
        use self::RendererCreationError::*;
        match *self {
            ContextCreationError(ref s) => Some(s),
            ProgramCreationError(ref s) => Some(s),
            TextCreationError(ref s) => Some(s),
            PickerCreationError(ref s) => Some(s),
        }
    }
}

impl<T: Error> From<GliumCreationError<T>> for RendererCreationError<T> {
    fn from(err: GliumCreationError<T>) -> Self {
        RendererCreationError::ContextCreationError(err)
    }
}

impl<T: Error> From<ProgramCreationError> for RendererCreationError<T> {
    fn from(err: ProgramCreationError) -> Self {
        RendererCreationError::ProgramCreationError(err)
    }
}

impl<T: Error> From<TextCreationError> for RendererCreationError<T> {
    fn from(err: TextCreationError) -> Self {
        RendererCreationError::TextCreationError(err)
    }
}

impl<T: Error> From<PickerCreationError> for RendererCreationError<T> {
    fn from(err: PickerCreationError) -> Self {
        RendererCreationError::PickerCreationError(err)
    }
}

#[derive (Debug)]
pub enum PickerCreationError {
    TextureCreationError(TextureCreationError),
    RenderBufferCreationError(RenderBufferCreationError),
    ProgramCreationError(ProgramCreationError),
    ValidationError(ValidationError),
}

impl fmt::Display for PickerCreationError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::PickerCreationError::*;
        match *self {
            TextureCreationError(ref s) =>
                write!(fmt, "{}: {}", self.description(), s),
            RenderBufferCreationError(ref s) =>
                write!(fmt, "{}: {}", self.description(), s),
            ProgramCreationError(ref s) =>
                write!(fmt, "{}: {}", self.description(), s),
            ValidationError(ref s) =>
                write!(fmt, "{}: {}", self.description(), s),
        }
    }
}

impl Error for PickerCreationError {
    fn description(&self) -> &str {
        use self::PickerCreationError::*;
        match *self {
            TextureCreationError(_) =>
                "Could not create texture",
            RenderBufferCreationError(_) =>
                "Could not create render buffer",
            ProgramCreationError(_) =>
                "Could not create program",
            ValidationError(_) =>
                "Could not validate framebuffer",
        }
    }

    #[inline]
    fn cause(&self) -> Option<&Error> {
        use self::PickerCreationError::*;
        match *self {
            TextureCreationError(ref s) => Some(s),
            RenderBufferCreationError(ref s) => Some(s),
            ProgramCreationError(ref s) => Some(s),
            ValidationError(ref s) => Some(s),
        }
    }
}

impl From<TextureCreationError> for PickerCreationError {
    fn from(err: TextureCreationError) -> Self {
        PickerCreationError::TextureCreationError(err)
    }
}

impl From<RenderBufferCreationError> for PickerCreationError {
    fn from(err: RenderBufferCreationError) -> Self {
        PickerCreationError::RenderBufferCreationError(err)
    }
}

impl From<ProgramCreationError> for PickerCreationError {
    fn from(err: ProgramCreationError) -> Self {
        PickerCreationError::ProgramCreationError(err)
    }
}

impl From<ValidationError> for PickerCreationError {
    fn from(err: ValidationError) -> Self {
        PickerCreationError::ValidationError(err)
    }
}

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
        TextCreationError::ReadFileError(err)
    }
}

impl From<()> for TextCreationError {
    fn from(_err: ()) -> Self {
        TextCreationError::BuildFontTextureError
    }
}
