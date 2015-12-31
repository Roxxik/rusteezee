use std::error::Error;
use std::io::Error as IOError;
use std::fmt;

use glium::GliumCreationError;
use glium::program::ProgramCreationError;

#[derive(Debug)]
pub enum RenderCreationError<T> {
    ContextCreationError(GliumCreationError<T>),
    ProgramCreationError(ProgramCreationError),
    TextCreationError(TextCreationError),
}

impl<T: Error> fmt::Display for RenderCreationError<T> {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        use self::RenderCreationError::*;
        match *self {
            ContextCreationError(ref s) =>
                write!(fmt, "{}: {}", self.description(), s),
            ProgramCreationError(ref s) =>
                write!(fmt, "{}: {}", self.description(), s),
            TextCreationError(ref s) =>
                write!(fmt, "{}: {}", self.description(), s),
        }
    }
}

impl<T: Error> Error for RenderCreationError<T> {
    fn description(&self) -> &str {
        use self::RenderCreationError::*;
        match *self {
            ContextCreationError(_) =>
                "Error while creating the Render Context",
            ProgramCreationError(_) =>
                "Error while compiling the Shader",
            TextCreationError(_) =>
                "Error while creating the Fontrenderer",
        }
    }

    #[inline]
    fn cause(&self) -> Option<&Error> {
        use self::RenderCreationError::*;
        match *self {
            ContextCreationError(ref s) => Some(s),
            ProgramCreationError(ref s) => Some(s),
            TextCreationError(ref s) => Some(s),
        }
    }
}

impl<T: Error> From<GliumCreationError<T>> for RenderCreationError<T> {
    fn from(err: GliumCreationError<T>) -> Self {
        use self::RenderCreationError::*;
        ContextCreationError(err)
    }
}

impl<T: Error> From<ProgramCreationError> for RenderCreationError<T> {
    fn from(err: ProgramCreationError) -> Self {
        use self::RenderCreationError::*;
        ProgramCreationError(err)
    }
}

impl<T: Error> From<TextCreationError> for RenderCreationError<T> {
    fn from(err: TextCreationError) -> Self {
        use self::RenderCreationError::*;
        TextCreationError(err)
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
