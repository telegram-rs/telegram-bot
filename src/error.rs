use std::fmt;
use rustc_serialize::json;
/// Telegram-Bot Result
pub type Result<T> = ::std::result::Result<T, Error>;

/// Telegram-Bot Error: Anything that may fail (HTTP, JSON, ...)
#[derive(Debug)]
pub enum Error {
    /// HTTP related error
    Http(::hyper::error::Error),
    /// IO related error (mainly reading the http result)
    Io(::std::io::Error),
    /// Error while decoding JSON data
    JsonDecode(json::DecoderError),
    /// Error while encoding JSON data
    JsonEncode(json::EncoderError),
    Api(String),
    InvalidState(String),
    UserInterrupt,
}

impl ::std::error::Error for Error {
    fn description(&self) -> &str {
        match *self {
            Error::Http(ref e) => e.description(),
            Error::Io(ref e) => e.description(),
            Error::JsonDecode(ref e) => e.description(),
            Error::JsonEncode(ref e) => e.description(),
            Error::Api(ref s) => &s,
            Error::InvalidState(ref s) => &s,
            Error::UserInterrupt => "user interrupt",
        }
    }
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Error::Http(ref e) => e.fmt(f),
            Error::Io(ref e) => e.fmt(f),
            Error::JsonDecode(ref e) => e.fmt(f),
            Error::JsonEncode(ref e) => e.fmt(f),
            Error::Api(ref s) => s.fmt(f),
            Error::InvalidState(ref s) => s.fmt(f),
            Error::UserInterrupt => "user interrupt".fmt(f),
        }
    }
}

macro_rules! from_impl {
    ($ty:path, $variant:ident) => (
        impl From<$ty> for Error {
            fn from(e: $ty) -> Self {
                Error::$variant(e)
            }
        }
    )
}

from_impl!(::hyper::error::Error, Http);
from_impl!(::std::io::Error, Io);
from_impl!(json::DecoderError, JsonDecode);
from_impl!(json::EncoderError, JsonEncode);
