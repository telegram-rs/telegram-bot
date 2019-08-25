use std::time::Duration;

use tokio::timer::Timeout;

use telegram_bot_raw::{Request, ResponseType};

use crate::connector;
use crate::errors::Error;
use crate::stream::UpdatesStream;

/// Main type for sending requests to the Telegram bot API.
#[derive(Clone)]
pub struct Api {
    pub token: String,
}

pub(crate) async fn send_request<Req: Request>(
    token: String,
    request: Req,
) -> Result<<Req::Response as ResponseType>::Type, Error> {
    let request = request.serialize()?;
    let response = connector::request(token.as_ref(), request).await?;

    Req::Response::deserialize(response).map_err(From::from)
}

pub(crate) async fn send_timeout_request<Req: Request>(
    token: String,
    request: Req,
    duration: Duration,
) -> Result<<Req::Response as ResponseType>::Type, Error> {
    Timeout::new(send_request(token, request), duration).await?
}

impl Api {
    ///  create a new `Api` instance.
    ///
    /// # Example
    ///
    /// Using default connector.
    ///
    /// ```rust
    /// use telegram_bot::Api;
    ///
    /// # fn main() {
    /// # let telegram_token = "token";
    /// let api = Api::new(telegram_token);
    /// # }
    /// ```
    pub fn new<T: AsRef<str>>(token: T) -> Api {
        Api {
            token: token.as_ref().to_string(),
        }
    }

    /// Create a stream which produces updates from the Telegram server.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use telegram_bot::Api;
    /// use futures::StreamExt;
    ///
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let api: Api = Api::new("token");
    ///
    /// let mut stream = api.stream();
    /// let update = stream.next().await;
    ///     println!("{:?}", update);
    /// # }
    /// ```
    pub fn stream(&self) -> UpdatesStream {
        UpdatesStream::new(&self.token)
    }

    /// Send a request to the Telegram server and wait for a response, timing out after `duration`.
    /// Future will resolve to `None` if timeout fired.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use telegram_bot::{Api, GetMe};
    /// # use std::time::Duration;
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let telegram_token = "token";
    /// # let api = Api::new(telegram_token);
    /// # if false {
    /// let result = api.send_timeout(GetMe, Duration::from_secs(2)).await;
    /// println!("{:?}", result);
    /// # }
    /// # }
    /// ```
    pub async fn send_timeout<Req: Request>(
        &self,
        request: Req,
        duration: Duration,
    ) -> Result<<Req::Response as ResponseType>::Type, Error> {
        send_timeout_request(self.token.clone(), request, duration).await
    }

    /// Send a request to the Telegram server and wait for a response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use telegram_bot::{Api, GetMe};
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let telegram_token = "token";
    /// # let api = Api::new(telegram_token);
    /// # if false {
    /// let result = api.send(GetMe).await;
    /// println!("{:?}", result);
    /// # }
    /// # }
    /// ```
    pub async fn send<Req: Request>(
        &self,
        request: Req,
    ) -> Result<<Req::Response as ResponseType>::Type, Error> {
        send_request(self.token.clone(), request).await
    }
}
