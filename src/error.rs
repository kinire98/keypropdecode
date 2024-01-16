use std::{
    error,
    fmt::{Debug, Display},
};

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
}
#[derive(PartialEq)]
pub enum ErrorKind {
    FileNotFound,
    NotAFile(String),
    Other(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::FileNotFound => write!(f, "The file path you provided was not found"),
            ErrorKind::NotAFile(msg) => write!(f, "Not marked as a file: {}", msg),
            ErrorKind::Other(msg) => write!(f, "Undefined error: {}", msg),
        }
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::FileNotFound => write!(f, "The file path you provided was not found"),
            ErrorKind::NotAFile(msg) => write!(f, "Not marked as a file: {}", msg),
            ErrorKind::Other(msg) => write!(f, "Undefined error: {}", msg),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self)
    }
}
