use std::{error::Error, fmt::Display, num::ParseIntError};

/// The different types of syntax error that can occur
#[derive(Debug)]
pub enum SyntaxError {
    WrongLineNumber(usize),
    ParseIntError(ParseIntError),
}

impl Display for SyntaxError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SyntaxError::WrongLineNumber(line) => write!(f, "Wrong line number at line {line}"),
            SyntaxError::ParseIntError(e) => e.fmt(f),
        }
    }
}

impl Error for SyntaxError {}

impl From<ParseIntError> for SyntaxError {
    fn from(value: ParseIntError) -> Self {
        SyntaxError::ParseIntError(value)
    }
}
