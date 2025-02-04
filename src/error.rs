use std::{
    error,
    fmt,
    io,
    result,
};

#[derive(Debug)]
pub enum Error {
    FileCreation(io::Error),
    FileWrite(io::Error),

    InvalidDimension(&'static str),
    Custom(&'static str),
}

pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileCreation(err) => writeln!(f, "File Creation Failure: {err}"),
            Self::FileWrite(err) => writeln!(f, "File Write Failure: {err}"),
            Self::InvalidDimension(msg) => writeln!(f, "Invalid Dimensions: {msg}"),
            Self::Custom(msg) => writeln!(f, "{msg}"),
        }
    }
}

impl error::Error for Error {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        match self {
            Self::FileCreation(err) => Some(err),
            Self::FileWrite(err) => Some(err),
            _ => None,
        }
    }
}

impl From<io::Error> for Error {
    fn from(err: io::Error) -> Self {
        match err.kind() {
            io::ErrorKind::NotFound
            | io::ErrorKind::AlreadyExists
            | io::ErrorKind::PermissionDenied => Self::FileCreation(err),
            _ => Self::FileWrite(err),
        }
    }
}
