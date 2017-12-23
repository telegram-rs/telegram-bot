//! Connector with hyper backend.

use std::fmt;
use std::str::FromStr;
use std::rc::Rc;

use futures::{Future, Stream};
use futures::future::result;
use hyper;
use hyper::{Method, Uri};
use hyper::client::{Client, Connect};
use hyper::header::ContentType;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Handle;

use telegram_bot_raw::{HttpRequest, HttpResponse, Method as TelegramMethod, Body as TelegramBody};

use errors::Error;
use future::{TelegramFuture, NewTelegramFuture};

use super::_base::Connector;

/// This connector uses `hyper` backend.
pub struct HyperConnector<C> {
    inner: Rc<Client<C>>
}

impl<C> fmt::Debug for HyperConnector<C> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        "hyper connector".fmt(formatter)
    }
}

impl<C> HyperConnector<C> {
    pub fn new(client: Client<C>) -> Self {
        HyperConnector {
            inner: Rc::new(client)
        }
    }
}

impl<C: Connect> Connector for HyperConnector<C> {
    fn request(&self, token: &str, req: HttpRequest) -> TelegramFuture<HttpResponse> {
        let uri = result(Uri::from_str(&req.url.url(token))).map_err(From::from);

        let client = self.inner.clone();
        let request = uri.and_then(move |uri| {
            let method = match req.method {
                TelegramMethod::Get => Method::Get,
                TelegramMethod::Post => Method::Post,
            };
            let mut http_request = hyper::client::Request::new(method, uri);

            match req.body {
                TelegramBody::Empty => (),
                TelegramBody::Json(body) => {
                    http_request.set_body(body);
                    http_request.headers_mut().set(ContentType::json());
                }
                body => panic!("Unknown body type {:?}", body)
            }

            client.request(http_request).map_err(From::from)
        });

        let future = request.and_then(move |response| {
            response.body().map_err(From::from)
                .fold(vec![], |mut result, chunk| -> Result<Vec<u8>, Error> {
                    result.extend_from_slice(&chunk);
                    Ok(result)
            })
        });

        let future = future.and_then(|body| {
            Ok(HttpResponse {
                body: Some(body),
            })
        });

        TelegramFuture::new(Box::new(future))
    }
}

/// Returns default hyper connector. Uses one resolve thread and `HttpsConnector`.
pub fn default_connector(handle: &Handle) -> Result<Box<Connector>, Error> {
    let connector = HttpsConnector::new(1, handle).map_err(|err| {
        ::std::io::Error::new(::std::io::ErrorKind::Other, format!("tls error: {}", err))
    })?;
    let config = Client::configure().connector(connector);
    Ok(Box::new(HyperConnector::new(config.build(handle))))
}
