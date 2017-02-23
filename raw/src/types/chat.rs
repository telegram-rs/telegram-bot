use serde::de::{Deserialize, Deserializer, Error};

use types::primitive::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct User {
    pub id: Integer,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Group {
    pub id: Integer,
    pub title: String,
    pub all_members_are_administrators: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Supergroup {
    pub id: Integer,
    pub title: String,
    pub username: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Channel {
    pub id: Integer,
    pub title: String,
    pub username: Option<String>,
}


#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum Chat {
    Private(User),
    Group(Group),
    Supergroup(Supergroup),
    Channel(Channel),
    Unknown(RawChat),
}

impl Deserialize for Chat {
    fn deserialize<D>(deserializer: D) -> Result<Chat, D::Error> where D: Deserializer {
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
                    id: raw.id,
                    username: raw.username,
                    first_name: required_field!(first_name),
                    last_name: raw.last_name,
                })
            },
            "group" => {
                Chat::Group(Group {
                    id: raw.id,
                    title: required_field!(title),
                    all_members_are_administrators: required_field!(all_members_are_administrators),
                })
            },
            "supergroup" => Chat::Supergroup(Supergroup {
                id: raw.id,
                title: required_field!(title),
                username: raw.username,
            }),
            "channel" => Chat::Channel(Channel {
                id: raw.id,
                title: required_field!(title),
                username: raw.username,
            }),
            _ => Chat::Unknown(raw)
        })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct RawChat {
    pub id: Integer,
    #[serde(rename="type")]
    pub type_: String,
    pub title: Option<String>,
    pub username: Option<String>,
    pub first_name: Option<String>,
    pub last_name: Option<String>,
    pub all_members_are_administrators: Option<bool>,
}
