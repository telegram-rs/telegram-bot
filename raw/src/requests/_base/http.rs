use std::fmt;

use crate::url::telegram_api_url;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum RequestUrl {
    Method(&'static str),
}

impl RequestUrl {
    pub fn method(method: &'static str) -> Self {
        RequestUrl::Method(method)
    }

    pub fn url(&self, token: &str) -> String {
        match self {
            &RequestUrl::Method(method) => format!("{}bot{}/{}", telegram_api_url(), token, method),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Method {
    Get,
    Post,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum MultipartValue {
    Text(String),
    File {
        path: String,
    },
    Data {
        file_name: Option<String>,
        data: Vec<u8>,
    },
}

pub type Multipart = Vec<(String, MultipartValue)>;

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum Body {
    Empty,
    Multipart(Multipart),
    Json(String),
    #[doc(hidden)]
    __Nonexhaustive,
}

impl fmt::Display for Body {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        match self {
            Body::Empty => "<empty body>".fmt(f),
            Body::Multipart(_) => unimplemented!("FIXME(knsd)"),
            Body::Json(s) => s.fmt(f),
            Body::__Nonexhaustive => unreachable!(),
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HttpRequest {
    pub url: RequestUrl,
    pub method: Method,
    pub body: Body,
}

impl HttpRequest {
    pub fn name(&self) -> &'static str {
        match self.url {
            RequestUrl::Method(method) => method,
        }
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HttpResponse {
    pub body: Option<Vec<u8>>,
}
