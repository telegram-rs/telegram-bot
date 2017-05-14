use std::cmp::max;
use std::collections::VecDeque;
use std::time::Duration;

use futures::{Future, Stream, Poll, Async};
use tokio_core::reactor::{Handle, Timeout};

use telegram_bot_raw::{GetUpdates, Update, Integer};

use api::{Api, HasHandle};
use errors::Error;
use future::{TelegramFuture, NewTelegramFuture};

const TELEGRAM_LONG_POLL_TIMEOUT_SECONDS: u64 = 5;
const TELEGRAM_LONG_POLL_ERROR_DELAY_MILLISECONDS: u64 = 500;

#[must_use = "streams do nothing unless polled"]
pub struct UpdatesStream {
    api: Api,
    handle: Handle,
    last_update: Integer,
    buffer: VecDeque<Update>,
    current_request: Option<TelegramFuture<Option<Vec<Update>>>>,
    timeout: Duration,
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
            None => Ok(None),
            Some(ref mut current_request) => {
                let polled_update = current_request.poll();
                match polled_update {
                    Ok(Async::NotReady) => return Ok(Async::NotReady),
                    Ok(Async::Ready(None)) => Ok(None),
                    Ok(Async::Ready(Some(updates))) => {
                        if updates.is_empty() {
                            Ok(None)
                        } else {
                            for update in updates.iter() {
                                self.last_update = max(update.id, self.last_update);
                            }
                            Ok(Some(updates))
                        }
                    },
                    Err(err) => Err(err)
                }
            }
        };

        match result {
            Err(err) => {
                let timeout_future = Timeout::new(self.error_delay, &self.handle).unwrap()
                    .map_err(From::from).map(|()| None);

                self.current_request = Some(TelegramFuture::new(Box::new(timeout_future)));
                return Err(err)
            }
            Ok(None) => {
                let timeout = self.timeout + Duration::from_secs(1);

                let request = self.api.send_timeout(GetUpdates::new()
                    .offset(self.last_update + 1)
                    .timeout(self.timeout.as_secs() as Integer)
                , timeout);

                self.current_request = Some(request);
                self.poll()
            },
            Ok(Some(mut updates)) => {
                self.current_request = None;
                // Updates are guarantied to be not empty
                self.buffer.extend(updates.drain(1..));
                let result = updates.pop().unwrap();
                return Ok(Async::Ready(Some(result)))
            }
        }
    }
}

impl UpdatesStream {
    pub fn new(api: Api) -> Self {
        UpdatesStream {
            api: api.clone(),
            handle: api.handle().clone(),
            last_update: 0,
            buffer: VecDeque::new(),
            current_request: None,
            timeout: Duration::from_secs(TELEGRAM_LONG_POLL_TIMEOUT_SECONDS),
            error_delay: Duration::from_millis(TELEGRAM_LONG_POLL_ERROR_DELAY_MILLISECONDS)
        }
    }

    pub fn timeout(&mut self, timeout: Duration) -> &mut Self {
        self.timeout = timeout;
        self
    }

    pub fn error_delay(&mut self, delay: Duration) -> &mut Self {
        self.error_delay = delay;
        self
    }
}
