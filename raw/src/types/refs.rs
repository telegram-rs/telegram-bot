use std::borrow::Cow;
use std::ops::Deref;

use serde::ser::{Serialize, Serializer};

use types::*;

macro_rules! integer_id_impls {
    ($name: ident) => {
        impl $name {
            pub fn new(inner: Integer) -> Self {
                $name(inner)
            }
        }

        impl ::std::fmt::Display for $name {
            fn fmt(&self, f: &mut ::std::fmt::Formatter) -> ::std::fmt::Result {
                self.0.fmt(f)
            }
        }

        impl From<Integer> for $name {
            fn from(inner: Integer) -> Self {
                $name::new(inner)
            }
        }

        impl From<$name> for Integer {
            fn from(from: $name) -> Self {
                from.0
            }
        }

        impl<'de> ::serde::de::Deserialize<'de> for $name {
            fn deserialize<D>(deserializer: D) -> Result<$name, D::Error>
                where D: ::serde::de::Deserializer<'de>
            {
                let inner = ::serde::de::Deserialize::deserialize(deserializer)?;
                Ok($name::new(inner))
            }
        }

        impl ::serde::ser::Serialize for $name {
            fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
                where S: ::serde::ser::Serializer
            {
                serializer.serialize_i64(self.0)
            }
        }
    };
}

/// Unique identifier for the target chat or username of the
/// target channel (in the format @channelusername)
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChatRef<'a> {
    Id(ChatId),
    #[doc(hidden)]
    ChannelUsername(Cow<'a, str>,),
}

impl<'a> ChatRef<'a> {
    pub fn from_chat_id(chat_id: ChatId) -> ChatRef<'a> {
        ChatRef::Id(chat_id)
    }
}

pub trait ToChatRef<'a> {
    fn to_chat_ref(&self) -> ChatRef<'a>;
}

impl<'a, S> ToChatRef<'a> for S where S: Deref, S::Target: ToChatRef<'a> {
    fn to_chat_ref(&self) -> ChatRef<'a> {
        self.deref().to_chat_ref()
    }
}

impl<'a> ToChatRef<'a> for ChatRef<'a> {
    fn to_chat_ref(&self) -> ChatRef<'a> {
        self.clone()
    }
}

impl<'a> ToChatRef<'a> for Chat {
    fn to_chat_ref(&self) -> ChatRef<'a> {
        self.id().to_chat_ref()
    }
}

impl<'a> ToChatRef<'a> for ChatMember {
    fn to_chat_ref(&self) -> ChatRef<'a> {
        self.user.to_chat_ref()
    }
}

impl<'a> ToChatRef<'a> for ForwardFrom {
    fn to_chat_ref(&self) -> ChatRef<'a> {
        match *self {
            ForwardFrom::User {ref user, ..} => user.to_chat_ref(),
            ForwardFrom::Channel {ref channel, ..} => channel.to_chat_ref(),
        }
    }
}

impl<'a> ToChatRef<'a> for Forward {
    fn to_chat_ref(&self) -> ChatRef<'a> {
        self.from.to_chat_ref()
    }
}

impl<'a> Serialize for ChatRef<'a> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
        where S: Serializer
    {
        match *self {
            ChatRef::Id(id) => serializer.serialize_i64(id.into()),
            ChatRef::ChannelUsername(ref username) => serializer.serialize_str(&username),
        }
    }
}

macro_rules! chat_id_impls {
    ($id: ident) => {
        integer_id_impls!($id);

        impl<'a> ToChatRef<'a> for $id {
            fn to_chat_ref(&self) -> ChatRef<'a> {
                ChatRef::from_chat_id((*self).into())
            }
        }
    };
}

macro_rules! specific_chat_id_impls {
    ($id: ident, $typ: ident) => {
        chat_id_impls!($id);

        impl From<$id> for ChatId {
            fn from(c: $id) -> Self {
                ChatId::new(c.into())
            }
        }

        impl<'a> ToChatRef<'a> for $typ {
            fn to_chat_ref(&self) -> ChatRef<'a> {
                self.id.to_chat_ref()
            }
        }
    };
}

pub trait ToUserId {
    fn to_user_id(&self) -> UserId;
}

impl<S> ToUserId for S where S: Deref, S::Target: ToUserId {
    fn to_user_id(&self) -> UserId {
        self.deref().to_user_id()
    }
}

impl ToUserId for UserId {
    fn to_user_id(&self) -> UserId {
        *self
    }
}

impl ToUserId for ChatMember {
    fn to_user_id(&self) -> UserId {
        self.user.id
    }
}

impl ToUserId for User {
    fn to_user_id(&self) -> UserId {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct UserId(Integer);
specific_chat_id_impls!(UserId, User);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct GroupId(Integer);
specific_chat_id_impls!(GroupId, Group);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct SupergroupId(Integer);
specific_chat_id_impls!(SupergroupId, Supergroup);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChannelId(Integer);
specific_chat_id_impls!(ChannelId, Channel);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct ChatId(Integer);
chat_id_impls!(ChatId);

pub trait ToMessageId {
    fn to_message_id(&self) -> MessageId;
}

impl<S> ToMessageId for S where S: Deref, S::Target: ToMessageId {
    fn to_message_id(&self) -> MessageId {
        self.deref().to_message_id()
    }
}

impl ToMessageId for MessageId {
    fn to_message_id(&self) -> MessageId {
        *self
    }
}

impl ToMessageId for Message {
    fn to_message_id(&self) -> MessageId {
        self.id
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct MessageId(Integer);
integer_id_impls!(MessageId);
