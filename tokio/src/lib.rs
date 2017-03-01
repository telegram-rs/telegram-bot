#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio_core;
extern crate url;
extern crate telegram_bot_raw;

use std::sync::Arc;
use std::cmp::max;
use std::collections::VecDeque;

use futures::{Future, Stream, Poll, Async};
use futures::future::{result};
use hyper::{Body, Method};
use hyper::client::Client;
use hyper::header::ContentType;
use hyper_tls::HttpsConnector;
use tokio_core::reactor::Handle;
use url::Url;

use telegram_bot_raw::{GetUpdates, Request, Response, ResponseParameters, Update, Integer};

const TELEGRAM_URL: &'static str = "https://api.telegram.org/";
const TELEGRAM_LONG_POLL_TIMEOUT: usize = 5;

error_chain! {
    foreign_links {
        Url(url::ParseError);
        Hyper(hyper::Error);
        Json(serde_json::Error);
    }

    errors {
        TelegramError {
            description: String,
            parameters: Option<ResponseParameters>
        }
    }
}

#[must_use = "futures do nothing unless polled"]
pub struct TelegramFuture<T> {
    inner: Box<Future<Item=T, Error=Error>>
}

impl<T> Future for TelegramFuture<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}

#[must_use = "streams do nothing unless polled"]
pub struct UpdatesStream {
    bot: Bot,
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
                let request = self.bot.send(GetUpdates {
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
    fn new(bot: &Bot) -> Self {
        UpdatesStream {
            bot: bot.clone(),
            last_update: 0,
            buffer: VecDeque::new(),
            current_request: None,
            timeout: TELEGRAM_LONG_POLL_TIMEOUT,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Bot {
    inner: Arc<BotInner>,
}

#[derive(Debug, Clone)]
struct BotInner {
    token: String,
    client: Client<HttpsConnector>,
}

impl Bot {
    pub fn from_token(handle: &Handle, token: &str) -> Result<Self> {
        let connector = HttpsConnector::new(1, handle);
        let config = Client::configure().connector(connector);

        Ok(Bot {
            inner: Arc::new(BotInner {
                token: token.to_string(),
                client: config.build(handle),
            }),
        })
    }

    pub fn stream(&self) -> UpdatesStream {
        UpdatesStream::new(self)
    }

    pub fn send<Req>(&self, request: Req) -> TelegramFuture<Req::Response>
        where Req: Request + 'static, <Req as Request>::Response: std::marker::Send + 'static {

        let url = result(url(&self.inner.token, request.name()));
        let body = futures::lazy(move || {
            serde_json::to_vec(&request).map(Body::from)
        }).map_err(From::from);

        let bot = self.clone();
        let response = url.join(body).and_then(move |(url, body)| {
            let mut http_request = hyper::client::Request::new(Method::Post, url);
            http_request.set_body(body);
            http_request.headers_mut().set(ContentType::json());

            bot.inner.client.request(http_request).map_err(From::from)
        });

        let bytes = response.and_then(|response| {
            response.body().map_err(From::from)
                .fold(vec![], |mut result, chunk| -> Result<Vec<u8>> {
                    result.extend_from_slice(&chunk);
                    Ok(result)
            })
        });

        let future = bytes.and_then(|bytes| {
            result(serde_json::from_slice(&bytes).map_err(From::from).and_then(|value| {
                match value {
                    Response::Success {result} => Ok(result),
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

fn url(token: &str, method: &str) -> Result<Url> {
    Url::parse(&format!("{}bot{}/{}", TELEGRAM_URL, token, method)).map_err(From::from)
}
