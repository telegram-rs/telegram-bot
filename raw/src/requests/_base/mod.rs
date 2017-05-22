#[macro_use]
pub mod reply_markup;

use serde::de::DeserializeOwned;
use serde::ser::{Serialize, Serializer, Error};
use serde_json::{Value, to_value};

use types::*;
use url::*;

pub use self::reply_markup::*;

/// This trait represent how to parse response from the Telegram server.
pub trait Response {
    /// Response from the Telegram server.
    type Raw: DeserializeOwned;
    /// Response that should be returned.
    type Type;

    /// Map `Raw` to `Type`.
    fn map(raw: Self::Raw) -> Self::Type;
}

/// Read `T` from the Telegram server and returns the same type.
pub struct IdResponse<T> {
    phantom: ::std::marker::PhantomData<T>
}

impl<T: DeserializeOwned> Response for IdResponse<T> {
    type Raw = T;
    type Type = T;

    fn map(raw: Self::Raw) -> Self::Type {
        raw
    }
}

/// Read `True` from the Telegram server and returns `()`.
pub struct TrueToUnitResponse;

impl Response for TrueToUnitResponse {
    type Raw = True;
    type Type = ();

    fn map(_: Self::Raw) -> Self::Type {
        ()
    }
}

/// Request corresponds to the specific Telegram API method.
pub trait Request: Serialize {
    /// How to parse response from the Telegram server.
    type Response: Response + Send + 'static;

    /// Name of the method.
    fn name(&self) -> &'static str;

    /// Detach request from lifetime bounds, so it can be transferred across thread boundaries.
    fn detach(&self) -> DetachedRequest<Self::Response> {
        DetachedRequest {
            name: self.name(),
            encoded: to_value(self).map_err(|err| format!("{}", err)),
            phantom: ::std::marker::PhantomData,
        }
    }

    fn get_url(&self, token: &str) -> String {
        format!("{}bot{}/{}", TELEGRAM_URL, token, self.name())
    }
}

impl<'a, Req: Request> Request for &'a Req {
    type Response = Req::Response;

    fn name(&self) -> &'static str {
        (*self).name()
    }
}

impl<'a, Req: Request> Request for &'a mut Req {
    type Response = Req::Response;

    fn name(&self) -> &'static str {
        (**self).name()
    }
}

/// Partially serialized request.
pub struct DetachedRequest<Resp> {
    name: &'static str,
    encoded: Result<Value, String>,
    phantom: ::std::marker::PhantomData<Resp>,
}

impl<Resp: Response + Send + 'static> Request for DetachedRequest<Resp> {
    type Response = Resp;

    fn name(&self) -> &'static str {
        self.name
    }
}

impl<Resp> Serialize for DetachedRequest<Resp> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {

        match self.encoded {
            Ok(ref value) => value.serialize(serializer),
            Err(ref err) => Err(S::Error::custom(err)),
        }
    }
}

/// Use this trait to convert a complex type to corresponding request and send it to the chat.
pub trait ToRequest<'b> {
    /// Request type.
    type Request: Request;

    /// Convert type to request and send it to the chat.
    fn to_request<C>(&'b self, chat: C) -> Self::Request where C: ToChatRef;
}

/// Use this trait to convert a complex type to corresponding request and reply to the message.
pub trait ToReplyRequest<'b> {
    /// Request type.
    type Request: Request;

    /// Convert type to request and reply to the message.
    fn to_reply_request<M>(&'b self, message: M) -> Self::Request
        where M: ToMessageId + ToSourceChat;
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
