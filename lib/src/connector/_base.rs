use std::fmt::Debug;

use future::TelegramFuture;

/// Connector provides basic IO with Telegram Bot API server.
pub trait Connector: Debug {
    /// Make POST request with `application/json` content type.
    fn post_json(&self, uri: &str, data: Vec<u8>) -> TelegramFuture<Vec<u8>>;
}
