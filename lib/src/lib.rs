//! This crate helps writing bots for the messenger Telegram.
//! See [readme](https://github.com/telegram-rs/telegram-bot) for details.

mod api;
mod errors;
mod macros;
mod stream;

pub mod connector;
pub mod prelude;
pub mod types;
pub mod util;

pub use self::api::Api;
pub use self::errors::Error;
pub use prelude::*;
pub use stream::UpdatesStream;
pub use types::*;
