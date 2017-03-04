use std::cmp::max;
use std::collections::VecDeque;

use futures::{Future, Stream, Poll, Async};

use telegram_bot_raw::{GetUpdates, Update, Integer};

use api::{Api, TelegramFuture};
use errors::{Error};

const TELEGRAM_LONG_POLL_TIMEOUT: usize = 5;

#[must_use = "streams do nothing unless polled"]
pub struct UpdatesStream {
    api: Api,
    last_update: Integer,
    buffer: VecDeque<Update>,
    current_request: Option<TelegramFuture<Vec<Update>>>,
    timeout: usize,
}

impl Stream for UpdatesStream {
    type Item = Update;
    type Error = Error;

    fn poll(&mut self) -> Poll<Option<Self::Item>, Self::Error> {
        if let Some(value) = self.buffer.pop_front() {
            return Ok(Async::Ready(Some(value)))
        }

        let result = match self.current_request {
            None => None,
            Some(ref mut current_request) => {
                let polled_update = current_request.poll();
                match polled_update {
                    Ok(Async::NotReady) => return Ok(Async::NotReady),
                    Ok(Async::Ready(updates)) => {
                        if updates.is_empty() {
                            None
                        } else {
                            for update in updates.iter() {
                                self.last_update = max(update.id, self.last_update);
                            }
                            Some(updates)
                        }
                    },
                    Err(err) => return Err(err)
                }
            }
        };

        match result {
            None => {
                let request = self.api.send(GetUpdates {
                    offset: Some(self.last_update + 1),
                    limit: None,
                    timeout: Some(self.timeout as Integer),
                    allowed_updates: Vec::new(),
                });

                self.current_request = Some(request);
                self.poll()
            },
            Some(mut updates) => {
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

    pub fn timeout(&mut self, timeout: usize) -> &mut Self {
        self.timeout = timeout;
        self
    }
}
