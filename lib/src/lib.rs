//! This crate helps writing bots for the messenger Telegram.
//! See [readme](https://github.com/telegram-rs/telegram-bot) for details.

#[macro_use]
extern crate error_chain;

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
