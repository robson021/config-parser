use std::error::Error;
use std::fmt::{Display, Formatter};

#[derive(Debug)]
pub enum ParserError {
    UnsupportedFileFormat,
    FileNotFound,
}

impl Display for ParserError {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&"Unsupported file format", f)
    }
}

impl Error for ParserError {}