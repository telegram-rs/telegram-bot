#[macro_use]
extern crate error_chain;
extern crate futures;
extern crate hyper;
extern crate hyper_tls;
extern crate serde_json;
extern crate tokio_core;
extern crate telegram_bot_raw;

mod api;
mod connector;
mod errors;
mod future;
mod stream;

pub mod prelude;
pub mod types;

pub use self::api::Api;
pub use self::errors::{Error, Result, ErrorKind};
pub use self::future::TelegramFuture;
pub use stream::UpdatesStream;
pub use prelude::*;
pub use types::*;