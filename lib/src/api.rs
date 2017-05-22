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
use stream::{NewUpdatesStream, UpdatesStream};

/// Main type for sending requests to the Telegram bot API.
#[derive(Clone)]
pub struct Api {
    inner: Rc<ApiInner>,
}

struct ApiInner {
    token: String,
    connector: Box<Connector>,
    handle: Handle,
}

#[derive(Debug)]
pub enum ConnectorConfig {
    Default,
    Specified(Box<Connector>)
}

impl Default for ConnectorConfig {
    fn default() -> Self {
        ConnectorConfig::Default
    }
}

impl ConnectorConfig {
    pub fn new(connector: Box<Connector>) -> Self {
        ConnectorConfig::Specified(connector)
    }

    pub fn take(self, handle: &Handle) -> Box<Connector> {
        match self {
            ConnectorConfig::Default => default_connector(&handle),
            ConnectorConfig::Specified(connector) => connector
        }
    }
}

/// Configuration for an `Api`.
#[derive(Debug)]
pub struct Config {
    token: String,
    connector: ConnectorConfig,
}

impl Config {
    /// Set connector type for an `Api`.
    pub fn connector(self, connector: Box<Connector>) -> Config {
        Config {
            token: self.token,
            connector: ConnectorConfig::new(connector),
        }
    }

    /// Create new `Api` instance.
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
    /// Start construction of the `Api` instance.
    ///
    /// # Examples
    ///
    /// Using default connector.
    ///
    /// ```rust
    /// # extern crate telegram_bot;
    /// # extern crate tokio_core;
    /// use telegram_bot::Api;
    /// use tokio_core::reactor::Core;
    ///
    /// # fn main() {
    /// let core = Core::new().unwrap();
    /// # let telegram_token = "token";
    /// let api = Api::configure(telegram_token).build(core.handle());
    /// # }
    /// ```
    ///
    /// Using custom connector.
    ///
    ///
    /// ```rust
    /// # extern crate telegram_bot;
    /// # extern crate tokio_core;
    /// # #[cfg(feature = "hyper_connector")]
    /// # fn main() {
    /// use telegram_bot::Api;
    /// use telegram_bot::connector::hyper;
    /// use tokio_core::reactor::Core;
    ///
    /// let core = Core::new().unwrap();
    /// # let telegram_token = "token";
    /// let api = Api::configure(telegram_token)
    ///     .connector(hyper::default_connector(&core.handle()))
    ///     .build(core.handle());
    /// # }
    ///
    /// # #[cfg(not(feature = "hyper_connector"))]
    /// # fn main() {}
    /// ```
    pub fn configure<T: AsRef<str>>(token: T) -> Config {
        Config {
            token: token.as_ref().to_string(),
            connector: Default::default(),
        }
    }

    /// Create a stream which produces updates from the Telegram server.
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
    /// # let api: Api = Api::configure("token").build(core.handle());
    /// use futures::Stream;
    ///
    /// let future = api.stream().for_each(|update| {
    ///     println!("{:?}", update);
    ///     Ok(())
    /// });
    /// # }
    /// ```
    pub fn stream(&self) -> UpdatesStream {
        UpdatesStream::new(self.clone(), self.inner.handle.clone())
    }

    /// Send a request to the Telegram server and do not wait for a response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate futures;
    /// # extern crate telegram_bot;
    /// # extern crate tokio_core;
    /// # use futures::Future;
    /// # use telegram_bot::{Api, GetMe, ChatId};
    /// # use telegram_bot::prelude::*;
    /// # use tokio_core::reactor::Core;
    /// #
    /// # fn main() {
    /// # let core = Core::new().unwrap();
    /// # let telegram_token = "token";
    /// # let api = Api::configure(telegram_token).build(core.handle());
    /// # if false {
    /// let chat = ChatId::new(61031);
    /// api.spawn(chat.text("Message"))
    /// # }
    /// # }
    pub fn spawn<Req: Request>(&self, request: Req) {
        self.inner.handle.spawn(self.send(request).then(|_| Ok(())))
    }

    /// Send a request to the Telegram server and wait for a response, timing out after `duration`.
    /// Future will resolve to `None` if timeout fired.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate futures;
    /// # extern crate telegram_bot;
    /// # extern crate tokio_core;
    /// # use futures::Future;
    /// # use telegram_bot::{Api, GetMe};
    /// # use tokio_core::reactor::Core;
    /// #
    /// # fn main() {
    /// # let core = Core::new().unwrap();
    /// # let telegram_token = "token";
    /// # let api = Api::configure(telegram_token).build(core.handle());
    /// # if false {
    /// use std::time::Duration;
    ///
    /// let future = api.send_timeout(GetMe, Duration::from_secs(5));
    /// future.and_then(|me| Ok(assert!(me.is_some())));
    /// # }
    /// # }
    /// ```
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

    /// Send a request to the Telegram server and wait for a response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate futures;
    /// # extern crate telegram_bot;
    /// # extern crate tokio_core;
    /// # use futures::Future;
    /// # use telegram_bot::{Api, GetMe};
    /// # use tokio_core::reactor::Core;
    /// #
    /// # fn main() {
    /// # let core = Core::new().unwrap();
    /// # let telegram_token = "token";
    /// # let api = Api::configure(telegram_token).build(core.handle());
    /// # if false {
    /// let future = api.send(GetMe);
    /// future.and_then(|me| Ok(println!("{:?}", me)));
    /// # }
    /// # }
    /// ```
    pub fn send<Req: Request>(&self, request: Req)
        -> TelegramFuture<<Req::Response as Response>::Type> {

        let encoded = result(serde_json::to_vec(&request).map_err(From::from));
        let url = request.get_url(&self.inner.token);

        let api = self.clone();
        let response = encoded.and_then(move |data| {
            api.inner.connector.post_json(&url, data)
        });

        let future = response.and_then(move |bytes| {
            result(serde_json::from_slice(&bytes).map_err(From::from).and_then(|value| {
                match value {
                    ResponseWrapper::Success {result} => {
                        Ok(Req::Response::map(result))
                    },
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

