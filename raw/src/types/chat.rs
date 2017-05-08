use std::borrow::Cow;
use std::ops::Deref;

use serde::de::{Deserialize, Deserializer, Error};
use serde::ser::{Serialize, Serializer};

use types::*;

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

/// This object represents a Telegram user or bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct User {
    /// Unique identifier for this user or bot.
    pub id: UserId,
    /// User‘s or bot’s first name.
    pub first_name: String,
    /// User‘s or bot’s last name.
    pub last_name: Option<String>,
    /// User‘s or bot’s username.
    pub username: Option<String>,
}

/// This object represents a group.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Group {
    /// Unique identifier for this chat.
    pub id: GroupId,
    /// Title, for supergroups, channels and group chats.
    pub title: String,
    /// True if a group has ‘All Members Are Admins’ enabled.
    pub all_members_are_administrators: bool,
}

/// This object represents a supergroup.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Supergroup {
    /// Unique identifier for this chat.
    pub id: SupergroupId,
    /// Title, for supergroups, channels and group chats.
    pub title: String,
    /// Username for supergroup.
    pub username: Option<String>,
}

/// This object represents a channel.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Channel {
    /// Unique identifier for this chat.
    pub id: ChannelId,
    /// Title, for supergroups, channels and group chats.
    pub title: String,
    /// Username for channel.
    pub username: Option<String>,
}

/// This object represents a chat.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Chat {
    Private(User),
    Group(Group),
    Supergroup(Supergroup),
    Channel(Channel),
    #[doc(hidden)]
    Unknown(RawChat),
}

impl Chat {
    pub fn id(&self) -> ChatId {
        match *self {
            Chat::Private(ref x) => x.id.into(),
            Chat::Group(ref x) => x.id.into(),
            Chat::Supergroup(ref x) => x.id.into(),
            Chat::Channel(ref x) => x.id.into(),
            Chat::Unknown(ref x) => x.id.into(),
        }
    }
}

impl<'de> Deserialize<'de> for Chat {
    fn deserialize<D>(deserializer: D) -> Result<Chat, D::Error>
        where D: Deserializer<'de>
    {
        let raw: RawChat = Deserialize::deserialize(deserializer)?;

        macro_rules! required_field {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(D::Error::missing_field(stringify!($name)))
                }
            }}
        }

        Ok(match raw.type_.as_ref() {
            "private" => {
                Chat::Private(User {
                    id: raw.id.into(),
                    username: raw.username,
                    first_name: required_field!(first_name),
                    last_name: raw.last_name,
                })
            }
            "group" => {
                Chat::Group(Group {
                    id: raw.id.into(),
                    title: required_field!(title),
                    all_members_are_administrators: required_field!(all_members_are_administrators),
                })
            }
            "supergroup" => {
                Chat::Supergroup(Supergroup {
                    id: raw.id.into(),
                    title: required_field!(title),
                    username: raw.username,
                })
            }
            "channel" => {
                Chat::Channel(Channel {
                    id: raw.id.into(),
                    title: required_field!(title),
                    username: raw.username,
                })
            }
            _ => Chat::Unknown(raw),
        })
    }
}

/// This object represents a chat, directly mapped.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct RawChat {
    /// Unique identifier for this chat.
    pub id: Integer,
    /// Type of chat, can be either “private”, “group”, “supergroup” or “channel”
    #[serde(rename="type")]
    pub type_: String,
    /// Title, for supergroups, channels and group chats
    pub title: Option<String>,
    /// Username, for private chats, supergroups and channels if available
    pub username: Option<String>,
    /// First name of the other party in a private chat
    pub first_name: Option<String>,
    /// Last name of the other party in a private chat
    pub last_name: Option<String>,
    /// True if a group has ‘All Members Are Admins’ enabled.
    pub all_members_are_administrators: Option<bool>,
}
