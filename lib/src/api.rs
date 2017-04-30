use std::rc::Rc;
use std::time::Duration;

use futures::{Future};
use futures::future::{result};
use serde_json;
use tokio_core::reactor::{Handle, Timeout};

use telegram_bot_raw::{Request, Response};

use connector::{Connector, default_connector};
use errors::{Result, ErrorKind};
use future::TelegramFuture;
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

impl Api {
    pub fn from_token(handle: &Handle, token: &str) -> Result<Self> {
        Ok(Api {
            inner: Rc::new(ApiInner {
                token: token.to_string(),
                connector: default_connector(handle),
                handle: handle.clone(),
            }),
        })
    }

    pub fn stream(&self) -> UpdatesStream {
        UpdatesStream::new(self)
    }

    pub fn spawn<Req>(&self, request: Req)
        where Req: Request, <Req as Request>::Response: ::std::marker::Send + 'static {

        self.inner.handle.spawn(self.send(request).then(|_| Ok(())))
    }

    pub fn send_timeout<Req>(
        &self, request: Req, duration: Duration) -> TelegramFuture<Option<Req::Response>>
        where Req: Request, <Req as Request>::Response: ::std::marker::Send + 'static {

        let timeout_future = result(Timeout::new(duration, &self.inner.handle))
            .flatten().map_err(From::from).map(|()| None);
        let send_future = self.send(request).map(|resp| Some(resp));

        let future = timeout_future.select(send_future)
            .map(|(item, _next)| item)
            .map_err(|(item, _next)| item);

        TelegramFuture {
            inner: Box::new(future)
        }
    }

    pub fn send<Req>(&self, request: Req) -> TelegramFuture<Req::Response>
        where Req: Request, <Req as Request>::Response: ::std::marker::Send + 'static {

        let name = Req::name();
        let encoded = result(serde_json::to_vec(&request).map_err(From::from));
        let url = url(&self.inner.token, name);

        let api = self.clone();
        let response = encoded.and_then(move |data| {
            api.inner.connector.post(&url, data)
        });

        let future = response.and_then(|bytes| {
            result(serde_json::from_slice(&bytes).map_err(From::from).and_then(|value| {
                match value {
                    Response::Success {result} => Ok(Req::map(result)),
                    Response::Error { description, parameters } => {
                        Err(ErrorKind::TelegramError {
                            description: description,
                            parameters: parameters
                        }.into())
                    },
                }
            }))
        });

        TelegramFuture {
            inner: Box::new(future)
        }
    }
}

fn url(token: &str, method: &str) -> String {
    format!("{}bot{}/{}", TELEGRAM_URL, token, method)
}
