//! Connector with tokio-curl backend.

use std::fmt;
use std::sync::Arc;
use std::rc::Rc;

use antidote::Mutex;
use curl::easy::{Easy, List};
use futures::Future;
use futures::future::result;
use tokio_core::reactor::Handle;
use tokio_curl::Session;

use errors::Error;
use future::{TelegramFuture, NewTelegramFuture};

use telegram_bot_raw::{HttpRequest, HttpResponse, Method, Body};

use super::_base::Connector;

/// This connector uses `tokio-curl` backend.
pub struct CurlConnector {
    inner: Rc<Session>
}

impl fmt::Debug for CurlConnector {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
        "curl connector".fmt(formatter)
    }
}

impl CurlConnector {
    pub fn new(handle: &Handle) -> Self {
        CurlConnector {
            inner: Rc::new(Session::new(handle.clone()))
        }
    }

    fn create_request(&self, token: &str, request: HttpRequest) -> Result<(Easy, Arc<Mutex<Vec<u8>>>), Error> {
        let mut handle = Easy::new();

        let url = request.url.url(token);
        handle.url(&url)?;

        match request.method {
            Method::Get => handle.get(true)?,
            Method::Post => handle.post(true)?,
        }

        match request.body {
            Body::Empty => (),
            Body::Json(body) => {
                handle.post_fields_copy(&body)?;

                let mut headers = List::new();
                headers.append(&format!("Content-Type: application/json"))?;
                handle.http_headers(headers)?;
            }
            body => panic!("Unknown body type {:?}", body)
        }

        let result = Arc::new(Mutex::new(Vec::new()));
        let write_result = result.clone();

        handle.write_function(move |data| {
            write_result.lock().extend_from_slice(data);
            Ok(data.len())
        })?;

        Ok((handle, result))
    }
}

impl Connector for CurlConnector {
    fn request(&self, token: &str, req: HttpRequest) -> TelegramFuture<HttpResponse> {
        let request = result(self.create_request(token, req));

        let session = self.inner.clone();
        let request = request.and_then(move |(handle, result)| {
            session.perform(handle).map_err(From::from).join(Ok(result))
        });

        let future = request.and_then(move |(_, result)| {
            let mut swap = Vec::new();
            let mut guard = result.lock();
            let prev: &mut Vec<u8> = &mut guard;
            ::std::mem::swap(prev, &mut swap);
            Ok(HttpResponse {
                body: Some(swap)
            })
        });

        TelegramFuture::new(Box::new(future))
    }
}

/// Returns default curl connector.
pub fn default_connector(handle: &Handle) -> Result<Box<Connector>, Error> {
    let connector = CurlConnector::new(handle);
    Ok(Box::new(connector))
}
