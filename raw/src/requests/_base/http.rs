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
pub enum Body {
    Empty,
    Json(Vec<u8>),
    #[doc(hidden)]
    __Nonexhaustive,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HttpRequest {
    pub url: RequestUrl,
    pub method: Method,
    pub body: Body,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct HttpResponse {
    pub body: Option<Vec<u8>>,
}
