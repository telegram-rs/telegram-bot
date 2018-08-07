use std::rc::Rc;
use std::time::Duration;

use futures::future::result;
use futures::Future;
use tokio;
use tokio_timer;

use telegram_bot_raw::{Request, ResponseType};

use connector::{default_connector, Connector};
use errors::Error;
use future::{NewTelegramFuture, TelegramFuture};
use stream::{NewUpdatesStream, UpdatesStream};

/// Main type for sending requests to the Telegram bot API.
#[derive(Clone)]
pub struct Api {
    inner: Rc<ApiInner>,
}

struct ApiInner {
    token: String,
    connector: Box<Connector>,
}

#[derive(Debug)]
pub enum ConnectorConfig {
    Default,
    Specified(Box<Connector>),
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

    pub fn take(self) -> Result<Box<Connector>, Error> {
        match self {
            ConnectorConfig::Default => default_connector(),
            ConnectorConfig::Specified(connector) => Ok(connector),
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
    pub fn build(self) -> Result<Api, Error> {
        Ok(Api {
            inner: Rc::new(ApiInner {
                token: self.token,
                connector: self.connector.take()?,
            }),
        })
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
    /// # extern crate tokio;
    /// use telegram_bot::Api;
    ///
    /// # fn main() {
    /// # let telegram_token = "token";
    /// let api = Api::configure(telegram_token).build().unwrap();
    /// # }
    /// ```
    ///
    /// Using custom connector.
    ///
    ///
    /// ```rust
    /// # extern crate telegram_bot;
    /// # extern crate tokio;
    /// # #[cfg(feature = "hyper_connector")]
    /// # fn main() {
    /// use telegram_bot::Api;
    /// use telegram_bot::connector::hyper;
    ///
    /// # let telegram_token = "token";
    /// let api = Api::configure(telegram_token)
    ///     .connector(hyper::default_connector().unwrap())
    ///     .build().unwrap();
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
    /// # extern crate tokio;
    /// # use telegram_bot::Api;
    /// # fn main() {
    /// # let api: Api = Api::configure("token").build().unwrap();
    /// use futures::Stream;
    ///
    /// let future = api.stream().for_each(|update| {
    ///     println!("{:?}", update);
    ///     Ok(())
    /// });
    /// # }
    /// ```
    pub fn stream(&self) -> UpdatesStream {
        UpdatesStream::new(self.clone())
    }

    /// Send a request to the Telegram server and do not wait for a response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate futures;
    /// # extern crate telegram_bot;
    /// # extern crate tokio;
    /// # use futures::Future;
    /// # use telegram_bot::{Api, GetMe, ChatId};
    /// # use telegram_bot::prelude::*;
    /// #
    /// # fn main() {
    /// # let telegram_token = "token";
    /// # let api = Api::configure(telegram_token).build().unwrap();
    /// # if false {
    /// let chat = ChatId::new(61031);
    /// api.spawn(chat.text("Message"))
    /// # }
    /// # }
    pub fn spawn<Req: Request>(&self, request: Req) {
        tokio::executor::current_thread::spawn(self.send(request).then(|_| Ok(())));
    }

    /// Send a request to the Telegram server and wait for a response, timing out after `duration`.
    /// Future will resolve to `None` if timeout fired.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # extern crate futures;
    /// # extern crate telegram_bot;
    /// # extern crate tokio;
    /// # use futures::Future;
    /// # use telegram_bot::{Api, GetMe};
    /// #
    /// # fn main() {
    /// # let telegram_token = "token";
    /// # let api = Api::configure(telegram_token).build().unwrap();
    /// # if false {
    /// use std::time::Duration;
    ///
    /// let future = api.send_timeout(GetMe, Duration::from_secs(5));
    /// future.and_then(|me| Ok(assert!(me.is_some())));
    /// # }
    /// # }
    /// ```
    pub fn send_timeout<Req: Request>(
        &self,
        request: Req,
        duration: Duration,
    ) -> TelegramFuture<Option<<Req::Response as ResponseType>::Type>> {
        let timeout_future = tokio_timer::sleep(duration)
            .map_err(From::from)
            .map(|()| None);
        let send_future = self.send(request).map(|resp| Some(resp));

        let future = timeout_future
            .select(send_future)
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
    /// # extern crate tokio;
    /// # use futures::Future;
    /// # use telegram_bot::{Api, GetMe};
    /// #
    /// # fn main() {
    /// # let telegram_token = "token";
    /// # let api = Api::configure(telegram_token).build().unwrap();
    /// # if false {
    /// let future = api.send(GetMe);
    /// future.and_then(|me| Ok(println!("{:?}", me)));
    /// # }
    /// # }
    /// ```
    pub fn send<Req: Request>(
        &self,
        request: Req,
    ) -> TelegramFuture<<Req::Response as ResponseType>::Type> {
        let request = request.serialize().map_err(From::from);

        let request = result(request);

        let api = self.clone();
        let response = request.and_then(move |request| {
            let ref token = api.inner.token;
            api.inner.connector.request(token, request)
        });

        let future = response
            .and_then(move |response| Req::Response::deserialize(response).map_err(From::from));

        TelegramFuture::new(Box::new(future))
    }
}
