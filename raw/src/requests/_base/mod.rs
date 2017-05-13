#[macro_use]
pub mod reply_markup;

use serde::de::DeserializeOwned;
use serde::ser::{Serialize, Serializer, Error};
use serde_json::{Value, to_value};

use types::*;

pub use self::reply_markup::*;

pub trait Response {
    type Raw: DeserializeOwned;
    type Type;

    fn map(Self::Raw) -> Self::Type;
}

pub struct IdResponse<T> {
    _phantom: ::std::marker::PhantomData<T>
}

impl<T: DeserializeOwned> Response for IdResponse<T> {
    type Raw = T;
    type Type = T;

    fn map(value: Self::Raw) -> Self::Type {
        value
    }
}

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
    type Response: Response + Send + 'static;

    /// Name of the method.
    fn name(&self) -> &'static str;

    fn detach(&self) -> DetachedRequest<Self::Response> {
        DetachedRequest {
            name: self.name(),
            encoded: to_value(self).map_err(|err| format!("{}", err)),
            phantom: ::std::marker::PhantomData,
        }
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

pub trait ToRequest<'b, 'c> {
    type Request: Request;
    fn to_request<C>(&'b self, chat: C) -> Self::Request where C: ToChatRef<'c>;
}

pub trait ToReplyRequest<'b, 'c> {
    type Request: Request;
    fn to_reply_request(&'b self, message: &Message) -> Self::Request;
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
