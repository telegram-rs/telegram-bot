use std::net::SocketAddr;

use futures::{Future, Poll, Stream};
use futures::future::ok;
use futures::sync::mpsc::{channel, Receiver, Sender};
use tokio_core::reactor::Handle;

use telegram_bot_raw::Update;
use telegram_bot_raw::{DeleteWebhook, SetWebhook};

use serde_json;

use api::Api;

use hyper::error::Error;
use hyper::{Method, StatusCode};
use hyper::server::{Http, Request, Response, Service};

const TELEGRAM_WEBHOOK_DEFAULT_PATH: &'static str = "/";

/// This type represents stream of Telegram API updates and uses
/// webhook under the hood.
#[must_use = "streams do nothing unless polled"]
pub struct Webhook {
    api: Api,
    handle: Handle,
    path: String,
    sink: Sender<Update>,
    source: Receiver<Update>,
}

#[derive(Clone)]
struct WebhookService {
    path: String,
    sink: Sender<Update>,
}

impl Service for WebhookService {
    type Request = Request;
    type Response = Response;
    type Error = Error;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        use futures::Sink;
        let mut response = Response::new();
        let (method, path): (_, String) = (req.method().clone(), req.path().into());
        let sink = self.sink.clone();

        match (method, path) {
            (Method::Post, ref p) if *p == self.path => {
                let send_body_fut = req.body()
                    .concat2()
                    .map_err(|_| ())
                    .and_then(|x| serde_json::from_slice(&x).map_err(|_| ()))
                    .map_err(|_| ())
                    .and_then(|u: Update| sink.send(u).map_err(|_| ()));
                response.set_status(StatusCode::Ok);
                return Box::new(send_body_fut.and_then(|_| ok(response)).map_err(|_| Error::Method));
            }
            (_, _) => {
                response.set_status(StatusCode::NotFound);
                return Box::new(ok(response))
            }
        };
    }
}

impl Stream for Webhook {
    type Item = Update;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        let res = self.source.poll();
        res
    }
}

impl Webhook {
    pub fn new(api: Api, handle: Handle) -> Self {
        let (sink, source) = channel(100);
        Self {
            path: TELEGRAM_WEBHOOK_DEFAULT_PATH.into(),
            sink,
            source,
            api,
            handle,
        }
    }

    pub fn path<T>(&mut self, p: T) -> &mut Self
    where T: AsRef<str> {
        self.path = p.as_ref().to_string();
        self
    }

    // Register webhook on Telegram
    pub fn register<T>(&self, url: T)
    where
        T: AsRef<str>,
    {
        self.api.spawn(SetWebhook::new(url.as_ref()));
    }

    pub fn unregister(&self)
    {
        self.api.spawn(DeleteWebhook::new());
    }

    // Listen on a port and start serving the webhook service
    pub fn serve_at(&self, addr: SocketAddr) {
        let handle = self.handle.clone();
        let service = WebhookService {
            path: self.path.clone(),
            sink: self.sink.clone(),
        };

        let server_handle = handle.clone();
        let serve = Http::new()
            .serve_addr_handle(&addr, &handle, move || Ok(service.clone()))
            .unwrap()
            .for_each(move |conn| {
                server_handle.spawn(conn.map(|_| ()).map_err(|_| ()));
                Ok(())
            })
            .map_err(|_| ());
        handle.spawn(serve);
    }
}

impl Drop for Webhook {
    fn drop(&mut self) {
        self.unregister();
    }
}
