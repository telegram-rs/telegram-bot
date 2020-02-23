use std::sync::{
    atomic::{AtomicUsize, Ordering},
    Arc,
};
use std::time::Duration;

use futures::{Future, FutureExt};
use tokio::time::timeout;
use tracing_futures::Instrument;

use telegram_bot_raw::{HttpRequest, Request, ResponseType};

use crate::connector::{default_connector, Connector};
use crate::errors::{Error, ErrorKind};
use crate::stream::UpdatesStream;

/// Main type for sending requests to the Telegram bot API.
#[derive(Clone)]
pub struct Api(Arc<ApiInner>);

struct ApiInner {
    token: String,
    connector: Box<dyn Connector>,
    next_request_id: AtomicUsize,
}

impl Api {
    /// Create a new `Api` instance.
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
    pub fn new<T: AsRef<str>>(token: T) -> Self {
        Self::with_connector(token, default_connector())
    }

    /// Create a new `Api` instance wtih custom connector.
    pub fn with_connector<T: AsRef<str>>(token: T, connector: Box<dyn Connector>) -> Self {
        Api(Arc::new(ApiInner {
            token: token.as_ref().to_string(),
            connector,
            next_request_id: AtomicUsize::new(0),
        }))
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
        UpdatesStream::new(&self)
    }

    /// Send a request to the Telegram server and do not wait for a response.
    ///
    /// # Examples
    ///
    /// ```rust
    /// # use telegram_bot::{Api, ChatId, prelude::*};
    /// # use std::time::Duration;
    /// #
    /// # #[tokio::main]
    /// # async fn main() {
    /// # let telegram_token = "token";
    /// # let api = Api::new(telegram_token);
    /// # if false {
    /// let chat = ChatId::new(61031);
    /// api.spawn(chat.text("Message"));
    /// # }
    /// # }
    /// ```
    pub fn spawn<Req: Request>(&self, request: Req) {
        let api = self.clone();
        if let Ok(request) = request.serialize() {
            tokio::spawn(async move {
                let _ = api.send_http_request::<Req::Response>(request).await;
            });
        }
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
    pub fn send_timeout<Req: Request>(
        &self,
        request: Req,
        duration: Duration,
    ) -> impl Future<Output = Result<Option<<Req::Response as ResponseType>::Type>, Error>> + Send
    {
        let api = self.clone();
        let request = request.serialize();
        async move {
            match timeout(
                duration,
                api.send_http_request::<Req::Response>(request.map_err(ErrorKind::from)?),
            )
            .await
            {
                Err(_) => Ok(None),
                Ok(Ok(result)) => Ok(Some(result)),
                Ok(Err(error)) => Err(error),
            }
        }
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
    pub fn send<Req: Request>(
        &self,
        request: Req,
    ) -> impl Future<Output = Result<<Req::Response as ResponseType>::Type, Error>> + Send {
        let api = self.clone();
        let request = request.serialize();
        async move {
            api.send_http_request::<Req::Response>(request.map_err(ErrorKind::from)?)
                .await
        }
    }

    async fn send_http_request<Resp: ResponseType>(
        &self,
        request: HttpRequest,
    ) -> Result<Resp::Type, Error> {
        let request_id = self.0.next_request_id.fetch_add(1, Ordering::Relaxed);
        let span = tracing::trace_span!("send_http_request", request_id = request_id);
        async {
            tracing::trace!(name = %request.name(), body = %request.body, "sending request");
            let http_response = self.0.connector.request(&self.0.token, request).await?;
            tracing::trace!(
                response = %match http_response.body {
                    Some(ref vec) => match std::str::from_utf8(vec) {
                        Ok(str) => str,
                        Err(_) => "<invalid utf-8 string>"
                    },
                    None => "<empty body>",
                }, "response received"
            );

            let response = Resp::deserialize(http_response).map_err(ErrorKind::from)?;
            tracing::trace!("response deserialized");
            Ok(response)
        }
        .map(|result| {
            if let Err(ref error) = result {
                tracing::error!(error = %error);
            }
            result
        })
        .instrument(span)
        .await
    }
}
