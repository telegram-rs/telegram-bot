#[macro_use]
pub mod reply_markup;

use serde::de::DeserializeOwned;
use serde::ser::Serialize;

pub use self::reply_markup::*;

/// Request corresponds to the specific Telegram API method.
pub trait Request: Serialize {
    type Response;
    /// Directly mapped from Telegram API response.
    type RawResponse: DeserializeOwned;

    /// Map `RawResponse` to `Response`, `id` usually.
    fn map(raw: Self::RawResponse) -> Self::Response;

    /// Name of the method.
    fn name() -> &'static str;
}

impl<'a, Req: Request> Request for &'a Req {
    type Response = Req::Response;
    type RawResponse = Req::RawResponse;

    fn map(raw: Self::RawResponse) -> Self::Response {
        Req::map(raw)
    }

    fn name() -> &'static str {
        Req::name()
    }
}

/// Strongly typed ParseMode.
/// See [documentation](https://core.telegram.org/bots/api#formatting-options) for details.
#[derive(Debug, Clone, Copy, PartialEq, PartialOrd, Serialize)]
pub enum ParseMode {
    /// Use markdown formatting.
    Markdown,
    /// Use HTML formatting.
    #[serde(rename = "HTML")]
    Html,
}
