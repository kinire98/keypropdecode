use std::{
    error,
    fmt::{Debug, Display}
};

pub type Result<T> = std::result::Result<T, self::Error>;

#[derive(PartialEq)]
pub struct Error {
    pub kind: ErrorKind,
}
#[derive(PartialEq)]
pub enum ErrorKind {
    FileNotFound,
    ConflictingFlags(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::ConflictingFlags(msg) => write!(
                f,
                "An element cannot be an archive and a directory at the same time: {}",
                msg
            ),
            ErrorKind::FileNotFound => write!(f, "The file path you provided was not found"),
        }
    }
}
impl Debug for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.kind {
            ErrorKind::ConflictingFlags(msg) => write!(
                f,
                "An element cannot be an archive and a directory at the same time: {}",
                msg
            ),
            ErrorKind::FileNotFound => write!(f, "The file path you provided was not found"),
        }
    }
}

impl error::Error for Error {
    fn cause(&self) -> Option<&dyn error::Error> {
        Some(self)
    }
}
