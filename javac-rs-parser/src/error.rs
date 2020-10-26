use std::error::Error;
use std::fmt::Formatter;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ParseError {
    description: String,
}

impl ParseError {
    pub fn new(description: String) -> Self {
        ParseError { description }
    }
}

impl Error for ParseError {}

impl std::fmt::Display for ParseError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "Parse error: {}", self.description)
    }
}

impl From<std::num::ParseIntError> for ParseError {
    fn from(error: std::num::ParseIntError) -> Self {
        ParseError::new(error.to_string())
    }
}

impl From<std::num::ParseFloatError> for ParseError {
    fn from(error: std::num::ParseFloatError) -> Self {
        ParseError::new(error.to_string())
    }
}
