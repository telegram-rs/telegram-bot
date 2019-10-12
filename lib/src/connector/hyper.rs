use std::io::Read;
use std::pin::Pin;
use std::str::FromStr;

use futures::{Future, FutureExt, TryStreamExt};
use hyper::{
    client::{connect::Connect, Client},
    header::{CONTENT_LENGTH, CONTENT_TYPE},
    Method, Request, Uri,
};
use hyper_tls::HttpsConnector;
use telegram_bot_raw::{Body as TelegramBody, HttpRequest, HttpResponse, Method as TelegramMethod};

use crate::errors::Error;

use super::Connector;

#[derive(Debug)]
pub struct HyperConnector<C>(Client<C>);

impl<C> HyperConnector<C> {
    pub fn new(client: Client<C>) -> Self {
        HyperConnector(client)
    }
}

impl<C: Connect + std::fmt::Debug + 'static> Connector for HyperConnector<C> {
    fn request(
        &self,
        token: &str,
        req: HttpRequest,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, Error>> + Send>> {
        let uri = Uri::from_str(&req.url.url(token));
        let client = self.0.clone();

        let future = async move {
            let uri = uri?;

            let method = match req.method {
                TelegramMethod::Get => Method::GET,
                TelegramMethod::Post => Method::POST,
            };

            let mut http_request = Request::builder();
            http_request.method(method).uri(uri);

            let request = match req.body {
                TelegramBody::Empty => http_request.body(Into::<hyper::Body>::into(vec![])),
                TelegramBody::Json(body) => {
                    http_request.headers_mut().map(|headers| {
                        headers.insert(CONTENT_TYPE, "application/json".parse().unwrap())
                    });
                    http_request.body(Into::<hyper::Body>::into(body))
                }
                TelegramBody::Multipart(parts) => {
                    let mut adapter = multipart::Adapter::from(parts);
                    let boundary = adapter.prepared.boundary();

                    http_request.headers_mut().map(|headers| {
                        headers.insert(
                            CONTENT_TYPE,
                            format!("multipart/form-data;boundary={bound}", bound = boundary)
                                .parse()
                                .unwrap(),
                        );
                    });

                    let mut bytes = Vec::new();
                    adapter.prepared.read_to_end(&mut bytes).unwrap();
                    http_request.body(bytes.into())
                }
                body => panic!("Unknown body type {:?}", body),
            };

            let response = client.request(request.unwrap()).await?;
            let whole_chunk = response.into_body().try_concat().await;

            let body = whole_chunk
                .iter()
                .fold(vec![], |mut acc, chunk| -> Vec<u8> {
                    acc.extend_from_slice(&chunk);
                    acc
                });

            Ok::<HttpResponse, Error>(HttpResponse { body: Some(body) })
        };

        future.boxed()
    }
}

pub fn default_connector() -> Result<Box<dyn Connector>, Error> {
    let connector = HttpsConnector::new().map_err(|err| {
        ::std::io::Error::new(::std::io::ErrorKind::Other, format!("tls error: {}", err))
    })?;
    Ok(Box::new(HyperConnector::new(
        Client::builder().build(connector),
    )))
}

mod multipart {
    use std::io::Cursor;

    use multipart::client::lazy::*;
    use telegram_bot_raw::MultipartValue;

    pub struct Adapter<'d> {
        pub prepared: PreparedFields<'d>,
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
}
