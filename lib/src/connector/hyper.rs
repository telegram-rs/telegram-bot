//! Connector with hyper backend.

use std::fmt;
use std::rc::Rc;
use std::str::FromStr;

use futures::future::result;
use futures::{Future, Stream};
use hyper;
use hyper::client::connect::Connect;
use hyper::client::Client;
use hyper::{Body, Request};
use hyper::{Method, Uri};
use hyper_tls::HttpsConnector;

use telegram_bot_raw::{Body as TelegramBody, HttpRequest, HttpResponse, Method as TelegramMethod};

use errors::Error;
use future::{NewTelegramFuture, TelegramFuture};

use super::_base::Connector;

/// This connector uses `hyper` backend.
pub struct HyperConnector<C> {
    inner: Rc<Client<C>>,
}

impl<C> fmt::Debug for HyperConnector<C> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        "hyper connector".fmt(formatter)
    }
}

impl<C> HyperConnector<C> {
    pub fn new(client: Client<C>) -> Self {
        HyperConnector {
            inner: Rc::new(client),
        }
    }
}

impl<C: Connect + 'static> Connector for HyperConnector<C> {
    fn request(&self, token: &str, req: HttpRequest) -> TelegramFuture<HttpResponse> {
        let uri = result(Uri::from_str(&req.url.url(token))).map_err(From::from);

        let client = self.inner.clone();
        let request = uri.and_then(move |uri| {
            let method = match req.method {
                TelegramMethod::Get => Method::GET,
                TelegramMethod::Post => Method::POST,
            };
            let mut builder = Request::builder();
            let http_request = builder.method(method).uri(uri);

            let http_request = match req.body {
                TelegramBody::Empty => http_request.body(Body::empty()).unwrap(),
                TelegramBody::Json(body) => {
                    let mut r = http_request.body(body.into()).unwrap();
                    r.headers_mut().insert(
                        hyper::header::CONTENT_TYPE,
                        "application/json".parse().unwrap(),
                    );
                    r
                }
                body => panic!("Unknown body type {:?}", body),
            };

            client.request(http_request).map_err(From::from)
        });

        let future = request.and_then(move |response| {
            response.into_body().map_err(From::from).fold(
                vec![],
                |mut result, chunk| -> Result<Vec<u8>, Error> {
                    result.extend_from_slice(&chunk);
                    Ok(result)
                },
            )
        });

        let future = future.and_then(|body| Ok(HttpResponse { body: Some(body) }));

        TelegramFuture::new(Box::new(future))
    }
}

/// Returns default hyper connector. Uses one resolve thread and `HttpsConnector`.
pub fn default_connector() -> Result<Box<Connector>, Error> {
    let connector = HttpsConnector::new(1).map_err(|err| {
        ::std::io::Error::new(::std::io::ErrorKind::Other, format!("tls error: {}", err))
    })?;
    let client = Client::builder().build(connector);
    Ok(Box::new(HyperConnector::new(client)))
}
