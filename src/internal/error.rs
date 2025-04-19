use ParserError::{FileNotFound, InvalidPropertiesFormat, UnsupportedFileFormat};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum ParserError {
    UnsupportedFileFormat,
    FileNotFound,
    InvalidPropertiesFormat,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FileNotFound => fmt::Display::fmt(&"File not found", f),
            UnsupportedFileFormat => fmt::Display::fmt(&"Unsupported file format", f),
            InvalidPropertiesFormat => fmt::Display::fmt(&"Invalid properties", f),
        }
    }
}

impl Error for ParserError {}
