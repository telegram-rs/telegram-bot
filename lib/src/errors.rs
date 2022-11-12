use std::error;
use std::fmt;

#[derive(Debug)]
pub struct Error(ErrorKind);

#[derive(Debug)]
pub enum ErrorKind {
    Raw(telegram_bot_raw::Error),
    #[cfg(feature = "connector-hyper")]
    Hyper(hyper::Error),
    #[cfg(feature = "connector-hyper")]
    Http(hyper::http::Error),
    Io(std::io::Error),
    InvalidMultipartFilename,
    Generic(Box<dyn std::error::Error + Send>),
}

impl ErrorKind {
    pub fn from_generic<E: std::error::Error + Send + 'static>(e: E) -> Self {
        Self::Generic(Box::new(e))
    }

    pub fn from_generic_boxed(e: Box<dyn std::error::Error + Send>) -> Self {
        Self::Generic(e)
    }
}

impl From<telegram_bot_raw::Error> for ErrorKind {
    fn from(error: telegram_bot_raw::Error) -> Self {
        ErrorKind::Raw(error)
    }
}

#[cfg(feature = "connector-hyper")]
impl From<hyper::Error> for ErrorKind {
    fn from(error: hyper::Error) -> Self {
        ErrorKind::Hyper(error)
    }
}

#[cfg(feature = "connector-hyper")]
impl From<hyper::http::Error> for ErrorKind {
    fn from(error: hyper::http::Error) -> Self {
        ErrorKind::Http(error)
    }
}

impl From<std::io::Error> for ErrorKind {
    fn from(error: std::io::Error) -> Self {
        ErrorKind::Io(error)
    }
}

impl From<ErrorKind> for Error {
    fn from(kind: ErrorKind) -> Self {
        Error(kind)
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match &self.0 {
            ErrorKind::Raw(error) => write!(f, "{}", error),
            #[cfg(feature = "connector-hyper")]
            ErrorKind::Hyper(error) => write!(f, "{}", error),
            #[cfg(feature = "connector-hyper")]
            ErrorKind::Http(error) => write!(f, "{}", error),
            ErrorKind::Io(error) => write!(f, "{}", error),
            ErrorKind::InvalidMultipartFilename => write!(f, "invalid multipart filename"),
            ErrorKind::Generic(error) => write!(f, "{}", error),
        }
    }
}

impl error::Error for Error {}
