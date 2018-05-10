use std::net::SocketAddr;

use futures::future::ok;
use futures::sync::mpsc::{channel, Receiver, Sender};
use futures::{Future, Poll, Stream};
use tokio_core::reactor::Handle;

use telegram_bot_raw::Update;
use telegram_bot_raw::{DeleteWebhook, SetWebhook};

use api::Api;

use hyper::error::Error as HyperError;
use hyper::server::{Http, Request, Response, Service};
use hyper::{Method, StatusCode};

use errors::{Error, ErrorKind};

const TELEGRAM_WEBHOOK_DEFAULT_PATH: &'static str = "/";

/// This type represents stream of Telegram API updates and uses
/// webhook under the hood.
#[must_use = "streams do nothing unless polled"]
pub struct WebhookConfig {
    api: Api,
    handle: Handle,
    registered: bool,
    path: String,
}

pub struct WebhookStream {
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
    type Error = HyperError;
    type Future = Box<Future<Item = Self::Response, Error = Self::Error>>;

    fn call(&self, req: Request) -> Self::Future {
        use futures::Sink;
        use std::str;
        let mut response = Response::new();
        let (method, path): (_, String) = (req.method().clone(), req.path().into());
        let sink = self.sink.clone();

        match (method, path) {
            (Method::Post, ref p) if *p == self.path => {
                response.set_status(StatusCode::Ok);
                let response_fut = req.body()
                    .concat2()
                    .map_err(|_| ())
                    .and_then(|chunk| {
                        let s = str::from_utf8(&chunk).map_err(|_| ())?;
                        Update::from_raw_json(&s).map_err(|_| ())
                    })
                    .and_then(|update| sink.send(update).map_err(|_| ()))
                    .then(|_| ok(response));
                return Box::new(response_fut);
            }
            (_, _) => {
                response.set_status(StatusCode::NotFound);
                return Box::new(ok(response));
            }
        };
    }
}

impl Stream for WebhookStream {
    type Item = Update;
    type Error = ();

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        self.source.poll()
    }
}

/// Telegram Webhook
///
/// This module implements support for Telegram Webhook feature.
///
/// Implementation details:
///
/// When a webhook instance is set up and started serving, it will start
/// an asynchronized HTTP server that binds on specified port. This server will
/// handle incoming POST requests that matches upon given *path* (otherwise an
/// HTTP error 404 will be responded). The requested body will be parsed as JSON
/// into an `Update` object, and sent through a buffered mpsc channel. When
/// you call a Stream method (like `for_each`) on a WebhookStream, it will try to
/// obtain updates from the channel.
///
/// Of course, after setting up the service, we need to ask Telegram to begin
/// sending updates to our webhook server, that is, we need to register our
/// callback URL with the `setWebhook` bot API. Such functionality is provided
/// by `Webhook::register(url)` method. When the app quits, you need to
/// unregister webhook, or you'll no longer be able to get updates with long
/// polling next time. This is automatically done when a WebhookConfig finish its
/// lifetime thanks to the `std::ops::Drop` trait implementation.
///
/// Notice:
///
/// Since Telegram requires webhook to run on HTTPS protocol,
/// you need a working HTTPS gateway to forward requests to your webhook.
/// In development environment, I found ngrok helpful for debugging.
/// In production, a modern web server (like nginx) will do the job.
///
/// # Examples
///
/// ```rust
/// # extern crate futures;
/// # extern crate telegram_bot;
/// # extern crate tokio_core;
/// # use telegram_bot::Api;
/// # use tokio_core::reactor::Core;
/// # fn main() {
/// # let core = Core::new().unwrap();
/// # let api: Api = Api::configure("token").build(core.handle()).unwrap();
/// use futures::Stream;
///
/// let mut webhook = api.webhook();
/// webhook.path("/my/crazy/path");
/// webhook.register("https://my.website.com/telegram-webhook");
/// let stream = webhook.serve_at("127.0.0.1:9876".parse().unwrap()).unwrap();
/// let future = stream.for_each(|update| {
///    println!("{:?}", update);
///    Ok(())
/// });
/// # }
/// ```
///
/// ```
/// // # sample nginx config
/// // http {
/// //    server_name my.website.com;
/// //    listen 443 ssl;
/// //    # other tls settings
/// //
/// //    location /telegram-webhook {
/// //        proxy_pass http://127.0.0.1:9876/my/crazy/path;
/// //    }
/// // }
/// ```
///
impl WebhookConfig {
    /// Create a webhook instance
    ///
    /// Please call `Api::webhook()` instead of calling this function directly.
    pub fn new(api: Api, handle: Handle) -> Self {
        Self {
            path: TELEGRAM_WEBHOOK_DEFAULT_PATH.into(),
            api,
            handle,
            registered: false,
        }
    }

    /// Specify webhook server matching path
    pub fn path<T>(&mut self, p: T) -> &mut Self
    where
        T: AsRef<str>,
    {
        self.path = p.as_ref().to_string();
        self
    }

    // Register webhook callback URL with Telegram
    pub fn register<T>(&mut self, url: T)
    where
        T: AsRef<str>,
    {
        self.api.spawn(SetWebhook::new(url.as_ref()));
        self.registered = true;
    }

    // Unregister webhook from Telegram
    pub fn unregister(&self) {
        self.api.spawn(DeleteWebhook::new());
    }

    // Listen on a port and start serving the webhook service
    pub fn serve_at(&self, addr: SocketAddr) -> Result<WebhookStream, Error> {
        if !self.registered {
            return Err(ErrorKind::WebhookNotRegistered.into());
        }
        let handle = self.handle.clone();
        let (sink, source) = channel(100);

        let service = WebhookService {
            path: self.path.clone(),
            sink: sink.clone(),
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

        Ok(WebhookStream { source })
    }
}

impl Drop for WebhookConfig {
    fn drop(&mut self) {
        self.api.spawn(DeleteWebhook::new());
    }
}
