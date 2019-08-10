//! Connector with hyper backend.

use std::str::FromStr;

use futures::TryStreamExt;
use hyper::client::Client;
use hyper::header::CONTENT_TYPE;
use hyper::{Method, Request, Uri};
use hyper_tls::HttpsConnector;

use telegram_bot_raw::{Body as TelegramBody, HttpRequest, HttpResponse, Method as TelegramMethod};

use crate::errors::Error;

pub(crate) async fn request(token: &str, req: HttpRequest) -> Result<HttpResponse, Error> {
    let uri = Uri::from_str(&req.url.url(token))?;

    let method = match req.method {
        TelegramMethod::Get => Method::GET,
        TelegramMethod::Post => Method::POST,
    };

    let mut http_request = Request::builder();
    http_request.method(method).uri(uri);

    let request = match req.body {
        TelegramBody::Empty => http_request.body(Into::<hyper::Body>::into(vec![])),
        TelegramBody::Json(body) => {
            http_request
                .headers_mut()
                .map(|headers| headers.insert(CONTENT_TYPE, "application/json".parse().unwrap()));
            http_request.body(Into::<hyper::Body>::into(body))
        }
        body => panic!("Unknown body type {:?}", body),
    };

    let connector = HttpsConnector::new(1).map_err(|err| {
        ::std::io::Error::new(::std::io::ErrorKind::Other, format!("tls error: {}", err))
    })?;

    let client = Client::builder().build(connector);

    let response = client.request(request.unwrap()).await?;

    let whole_chunk = response.into_body().try_concat().await;

    let body = whole_chunk
        .iter()
        .fold(vec![], |mut acc, chunk| -> Vec<u8> {
            acc.extend_from_slice(&chunk);
            acc
        });

    Ok(HttpResponse { body: Some(body) })
}
