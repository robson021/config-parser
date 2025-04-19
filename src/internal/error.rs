use ParserError::{FileNotFound, InvalidPropertiesFormat, UnsupportedFileFormat};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub(crate) enum ParserError {
    UnsupportedFileFormat,
    FileNotFound(String),
    InvalidPropertiesFormat(String),
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match self {
            FileNotFound(path) => format!("File not found: {}", path).fmt(f),
            UnsupportedFileFormat => fmt::Display::fmt(&"Unsupported file format", f),
            InvalidPropertiesFormat(reason) => {
                format!("Invalid properties. Reason: {}", reason).fmt(f)
            }
        }
    }
}

impl Error for ParserError {}
