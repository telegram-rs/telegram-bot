use serde::de::{Deserialize, Deserializer, Error};

use types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Message {
    pub id: Integer,
    pub from: Option<User>,
    pub date: Integer,
    pub chat: Chat,
    pub forward: Option<Forward>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<Integer>,
    pub caption: Option<String>,
    pub kind: MessageKind,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct Forward {
    pub date: Integer,
    pub from: ForwardFrom,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ForwardFrom {
    User {
        user: User,
    },
    Channel {
        channel: Channel,
        message_id: Integer
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MessageKind {
    Text {
        data: String,
        entities: Vec<MessageEntity>,
    },
    Audio {
        data: Audio,
    },
    Document {
        data: Document,
    },
    Photo {
        data: Vec<PhotoSize>,
    },
    Sticker {
        data: Sticker,
    },
    Video {
        data: Video,
    },
    Voice {
        data: Voice,
    },
    Contact {
        data: Contact,
    },
    Location {
        data: Location,
    },
    Venue {
        data: Venue,
    },
    NewChatMember {
        data: User,
    },
    LeftChatMember {
        data: User,
    },
    NewChatTitle {
        data: String,
    },
    NewChatPhoto {
        data: PhotoSize,
    },
    DeleteChatPhoto,
    GroupChatCreated,
    SupergroupChatCreated,
    ChannelChatCreated,
    MigrateToChatId {
        data: Integer,
    },
    MigrateFromChatId {
        data: Integer,
    },
    PinnedMessage {
        data: Box<Message>,
    },
    #[doc(hidden)]
    Unknown {
        raw: RawMessage
    }
}

impl Deserialize for Message {  // TODO(knsd): Remove .clone()
    fn deserialize<D>(deserializer: D) -> Result<Message, D::Error> where D: Deserializer {
        let raw: RawMessage = Deserialize::deserialize(deserializer)?;

        let id = raw.message_id;
        let from = raw.from.clone();
        let date = raw.date;
        let chat = raw.chat.clone();
        let reply_to_message = raw.reply_to_message.clone();
        let edit_date = raw.edit_date;
        let caption = raw.caption.clone();

        let forward = match (raw.forward_date, &raw.forward_from,
                             &raw.forward_from_chat, raw.forward_from_message_id) {
            (None, &None, &None, None) => None,
            (Some(date), &Some(ref from), &None, None) => {
                Some(Forward {
                    date: date,
                    from: ForwardFrom::User {
                        user: from.clone(),
                    },
                })
            },
            (Some(date), &None, &Some(Chat::Channel(ref channel)), Some(message_id)) => {
                Some(Forward {
                    date: date,
                    from: ForwardFrom::Channel {
                        channel: channel.clone(),
                        message_id: message_id,
                    },
                })
            },
            _ => {
                return Err(D::Error::custom("invalid forward fields combination"))
            }
        };

        let make_message = |kind| {
            Ok(Message {
                id: id,
                from: from,
                date: date,
                chat: chat,
                forward: forward,
                reply_to_message: reply_to_message,
                edit_date: edit_date,
                caption: caption,
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
            })
        }

        maybe_field!(audio, Audio);
        maybe_field!(document, Document);
        maybe_field!(photo, Photo);
        maybe_field!(sticker, Sticker);
        maybe_field!(video, Video);
        maybe_field!(voice, Voice);
        maybe_field!(contact, Contact);
        maybe_field!(location, Location);
        maybe_field!(venue, Venue);
        maybe_field!(new_chat_member, NewChatMember);
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

        make_message(MessageKind::Unknown {
            raw: raw,
        })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct RawMessage {
    pub message_id: Integer,
    pub from: Option<User>,
    pub date: Integer,
    pub chat: Chat,
    pub forward_from: Option<User>,
    pub forward_from_chat: Option<Chat>,
    pub forward_from_message_id: Option<Integer>,
    pub forward_date: Option<Integer>,
    pub reply_to_message: Option<Box<Message>>,
    pub edit_date: Option<Integer>,
    pub text: Option<String>,
    pub entities: Option<Vec<MessageEntity>>,
    pub audio: Option<Audio>,
    pub document: Option<Document>,
//    pub game: Option<Game>,
    pub photo: Option<Vec<PhotoSize>>,
    pub sticker: Option<Sticker>,
    pub video: Option<Video>,
    pub voice: Option<Voice>,
    pub caption: Option<String>,
    pub contact: Option<Contact>,
    pub location: Option<Location>,
    pub venue: Option<Venue>,
    pub new_chat_member: Option<User>,
    pub left_chat_member: Option<User>,
    pub new_chat_title: Option<String>,
    pub new_chat_photo: Option<PhotoSize>,
    pub delete_chat_photo: Option<True>,
    pub group_chat_created: Option<True>,
    pub supergroup_chat_created: Option<True>,
    pub channel_chat_created: Option<True>,
    pub migrate_to_chat_id: Option<Integer>,
    pub migrate_from_chat_id: Option<Integer>,
    pub pinned_message: Option<Box<Message>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct MessageEntity {
    offset: Integer,
    length: Integer,
    kind: MessageEntityKind,
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum MessageEntityKind {
    Mention,
    Hashtag,
    BotCommand,
    Url,
    Email,
    Bold,
    Italic,
    Code,
    Pre,
    TextLink(String), // TODO(knsd) URL?
    TextMention(User),
    #[doc(hidden)]
    Unknown(RawMessageEntity),
}

impl Deserialize for MessageEntity {
    fn deserialize<D>(deserializer: D) -> Result<MessageEntity, D::Error> where D: Deserializer {
        use self::MessageEntityKind::*;

        let raw: RawMessageEntity = Deserialize::deserialize(deserializer)?;

        let offset = raw.offset;
        let length = raw.length;

        macro_rules! required_field {
            ($name:ident) => {{
                match raw.$name {
                    Some(val) => val,
                    None => return Err(D::Error::missing_field(stringify!($name)))
                }
            }}
        }

        let kind = match raw.type_.as_str() {
            "mention" => Mention,
            "hashtag" => Hashtag,
            "bot_command" => BotCommand,
            "url" => Url,
            "email" => Email,
            "bold" => Bold,
            "italic" => Italic,
            "code" => Code,
            "pre" => Pre,
            "text_link" => TextLink(required_field!(url)),
            "text_mention" => TextMention(required_field!(user)),
            _ => Unknown(raw),
        };

        Ok(MessageEntity {
            offset: offset,
            length: length,
            kind: kind,
        })
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct RawMessageEntity {
    #[serde(rename="type")]
    pub type_: String,
    pub offset: Integer,
    pub length: Integer,
    pub url: Option<String>,
    pub user: Option<User>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: Integer,
    pub height: Integer,
    pub file_size: Option<Integer>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Audio {
    pub file_id: String,
    pub duration: Integer,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Document {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Sticker {
    pub file_id: String,
    pub width: Integer,
    pub height: Integer,
    pub thumb: Option<PhotoSize>,
    pub emoji: Option<String>,
    pub file_size: Option<Integer>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Video {
    pub file_id: String,
    pub width: Integer,
    pub height: Integer,
    pub duration: Integer,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Voice {
    pub file_id: String,
    pub duration: Integer,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<Integer>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Location {
    pub longitude: Float,
    pub latitude: Float,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct Venue {
    pub location: Location,
    pub title: String,
    pub address: String,
    pub foursquare_id: Option<String>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct UserProfilePhotos {
    pub total_count: Integer,
    pub photos: Vec<Vec<PhotoSize>>,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct File {
    pub file_id: String,
    pub file_size: Option<Integer>,
    pub file_path: Option<String>,
}