use std::path::Path;

use bytes::Bytes;

#[derive(Debug, Clone, PartialEq, PartialOrd, Ord, Eq, Hash)]
pub struct Text(Bytes);

impl Text {
    pub fn as_str(&self) -> &str {
        unsafe { std::str::from_utf8_unchecked(self.0.as_ref()) }
    }
}

impl From<String> for Text {
    fn from(value: String) -> Self {
        Text(value.into())
    }
}

impl<'a> From<&'a str> for Text {
    fn from(value: &'a str) -> Self {
        Text(value.to_owned().into())
    }
}

impl AsRef<str> for Text {
    fn as_ref(&self) -> &str {
        self.as_str()
    }
}

impl AsRef<Path> for Text {
    fn as_ref(&self) -> &Path {
        self.as_str().as_ref()
    }
}
