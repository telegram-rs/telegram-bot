use std::sync::Arc;

use futures;
use futures::{Future, Stream, Poll};
use futures::future::{result};
use hyper;
use hyper::{Body, Method};
use hyper::client::Client;
use hyper::header::ContentType;
use hyper_tls::HttpsConnector;
use serde_json;
use tokio_core::reactor::Handle;
use url::Url;

use telegram_bot_raw::{Request, Response};

use errors::{Error, Result, ErrorKind};

use stream::UpdatesStream;

const TELEGRAM_URL: &'static str = "https://api.telegram.org/";

#[must_use = "futures do nothing unless polled"]
pub struct TelegramFuture<T> {
    inner: Box<Future<Item=T, Error=Error>>
}

impl<T> Future for TelegramFuture<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

#[derive(Clone)]
pub struct Api {
    inner: Arc<ApiInner>,
}

#[derive(Clone)]
struct ApiInner {
    token: String,
    client: Client<HttpsConnector>,
    handle: Handle,
}

impl Api {
    pub fn from_token(handle: &Handle, token: &str) -> Result<Self> {
        let connector = HttpsConnector::new(1, handle);
        let config = Client::configure().connector(connector);

        Ok(Api {
            inner: Arc::new(ApiInner {
                token: token.to_string(),
                client: config.build(handle),
                handle: handle.clone(),
            }),
        })
    }

    pub fn stream(&self) -> UpdatesStream {
        UpdatesStream::new(self)
    }

    pub fn spawn<Req>(&self, request: Req)
        where Req: Request + 'static, <Req as Request>::Response: ::std::marker::Send + 'static {

        self.inner.handle.spawn(self.send(request).then(|_| Ok(())))
    }

    pub fn send<Req>(&self, request: Req) -> TelegramFuture<Req::Response>
        where Req: Request + 'static, <Req as Request>::Response: ::std::marker::Send + 'static {

        let url = result(url(&self.inner.token, request.name()));
        let body = futures::lazy(move || {
            serde_json::to_vec(&request).map(Body::from)
        }).map_err(From::from);

        let api = self.clone();
        let response = url.join(body).and_then(move |(url, body)| {
            let mut http_request = hyper::client::Request::new(Method::Post, url);
            http_request.set_body(body);
            http_request.headers_mut().set(ContentType::json());

            api.inner.client.request(http_request).map_err(From::from)
        });

        let bytes = response.and_then(|response| {
            response.body().map_err(From::from)
                .fold(vec![], |mut result, chunk| -> Result<Vec<u8>> {
                    result.extend_from_slice(&chunk);
                    Ok(result)
            })
        });

        let future = bytes.and_then(|bytes| {
            result(serde_json::from_slice(&bytes).map_err(From::from).and_then(|value| {
                match value {
                    Response::Success {result} => Ok(result),
                    Response::Error { description, parameters } => {
                        Err(ErrorKind::TelegramError {
                            description: description,
                            parameters: parameters
                        }.into())
                    },
                }
            }))
        });

        TelegramFuture {
            inner: Box::new(future)
        }
    }
}

fn url(token: &str, method: &str) -> Result<Url> {
    Url::parse(&format!("{}bot{}/{}", TELEGRAM_URL, token, method)).map_err(From::from)
}
