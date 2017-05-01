use std::cmp::max;
use std::collections::VecDeque;
use std::time::Duration;

use futures::{Future, Stream, Poll, Async};

use telegram_bot_raw::{GetUpdates, Update, Integer};

use api::Api;
use errors::Error;
use future::TelegramFuture;

const TELEGRAM_LONG_POLL_TIMEOUT: u64 = 5;

#[must_use = "streams do nothing unless polled"]
pub struct UpdatesStream {
    api: Api,
    last_update: Integer,
    buffer: VecDeque<Update>,
    current_request: Option<TelegramFuture<Option<Vec<Update>>>>,
    timeout: u64,
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
                self.current_request = None;
                return Err(err)
            }
            Ok(None) => {
                let timeout = Duration::from_secs(self.timeout + 1);

                let request = self.api.send_timeout(GetUpdates::new()
                    .offset(self.last_update + 1)
                    .timeout(self.timeout as Integer)
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
    pub fn new(api: &Api) -> Self {
        UpdatesStream {
            api: api.clone(),
            last_update: 0,
            buffer: VecDeque::new(),
            current_request: None,
            timeout: TELEGRAM_LONG_POLL_TIMEOUT,
        }
    }

    pub fn timeout(&mut self, timeout: u64) -> &mut Self {
        self.timeout = timeout;
        self
    }
}
