use std::error;
use std::fmt;

use crate::types::*;

#[derive(Debug)]
pub struct Error(ErrorKind);

#[derive(Debug)]
pub(crate) enum ErrorKind {
    EmptyBody,
    TelegramError {
        description: String,
        parameters: Option<ResponseParameters>,
    },
    DetachedError(String),
    Json(::serde_json::Error),
}

impl From<::serde_json::Error> for ErrorKind {
    fn from(error: ::serde_json::Error) -> Self {
        ErrorKind::Json(error)
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
            ErrorKind::EmptyBody => write!(f, "empty body"),
            ErrorKind::TelegramError {
                description,
                parameters,
            } => {
                f.write_str(&description)?;
                if let Some(parameters) = parameters {
                    if let Some(chat_id) = parameters.migrate_to_chat_id {
                        write!(f, ", migrate to chat id: {}", chat_id)?;
                    }
                    if let Some(seconds) = parameters.retry_after {
                        write!(f, ", retry after: {}", seconds)?;
                    }
                }
                Ok(())
            }
            ErrorKind::DetachedError(s) => f.write_str(&s),
            ErrorKind::Json(error) => write!(f, "{}", error),
        }
    }
}

impl error::Error for Error {}
