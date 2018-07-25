use std::cmp::max;
use std::collections::VecDeque;
use std::time::Duration;

use futures::{Future, Stream, Poll, Async};
use futures::future;
use tokio_core::reactor::{Handle, Timeout};

use telegram_bot_raw::{GetUpdates, Update, Integer};

use api::Api;
use errors::Error;
use future::{TelegramFuture, NewTelegramFuture};

const TELEGRAM_LONG_POLL_TIMEOUT_SECONDS: u64 = 5;
const TELEGRAM_LONG_POLL_LIMIT_MESSAGES: Integer = 100;
const TELEGRAM_LONG_POLL_ERROR_DELAY_MILLISECONDS: u64 = 500;

/// This type represents stream of Telegram API updates and uses
/// long polling method under the hood.
#[must_use = "streams do nothing unless polled"]
pub struct UpdatesStream {
    api: Api,
    handle: Handle,
    last_update: Integer,
    buffer: VecDeque<Update>,
    current_request: Option<TelegramFuture<Option<Vec<Update>>>>,
    timeout: Duration,
    limit: Integer,
    error_delay: Duration
}

impl Stream for UpdatesStream {
    type Item = Update;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(value) = self.buffer.pop_front() {
            return Ok(Async::Ready(Some(value)))
        }

        let result = match self.current_request {
            None => Ok(false),
            Some(ref mut current_request) => {
                let polled_update = current_request.poll();
                match polled_update {
                    Ok(Async::NotReady) => return Ok(Async::NotReady),
                    Ok(Async::Ready(None)) => Ok(false),
                    Ok(Async::Ready(Some(updates))) => {
                        for update in updates {
                            self.last_update = max(update.id, self.last_update);
                            self.buffer.push_back(update)
                        }
                        Ok(true)
                    },
                    Err(err) => Err(err)
                }
            }
        };

        match result {
            Err(err) => {
                let timeout_future = future::result(Timeout::new(self.error_delay, &self.handle));

                let timeout_future = timeout_future.map_err(From::from).and_then(|timeout| {
                    timeout.map_err(From::from).map(|()| None)
                });

                self.current_request = Some(TelegramFuture::new(Box::new(timeout_future)));
                return Err(err)
            }
            Ok(false) => {
                let timeout = self.timeout + Duration::from_secs(1);

                let request = self.api.send_timeout(GetUpdates::new()
                    .offset(self.last_update + 1)
                    .timeout(self.timeout.as_secs() as Integer)
                    .limit(self.limit)
                , timeout);

                self.current_request = Some(request);
                self.poll()
            },
            Ok(true) => {
                self.current_request = None;
                self.poll()
            }
        }
    }
}

pub trait NewUpdatesStream {
    fn new(api: Api, handle: Handle) -> Self;
}

impl NewUpdatesStream for UpdatesStream{
    fn new(api: Api, handle: Handle) -> Self {
        UpdatesStream {
            api: api,
            handle: handle,
            last_update: 0,
            buffer: VecDeque::new(),
            current_request: None,
            timeout: Duration::from_secs(TELEGRAM_LONG_POLL_TIMEOUT_SECONDS),
            limit: TELEGRAM_LONG_POLL_LIMIT_MESSAGES,
            error_delay: Duration::from_millis(TELEGRAM_LONG_POLL_ERROR_DELAY_MILLISECONDS)
        }
    }
}

impl UpdatesStream {
    /// Set timeout for long polling requests, this corresponds with `timeout` field
    /// in [getUpdates](https://core.telegram.org/bots/api#getupdates) method,
    /// also this stream sets an additional request timeout for `timeout + 1 second`
    /// in case of invalid Telegram API server behaviour.
    ///
    /// Default timeout is 5 seconds.
    pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = timeout;
        self
    }

    /// Set limits the number of updates to be retrieved, this corresponds with `limit` field
    /// in [getUpdates](https://core.telegram.org/bots/api#getupdates) method.
    /// Values between 1—100 are accepted.
    ///
    /// Defaults to 100.
    pub fn limit(&mut self, limit: Integer) -> &mut Self {
        self.limit = limit;
        self
    }

    /// Set a delay between erroneous request and next request.
    /// This delay prevents busy looping in some cases.
    ///
    /// Default delay is 500 ms.
    pub fn error_delay(&mut self, delay: Duration) -> &mut Self {
        self.error_delay = delay;
        self
    }
}
