use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum MapperError {
    ParseError(String),
    InvalidContent(String),
    MissingData(String),
}

impl fmt::Display for MapperError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            MapperError::ParseError(msg) => write!(f, "Parse error: {msg}"),
            MapperError::InvalidContent(msg) => write!(f, "Invalid content: {msg}"),
            MapperError::MissingData(msg) => write!(f, "Missing data: {msg}"),
        }
    }
}

impl Error for MapperError {}
