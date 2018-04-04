use std::net::SocketAddr;

use futures::{Future, Poll, Stream};
use futures::future::ok;
use futures::sync::mpsc::{channel, Receiver, Sender};
use tokio_core::reactor::Handle;

use telegram_bot_raw::Update;
use telegram_bot_raw::{DeleteWebhook, SetWebhook};

use serde_json;

use api::Api;

use hyper::{Error, Method, StatusCode};
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
                req.body()
                    .concat2()
                    .wait()
                    .map_err(|_| ())
                    .and_then(|x| serde_json::from_slice(&x).map_err(|_| ()))
                    .map_err(|_| ())
                    .map(move |u: Update| sink.send(u))
                    .ok();
                response.set_status(StatusCode::Ok);
            }
            (_, _) => {
                response.set_status(StatusCode::NotFound);
            }
        }

        Box::new(ok(response))
    }
}

impl Stream for Webhook {
    type Item = Update;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.source.poll()
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

    // Register webhook on Telegram
    pub fn register<T>(&self, url: T)
    where
        T: AsRef<str>,
    {
        self.api.spawn(SetWebhook::new(url.as_ref()));
    }

    // Listen on a port and start serving the webhook service
    pub fn serve_at(&self, addr: SocketAddr) {
        let handle = self.handle.clone();
        let service = WebhookService {
            path: self.path.clone(),
            sink: self.sink.clone(),
        };

        let serve = Http::new()
            .serve_addr_handle(&addr, &handle, move || Ok(service.clone()))
            .unwrap()
            .for_each(|_| ok(()))
            .map_err(|_| ());
        handle.spawn(serve);
    }
}

impl Drop for Webhook {
    fn drop(&mut self) {
        self.api.spawn(DeleteWebhook::new());
    }
}
