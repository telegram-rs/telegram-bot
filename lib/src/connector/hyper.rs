//! Connector with hyper backend.

use std::fmt;
use std::str::FromStr;
use std::rc::Rc;

use futures::{Future, Stream};
use futures::future::result;
use hyper;
use hyper::{Body, Method, Uri};
use hyper::client::{Client, Connect};
use hyper::header::ContentType;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Handle;

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
    fn post_json(&self, uri: &str, data: Vec<u8>) -> TelegramFuture<Vec<u8>> {
        let uri = result(Uri::from_str(uri)).map_err(From::from);
        let body = Body::from(data);

        let client = self.inner.clone();
        let request = uri.and_then(move |uri| {
            let mut http_request = hyper::client::Request::new(Method::Post, uri);
            http_request.set_body(body);
            http_request.headers_mut().set(ContentType::json());
            client.request(http_request).map_err(From::from)
        });

        let future = request.and_then(move |response| {
            response.body().map_err(From::from)
                .fold(vec![], |mut result, chunk| -> Result<Vec<u8>, Error> {
                    result.extend_from_slice(&chunk);
                    Ok(result)
            })
        });

        TelegramFuture::new(Box::new(future))
    }
}

/// Returns default hyper connector. Uses one resolve thread and `HttpsConnector`.
pub fn default_connector(handle: &Handle) -> Box<Connector> {
    let connector = HttpsConnector::new(1, handle);
    let config = Client::configure().connector(connector);
    Box::new(HyperConnector::new(config.build(handle)))
}
