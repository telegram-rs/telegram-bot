mod reply_markup;

use serde::ser::{Serialize, Serializer};

use types::chat::*;
use types::message::*;
use types::primitive::*;
use requests::_base::reply_markup::*;

pub trait Request {
    type Response;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ChatId<'a> {
    ById(Integer),
    ByUsername(&'a str),
}

impl<'a> ChatId<'a> {
    pub fn from_id(id: Integer) -> Self {
        ChatId::ById(id)
    }

    pub fn from_username(username: &'a str) -> Self {
        ChatId::ByUsername(username)
    }
}

impl<'a> Serialize for ChatId<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            ChatId::ById(id) => serializer.serialize_i64(id),
            ChatId::ByUsername(username) => serializer.serialize_str(username),
        }
    }
}

impl<'a> From<Integer> for ChatId<'a> {
    fn from(value: Integer) -> ChatId<'a> {
        ChatId::from_id(value)
    }
}

impl<'a> From<&'a Chat> for ChatId<'a> {
    fn from(value: &'a Chat) -> ChatId<'a> {
        ChatId::from_id(value.id())
    }
}

macro_rules! from_chat_type {
    ($chat: ident) => {
        impl<'a> From<&'a $chat> for ChatId<'a> {
            fn from(value: &'a $chat) -> ChatId<'a> {
                ChatId::from_id(value.id)
            }
        }
    };
}

from_chat_type!(User);
from_chat_type!(Group);
from_chat_type!(Supergroup);
from_chat_type!(Channel);

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub enum ParseMode {
    Markdown,
    HTML
}

impl Serialize for ParseMode {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_str(match *self {
            ParseMode::Markdown => "markdown",
            ParseMode::HTML => "HTML",
        })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

impl Serialize for ReplyMarkup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        match *self {
            ReplyMarkup::InlineKeyboardMarkup(ref value) => value.serialize(serializer),
            ReplyMarkup::ReplyKeyboardMarkup(ref value) => value.serialize(serializer),
            ReplyMarkup::ReplyKeyboardRemove(ref value) => value.serialize(serializer),
            ReplyMarkup::ForceReply(ref value) => value.serialize(serializer),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
pub struct ReplyTo(pub Integer);

impl Serialize for ReplyTo {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        serializer.serialize_i64(self.0)
    }
}

impl From<Integer> for ReplyTo {
    fn from(value: Integer) -> Self {
        ReplyTo(value)
    }
}

impl<'a> From<&'a Message> for ReplyTo {
    fn from(value: &Message) -> Self {
        ReplyTo(value.id)
    }
}
