use std::sync::{Arc, Mutex};
use std::rc::Rc;

use curl::easy::{Easy, List};
use futures::Future;
use futures::future::result;
use tokio_core::reactor::Handle;
use tokio_curl::Session;

use errors::Error;
use future::{TelegramFuture, NewTelegramFuture};

use super::_base::Connector;

pub struct CurlConnector {
    inner: Rc<Session>
}

impl CurlConnector {
    pub fn new(handle: &Handle) -> Self {
        CurlConnector {
            inner: Rc::new(Session::new(handle.clone()))
        }
    }

    fn create_request(&self, uri: &str, data: Vec<u8>) -> Result<(Easy, Arc<Mutex<Vec<u8>>>), Error> {
        let mut header = List::new();
        header.append("Content-Type: application/json")?;

        let mut handle = Easy::new();
        handle.url(uri)?;
        handle.post(true)?;
        handle.post_fields_copy(&data)?;
        handle.http_headers(header)?;

        let result = Arc::new(Mutex::new(Vec::new()));
        let write_result = result.clone();

        handle.write_function(move |data| {
            write_result.lock().unwrap().extend_from_slice(data);
            Ok(data.len())
        })?;

        Ok((handle, result))
    }
}

impl Connector for CurlConnector {
    fn post_json(&self, uri: &str, data: Vec<u8>) -> TelegramFuture<Vec<u8>> {
        let request = result(self.create_request(uri, data));

        let session = self.inner.clone();
        let request = request.and_then(move |(handle, result)| {
            session.perform(handle).map_err(From::from).join(Ok(result))
        });

        let future = request.and_then(move |(_, result)| {
            let mut swap = Vec::new();
            let mut guard = result.lock().unwrap();
            let prev: &mut Vec<u8> = &mut guard;
            ::std::mem::swap(prev, &mut swap);
            Ok(swap)
        });

        TelegramFuture::new(Box::new(future))
    }
}

pub fn default_connector(handle: &Handle) -> Box<Connector> {
    let connector = CurlConnector::new(handle);
    Box::new(connector)
}
