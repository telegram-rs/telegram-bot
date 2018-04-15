//! Connector with hyper backend.

use std::fmt;
use std::rc::Rc;
use std::str::FromStr;

use futures::future::result;
use futures::{Future, Stream};
use hyper;
use hyper::client::{Client, Connect};
use hyper::header::ContentType;
use hyper::{Method, Uri};
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Handle;

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

mod multipart {
    use hyper::client::Request;
    use multipart::client::lazy::*;
    use std::io::Cursor;
    use telegram_bot_raw::MultipartValue;

    pub struct Adapter<'d> {
        prepared: PreparedFields<'d>,
    }

    impl<'d> From<Vec<(String, MultipartValue)>> for Adapter<'d> {
        fn from(fields: Vec<(String, MultipartValue)>) -> Self {
            let mut part = Multipart::new();
            for (key, value) in fields.into_iter() {
                match value {
                    MultipartValue::Text(text) => {
                        part.add_text(key, text);
                    }
                    MultipartValue::File { path } => {
                        part.add_file(key, path);
                    }
                    MultipartValue::Data { file_name, data } => {
                        part.add_stream(key, Cursor::new(data), file_name, None);
                    }
                }
            }
            Self {
                prepared: part.prepare().unwrap(),
            }
        }
    }

    impl<'d> Adapter<'d> {
        pub fn set_header(&mut self, req: &mut Request) {
            use hyper::header::ContentLength;

            let boundary = self.prepared.boundary();
            req.headers_mut().set_raw(
                "Content-Type",
                vec![
                    format!("multipart/form-data;boundary={bound}", bound = boundary).into_bytes(),
                ],
            );
            if let Some(len) = self.prepared.content_len() {
                req.headers_mut().set(ContentLength(len));
            }
        }

        pub fn set_body(&mut self, req: &mut Request) {
            use std::io::Read;

            let mut bytes = Vec::new();
            self.prepared.read_to_end(&mut bytes).unwrap();
            req.set_body(bytes);
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
                TelegramBody::Multipart(parts) => {
                    let mut adapter = multipart::Adapter::from(parts);
                    adapter.set_header(&mut http_request);
                    adapter.set_body(&mut http_request);
                }
                body => panic!("Unknown body type {:?}", body),
            }

            client.request(http_request).from_err()
        });

        let future = request.and_then(move |response| {
            response.body().map_err(From::from).fold(
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
pub fn default_connector(handle: &Handle) -> Result<Box<Connector>, Error> {
    let connector = HttpsConnector::new(1, handle).map_err(|err| {
        ::std::io::Error::new(::std::io::ErrorKind::Other, format!("tls error: {}", err))
    })?;
    let config = Client::configure().connector(connector);
    Ok(Box::new(HyperConnector::new(config.build(handle))))
}
