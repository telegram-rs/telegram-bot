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
    where
        D: Deserializer<'de>,
    {
        struct ChatMemberStatusVisitor;
        use self::ChatMemberStatus::*;

        impl<'de> Visitor<'de> for ChatMemberStatusVisitor {
            type Value = ChatMemberStatus;

            fn expecting(&self, formatter: &mut fmt::Formatter) -> fmt::Result {
                formatter.write_str("creator | administrator | member | left | kicked")
            }

            fn visit_str<E>(self, value: &str) -> Result<ChatMemberStatus, E>
            where
                E: de::Error,
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
    ///Optional. Restricted and kicked only. Date when restrictions will be lifted for this user, unix time
    pub until_date: Option<Integer>,
    ///Optional. Administrators only. True, if the bot is allowed to edit administrator privileges of that user
    pub can_be_edited: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can change the chat title, photo and other settings
    pub can_change_info: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can post in the channel, channels only
    pub can_post_messages: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can edit messages of other users and can pin messages, channels only
    pub can_edit_messages: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can delete messages of other users
    pub can_delete_messages: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can invite new users to the chat
    pub can_invite_users: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can restrict, ban or unban chat members
    pub can_restrict_members: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can pin messages, supergroups only
    pub can_pin_messages: Option<bool>,
    ///Optional. Administrators only. True, if the administrator can add new administrators with a subset of his own privileges or demote administrators that he has promoted, directly or indirectly (promoted by administrators that were appointed by the user)
    pub can_promote_members: Option<bool>,
    ///Optional. Restricted only. True, if the user can send text messages, contacts, locations and venues
    pub can_send_messages: Option<bool>,
    ///Optional. Restricted only. True, if the user can send audios, documents, photos, videos, video notes and voice notes, implies can_send_messages
    pub can_send_media_messages: Option<bool>,
    ///Optional. Restricted only. True, if the user can send animations, games, stickers and use inline bots, implies can_send_media_messages
    pub can_send_other_messages: Option<bool>,
    ///Optional. Restricted only. True, if user may add web page previews to his messages, implies can_send_media_messages
    pub can_add_web_page_previews: Option<bool>,
}
