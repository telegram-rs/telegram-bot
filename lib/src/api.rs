use std::borrow::Borrow;
use std::rc::Rc;
use std::time::Duration;

use futures::{Future};
use futures::future::{result};
use serde_json;
use tokio_core::reactor::{Handle, Timeout};

use telegram_bot_raw::{Request, ResponseWrapper, Response};

use connector::{Connector, default_connector};
use errors::ErrorKind;
use future::{TelegramFuture, NewTelegramFuture};
use stream::UpdatesStream;

const TELEGRAM_URL: &'static str = "https://api.telegram.org/";

#[derive(Clone)]
pub struct Api {
    inner: Rc<ApiInner>,
}

struct ApiInner {
    token: String,
    connector: Box<Connector>,
    handle: Handle,
}

pub trait HasHandle {
    fn handle(&self) -> &Handle;
}

impl HasHandle for Api {
    fn handle(&self) -> &Handle {
        &self.inner.handle
    }
}

pub trait ConnectorConfig {
    fn take(self, &Handle) -> Box<Connector>;
}

#[derive(Debug, Copy, Clone)]
pub struct DefaultConnector;

impl ConnectorConfig for DefaultConnector {
    fn take(self, handle: &Handle) -> Box<Connector> {
        default_connector(handle)
    }
}

#[derive(Debug)]
pub struct SpecifiedConnector {
    connector: Box<Connector>,
}

impl ConnectorConfig for SpecifiedConnector {
    fn take(self, _handle: &Handle) -> Box<Connector> {
        self.connector
    }
}

#[derive(Debug)]
pub struct Config<Connector> {
    token: String,
    connector: Connector,
}

impl<C: ConnectorConfig> Config<C> {
    pub fn connector(self, connector: Box<Connector>) -> Config<SpecifiedConnector> {
        Config {
            token: self.token,
            connector: SpecifiedConnector {
                connector: connector,
            }
        }
    }

    pub fn build<H: Borrow<Handle>>(self, handle: H) -> Api {
        let handle = handle.borrow().clone();
        Api {
            inner: Rc::new(ApiInner {
                token: self.token,
                connector: self.connector.take(&handle),
                handle: handle,
            }),
        }
    }
}

impl Api {
    pub fn configure<T: AsRef<str>>(token: T) -> Config<DefaultConnector> {
        Config {
            token: token.as_ref().to_string(),
            connector: DefaultConnector,
        }
    }

    pub fn stream(&self) -> UpdatesStream {
        UpdatesStream::new(self)
    }

    pub fn spawn<Req: Request>(&self, request: Req) {
        self.inner.handle.spawn(self.send(request).then(|_| Ok(())))
    }

    pub fn send_timeout<Req: Request>(
        &self, request: Req, duration: Duration)
        -> TelegramFuture<Option<<Req::Response as Response>::Type>> {

        let timeout_future = result(Timeout::new(duration, &self.inner.handle))
            .flatten().map_err(From::from).map(|()| None);
        let send_future = self.send(request).map(|resp| Some(resp));

        let future = timeout_future.select(send_future)
            .map(|(item, _next)| item)
            .map_err(|(item, _next)| item);

        TelegramFuture::new(Box::new(future))
    }

    pub fn send<Req: Request>(&self, request: Req)
        -> TelegramFuture<<Req::Response as Response>::Type> {

        let name = request.name();
        let encoded = result(serde_json::to_vec(&request).map_err(From::from));
        let url = url(&self.inner.token, name);

        let api = self.clone();
        let response = encoded.and_then(move |data| {
            api.inner.connector.post_json(&url, data)
        });

        let future = response.and_then(|bytes| {
            result(serde_json::from_slice(&bytes).map_err(From::from).and_then(|value| {
                match value {
                    ResponseWrapper::Success {result} => Ok(Req::Response::map(result)),
                    ResponseWrapper::Error { description, parameters } => {
                        Err(ErrorKind::TelegramError {
                            description: description,
                            parameters: parameters
                        }.into())
                    },
                }
            }))
        });

        TelegramFuture::new(Box::new(future))
    }
}

fn url(token: &str, method: &str) -> String {
    format!("{}bot{}/{}", TELEGRAM_URL, token, method)
}
