use {
    indicatif::style::TemplateError,
    std::{
        error,
        fmt,
        io,
        result,
    },
};

#[derive(Debug)]
pub enum Error {
    FileCreation(io::Error),
    FileWrite(io::Error),
    ProgressBarStyle(TemplateError),
    InvalidCamera(&'static str),
    InvalidDimension(&'static str),
    InvalidScene(&'static str),
    Custom(&'static str),
}

/// Custom Result for this specific project.
pub type Result<T> = result::Result<T, Error>;

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::FileCreation(err) => {
                writeln!(f, "File Creation Failure: {err}.")
            }
            Self::FileWrite(err) => {
                writeln!(f, "File Write Failure: {err}.")
            }
            Self::ProgressBarStyle(err) => {
                writeln!(f, "Progress Bar Style: {err}.")
            }
            Self::InvalidCamera(msg) => {
                writeln!(f, "Invalid Camera: {msg}.")
            }
            Self::InvalidDimension(msg) => {
                writeln!(f, "Invalid Dimensions: {msg}.")
            }
            Self::InvalidScene(msg) => {
                writeln!(f, "Invalid Scene: {msg}.")
            }
            Self::Custom(msg) => writeln!(f, "Error: {msg}."),
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
    fn from(value: io::Error) -> Self {
        match value.kind() {
            io::ErrorKind::NotFound
            | io::ErrorKind::AlreadyExists
            | io::ErrorKind::PermissionDenied => Self::FileCreation(value),
            _ => Self::FileWrite(value),
        }
    }
}

impl From<TemplateError> for Error {
    fn from(value: TemplateError) -> Self { Error::ProgressBarStyle(value) }
}
