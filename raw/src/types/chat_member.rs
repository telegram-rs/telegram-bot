use std::fmt;

use serde::de;
use serde::de::{Deserialize, Deserializer, Visitor};

use types::*;

/// The member's status in the chat
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub enum ChatMemberStatus {
    Creator,
    Administrator,
    Member,
    Left,
    Kicked,
    #[doc(hidden)]
    Unknown(String),
}

impl<'de> Deserialize<'de> for ChatMemberStatus {
    fn deserialize<D>(deserializer: D) -> Result<ChatMemberStatus, D::Error>
        where D: Deserializer<'de>
    {
        struct ChatMemberStatusVisitor;
        use self::ChatMemberStatus::*;

        impl<'de> Visitor<'de> for ChatMemberStatusVisitor {
            type Value = ChatMemberStatus;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("creator | administrator | member | left | kicked")
            }

            fn visit_str<E>(self, value: &str) -> Result<ChatMemberStatus, E>
                where E: de::Error
            {
                Ok(match value {
                    "creator" => Creator,
                    "administrator" => Administrator,
                    "member" => Member,
                    "left" => Left,
                    "kicked" => Kicked,
                    _unknown => Unknown(value.to_string()),
                })
            }
        }

        deserializer.deserialize_str(ChatMemberStatusVisitor)
    }
}

/// This object contains information about one member of the chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Eq, Ord, Hash, Deserialize)]
pub struct ChatMember {
    /// Information about the user.
    pub user: User,
    /// The member's status in the chat.
    pub status: ChatMemberStatus,
}
