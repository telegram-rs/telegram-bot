#[macro_use]
pub mod reply_markup;

use serde::de::DeserializeOwned;
use serde::ser::{Serialize, Serializer};

use types::*;

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

/// Unique identifier for the target chat or username of the
/// target channel (in the format @channelusername)
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ChatId<'a> {
    ById(Integer),
    ByUsername(&'a str),
}

impl<'a> ChatId<'a> {
    /// Create chat id from numeric identifier.
    pub fn from_id(id: Integer) -> Self {
        ChatId::ById(id)
    }

    /// Create chat id from channel username.
    pub fn from_username(username: &'a str) -> Self {
        ChatId::ByUsername(username)
    }
}

impl<'a> Serialize for ChatId<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            ChatId::ById(id) => serializer.serialize_i64(id),
            ChatId::ByUsername(username) => serializer.serialize_str(username),
        }
    }
}

impl<'a, 'b> From<&'b Integer> for ChatId<'a> {
    fn from(value: &'b Integer) -> ChatId<'a> {
        ChatId::from_id(*value)
    }
}

impl<'a> From<Integer> for ChatId<'a> {
    fn from(value: Integer) -> ChatId<'a> {
        ChatId::from_id(value)
    }
}

impl<'a, 'b> From<&'b Chat> for ChatId<'a> {
    fn from(value: &'b Chat) -> ChatId<'a> {
        ChatId::from_id(value.id())
    }
}

impl<'a, 'b> From<&'b ForwardFrom> for ChatId<'a> {
    fn from(value: &'b ForwardFrom) -> ChatId<'a> {
        let id = match *value {
            ForwardFrom::User { ref user, .. } => user.id,
            ForwardFrom::Channel { ref channel, .. } => channel.id,
        };
        ChatId::from_id(id)
    }
}

macro_rules! from_chat_type {
    ($chat: ident) => {
        impl<'a, 'b> From<&'b $chat> for ChatId<'a> {
            fn from(value: &'b $chat) -> ChatId<'a> {
                ChatId::from_id(value.id)
            }
        }
    };
}

from_chat_type!(User);
from_chat_type!(Group);
from_chat_type!(Supergroup);
from_chat_type!(Channel);

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct UserId {
    inner: Integer,
}

impl<'a> Serialize for UserId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_i64(self.inner)
    }
}

impl<'a> From<&'a Integer> for UserId {
    fn from(value: &'a Integer) -> UserId {
        UserId { inner: *value }
    }
}

impl From<Integer> for UserId {
    fn from(value: Integer) -> UserId {
        UserId { inner: value }
    }
}

impl<'b> From<&'b User> for UserId {
    fn from(value: &User) -> UserId {
        UserId { inner: value.id }
    }
}

impl<'b> From<&'b ChatMember> for UserId {
    fn from(value: &ChatMember) -> UserId {
        UserId { inner: value.user.id }
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

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct MessageId(pub Integer);

impl Serialize for MessageId {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        serializer.serialize_i64(self.0)
    }
}

impl<'a> From<&'a Integer> for MessageId {
    fn from(value: &'a Integer) -> Self {
        MessageId(*value)
    }
}

impl From<Integer> for MessageId {
    fn from(value: Integer) -> Self {
        MessageId(value)
    }
}

impl<'a> From<&'a Message> for MessageId {
    fn from(value: &Message) -> Self {
        MessageId(value.id)
    }
}
