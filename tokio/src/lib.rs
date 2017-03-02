#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio_core;
extern crate url;
extern crate telegram_bot_raw;

mod api;
mod errors;
pub mod prelude;
mod stream;

pub use self::api::{Api, TelegramFuture};
pub use self::errors::{Error, Result, ErrorKind};
pub use stream::UpdatesStream;
