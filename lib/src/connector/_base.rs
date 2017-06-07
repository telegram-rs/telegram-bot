use std::fmt::Debug;

use future::TelegramFuture;

use telegram_bot_raw::{HttpRequest, HttpResponse};

/// Connector provides basic IO with Telegram Bot API server.
pub trait Connector: Debug {
    fn request(&self, token: &str, req: HttpRequest) -> TelegramFuture<HttpResponse>;
}
