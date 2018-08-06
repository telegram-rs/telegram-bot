//! This crate helps writing bots for the messenger Telegram.
//! See [readme](https://github.com/telegram-rs/telegram-bot) for details.

extern crate antidote;
#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate http;
extern crate telegram_bot_raw;
extern crate tokio;
extern crate tokio_timer;

#[cfg(feature = "hyper_connector")]
extern crate hyper;
#[cfg(feature = "hyper_connector")]
extern crate hyper_tls;

mod api;
mod errors;
mod future;
mod macros;
mod stream;

pub mod connector;
pub mod prelude;
pub mod types;

pub use self::api::{Api, Config};
pub use self::errors::{Error, ErrorKind};
pub use self::future::TelegramFuture;
pub use connector::*;
pub use prelude::*;
pub use stream::UpdatesStream;
pub use types::*;
