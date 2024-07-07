use std::fmt;

#[derive(Debug)]
pub enum WaccrsError {
    LexerError(String),
    GenericError(String),
    IOError(std::io::Error),
    Utf8Error(std::str::Utf8Error),
}

impl std::error::Error for WaccrsError {}

impl fmt::Display for WaccrsError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            WaccrsError::LexerError(msg) => write!(f, "Lexer Error: {}", msg),
            WaccrsError::GenericError(s) => write!(f, "Error: {}", s),
            WaccrsError::IOError(e) => write!(f, "IO Error: {}", e),
            WaccrsError::Utf8Error(e) => write!(f, "UTF8 Error: {}", e),
        }
    }
}

impl From<&str> for WaccrsError {
    fn from(value: &str) -> Self {
        WaccrsError::GenericError(value.to_owned())
    }
}

impl From<std::io::Error> for WaccrsError {
    fn from(value: std::io::Error) -> Self {
        WaccrsError::IOError(value)
    }
}

impl From<std::str::Utf8Error> for WaccrsError {
    fn from(value: std::str::Utf8Error) -> Self {
        WaccrsError::Utf8Error(value)
    }
}

pub type Result<T> = core::result::Result<T, WaccrsError>;
