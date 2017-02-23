use std::fmt;

use serde::de;
use serde::de::{Deserialize, Deserializer, Visitor};

use types::chat::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ChatMemberStatus {
    Creator,
    Administrator,
    Member,
    Left,
    Kicked,
    Unknown(String),
}

impl Deserialize for ChatMemberStatus {
    fn deserialize<D>(deserializer: D) -> Result<ChatMemberStatus, D::Error> where D: Deserializer {
        struct ChatMemberStatusVisitor;
        use self::ChatMemberStatus::*;

        impl Visitor for ChatMemberStatusVisitor {
            type Value = ChatMemberStatus;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("creator | administrator | member | left | kicked")
            }

            fn visit_str<E>(self, value: &str) -> Result<ChatMemberStatus, E> where E: de::Error {
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

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ChatMember {
    pub user: User,
    pub status: ChatMemberStatus,
}