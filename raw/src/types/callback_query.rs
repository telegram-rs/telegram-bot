use types::*;
use serde::de::{Deserialize, Deserializer, Error};

/// This object represents an incoming callback query from a callback button in an inline keyboard.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct CallbackQuery {
    /// Unique identifier for this query
    pub id: CallbackQueryId,
    /// Sender
    pub from: User,
    /// Message with the callback button that originated the query.
    /// Note that message content and message date will not be available if the message is too old
    pub message: MessageCallback,
    /// Global identifier, uniquely corresponding to the chat to which the message
    /// with the callback button was sent. Useful for high scores in games.
    pub chat_instance: String,
    /// Data associated with the callback button. Be aware that a bad client can
    /// send arbitrary data in this field.
    pub data: String,
}


/// This object represents a chat message.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MessageCallback {
    /// Unique message identifier inside this chat.
    pub id: MessageId,
    /// Sender, can be empty for messages sent to channels.
    pub from: Option<User>,
    /// Date the message was sent in Unix time.
    pub date: Integer,
    /// Conversation the message belongs to.
    pub chat: MessageChat,
    /// Information about the original message.
    pub forward: Option<Forward>,
    /// For replies, the original message. Note that the Message object in this field will not
    /// contain further reply_to_message fields even if it itself is a reply.
    pub reply_to_message: Option<Box<MessageOrChannelPost>>,
    /// Date the message was last edited in Unix time.
    pub edit_date: Option<Integer>,
    /// Kind of the message.
    pub kind: MessageKind,
}

impl MessageCallback {
    fn from_raw_message(raw: RawMessage) -> Result<Self, String> {
        let id = raw.message_id;
        let from = raw.from.clone();
        let date = raw.date;
        let chat = match raw.chat.clone() {
            Chat::Private(x) => MessageChat::Private(x),
            Chat::Group(x) => MessageChat::Group(x),
            Chat::Supergroup(x) => MessageChat::Supergroup(x),
            Chat::Unknown(x) => MessageChat::Unknown(x),
            Chat::Channel(_) => return Err(format!("Channel chat in Message"))
        };

        let reply_to_message = raw.reply_to_message.clone();
        let edit_date = raw.edit_date;

        let forward = match (raw.forward_date,
                             &raw.forward_from,
                             &raw.forward_from_chat,
                             raw.forward_from_message_id) {
            (None, &None, &None, None) => None,
            (Some(date), &Some(ref from), &None, None) => {
                Some(Forward {
                    date: date,
                    from: ForwardFrom::User { user: from.clone() },
                })
            }
            (Some(date), &None, &Some(Chat::Channel(ref channel)), Some(message_id)) => {
                Some(Forward {
                    date: date,
                    from: ForwardFrom::Channel {
                        channel: channel.clone(),
                        message_id: message_id,
                    },
                })
            }
            _ => return Err(format!("invalid forward fields combination")),
        };

        let make_message = |kind| {
            Ok(MessageCallback {
                id: id.into(),
                from: from,
                date: date,
                chat: chat,
                forward: forward,
                reply_to_message: reply_to_message,
                edit_date: edit_date,
                kind: kind,
            })
        };

        macro_rules! maybe_field {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return make_message(MessageKind::$variant {
                        data: val
                    })
                }
            }}
        }

        macro_rules! maybe_field_with_caption {
            ($name:ident, $variant:ident) => {{
                if let Some(val) = raw.$name {
                    return make_message(MessageKind::$variant {
                        data: val,
                        caption: raw.caption,
                    })
                }
            }}
        }

        macro_rules! maybe_true_field {
            ($name:ident, $variant:ident) => {{
                if let Some(True) = raw.$name {
                    return make_message(MessageKind::$variant)
                }
            }}
        }

        if let Some(text) = raw.text {
            let entities = raw.entities.unwrap_or_else(Vec::new);
            return make_message(MessageKind::Text {
                data: text,
                entities: entities,
            });
        }

        maybe_field!(audio, Audio);
        maybe_field_with_caption!(document, Document);
        maybe_field_with_caption!(photo, Photo);
        maybe_field!(sticker, Sticker);
        maybe_field_with_caption!(video, Video);
        maybe_field!(voice, Voice);
        maybe_field!(video_note, VideoNote);
        maybe_field!(contact, Contact);
        maybe_field!(location, Location);
        maybe_field!(venue, Venue);
        maybe_field!(new_chat_members, NewChatMembers);
        maybe_field!(left_chat_member, LeftChatMember);
        maybe_field!(new_chat_title, NewChatTitle);
        maybe_field!(new_chat_photo, NewChatPhoto);
        maybe_true_field!(delete_chat_photo, DeleteChatPhoto);
        maybe_true_field!(delete_chat_photo, DeleteChatPhoto);
        maybe_true_field!(group_chat_created, GroupChatCreated);
        maybe_true_field!(supergroup_chat_created, SupergroupChatCreated);
        maybe_true_field!(channel_chat_created, ChannelChatCreated);
        maybe_field!(migrate_to_chat_id, MigrateToChatId);
        maybe_field!(migrate_from_chat_id, MigrateFromChatId);
        maybe_field!(pinned_message, PinnedMessage);

        make_message(MessageKind::Unknown { raw: raw })
    }
}

impl<'de> Deserialize<'de> for MessageCallback {
    fn deserialize<D>(deserializer: D) -> Result<MessageCallback, D::Error>
        where D: Deserializer<'de>
    {
        let raw: RawMessage = Deserialize::deserialize(deserializer)?;

        Self::from_raw_message(raw).map_err(|err| D::Error::custom(err))
    }
}
