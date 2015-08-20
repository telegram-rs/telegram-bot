//! Types of the Telegram API.
//!
//! This module contains definitions of the types defined
//! [here](https://core.telegram.org/bots/api/#available-types). Many Telegram
//! types, like "Location", map directly to Rust-structs. Other Telegram types,
//! like "Message", was made more rusty by using enums.
//!
//! All types implement `Decodable` and `Encodable`, so they can be serialized
//! as JSON. Non existing JSON-fields will result in `None` values for `Option`
//! types. `None` values don't result in JSON fields.
//!

use rustc_serialize::{Decodable, Encodable, Decoder, Encoder};
use std::convert::Into;

// ===========================================================================
// Helpers
// ===========================================================================
// Macro to implement "Encodable" quickly. "None" fields won't be encoded.
macro_rules! impl_encode {
    (
        $ty:ident, $count:expr,
        [$($id:expr => $field:ident),*],
        [$($o_id:expr => $o_field:ident),*]
    ) => {
        impl Encodable for $ty {
            fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
                e.emit_struct(stringify!($ty), $count, |e| {
                    $(
                        try!(e.emit_struct_field(stringify!($field), $id, |e| {
                            self.$field.encode(e)
                        }));
                    )*
                    $(
                        if let Some(ref v) = self.$o_field {
                            try!(e.emit_struct_field(
                                stringify!($o_field), $o_id, |e| {
                                v.encode(e)
                            }));
                        }
                    )*

                    Ok(())
                })
            }
        }
    }
}

// Decodes a field with a given name. If successful: Return decoded
// value. If not: Exit function with error value.
macro_rules! try_field {
    ($d:ident, $name:expr) => {
        try!($d.read_struct_field($name, 0, Decodable::decode))
    }
}


// ===========================================================================
// Telegram primitive types
// ===========================================================================
/// The Telegram "Integer": Currently i32.
pub type Integer = i32;
/// The Telegram "Float": Currently f32.
pub type Float = f32;


// ===========================================================================
// Types not explicitly mentioned or somehow different from Telegram types
// ===========================================================================
/// All API responses are from this type. Mostly used internal.
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct Response<T: Decodable> {
    pub ok: bool,
    pub error_code: Option<Integer>,
    pub description: Option<String>,
    pub result: Option<T>,
}

// ---------------------------------------------------------------------------
/// Represents one of "ReplyKeyboardMarkup", "ReplyKeyboardHide" or
/// "ForceReply". Used for the "reply_markup" field.
#[derive(Debug, PartialEq, Clone)]
pub enum ReplyMarkup {
    Keyboard(ReplyKeyboardMarkup),
    /// The boolean corresponds to the "selective" field of "ReplyKeyboardHide"
    KeyboardHide(bool),
    /// The boolean corresponds to the "selective" field of "ForceReply"
    ForceReply(bool),
}

impl From<ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(keyboard: ReplyKeyboardMarkup) -> ReplyMarkup {
        ReplyMarkup::Keyboard(keyboard)
    }
}

impl Encodable for ReplyMarkup {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        match *self {
            ReplyMarkup::Keyboard(ref k) => k.encode(e),
            ReplyMarkup::KeyboardHide(b) => {
                e.emit_struct("ReplyKeyboardHide", 2, |e| {
                    try!(e.emit_struct_field("hide_keyboard", 0, |e| {
                        true.encode(e)
                    }));
                    e.emit_struct_field("selective", 1, |e| b.encode(e))
                })
            },
            ReplyMarkup::ForceReply(b) => {
                e.emit_struct("ForceReply", 2, |e| {
                    try!(e.emit_struct_field("force_reply", 0, |e| {
                        true.encode(e)
                    }));
                    e.emit_struct_field("selective", 1, |e| b.encode(e))
                })
            },
        }
    }
}

// ---------------------------------------------------------------------------
/// Strongly typed ChatAction. Instead of passing a String to the
/// `send_chat_action` method, this is used.
#[derive(Debug, PartialEq, Clone, Copy)]
pub enum ChatAction {
    Typing,
    UploadPhoto,
    RecordVideo,
    UploadVideo,
    RecordAudio,
    UploadAudio,
    UploadDocument,
    FindLocation,
}

impl Decodable for ChatAction {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        Ok(match &*try!(d.read_str()) {
            "typing" => ChatAction::Typing,
            "upload_photo" => ChatAction::UploadPhoto,
            "record_video" => ChatAction::RecordVideo,
            "upload_video" => ChatAction::UploadVideo,
            "record_audio" => ChatAction::RecordAudio,
            "upload_audio" => ChatAction::UploadAudio,
            "upload_document" => ChatAction::UploadDocument,
            "find_location" => ChatAction::FindLocation,
            _ => return Err(d.error("Not a valid chat action")),
        })
    }
}

impl Into<&'static str> for ChatAction {
    fn into(self) -> &'static str {
        match self {
             ChatAction::Typing => "typing",
             ChatAction::UploadPhoto => "upload_photo",
             ChatAction::RecordVideo => "record_video",
             ChatAction::UploadVideo => "upload_video",
             ChatAction::RecordAudio => "record_audio",
             ChatAction::UploadAudio => "upload_audio",
             ChatAction::UploadDocument => "upload_document",
             ChatAction::FindLocation => "find_location",
        }
    }
}

impl ToString for ChatAction {
    fn to_string(&self) -> String {
        Into::<&str>::into(*self).into()
    }
}

impl Encodable for ChatAction {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        e.emit_str((*self).into())
    }
}

// ---------------------------------------------------------------------------
/// Either a User or a GroupChat. Used in "chat" field of Message. Has some
/// useful methods for less typing.
#[derive(Debug, PartialEq, Clone)]
pub enum Chat {
    User(User),
    Group(GroupChat),
}

impl Chat {
    /// Returns the chat id, which is needed to send messages.
    pub fn id(&self) -> Integer {
        match self {
            &Chat::User(ref u) => u.id,
            &Chat::Group(ref g) => g.id,
        }
    }

    /// Returns if the Chat is a User
    pub fn is_user(&self) -> bool {
        if let &Chat::User(_) = self { true } else { false }
    }

    /// Returns if the Chat is a Group
    pub fn is_group(&self) -> bool {
        !self.is_user()
    }
}

impl Decodable for Chat {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("", 0, |d| {
            // Both User and GroupChat have an 'id' field
            let id : Integer = try_field!(d, "id");

            // If there is a 'title' field, it's a GroupChat. A User otherwise.
            if let Some(title) = try_field!(d, "title") {
                Ok(Chat::Group(GroupChat {
                    id: id,
                    title: title,
                }))
            } else {
                Ok(Chat::User(User {
                    id: id,
                    first_name: try_field!(d, "first_name"),
                    last_name: try_field!(d, "last_name"),
                    username: try_field!(d, "username"),
                }))
            }
        })
    }
}

impl Encodable for Chat {
    fn encode<E: Encoder>(&self, e: &mut E) -> Result<(), E::Error> {
        match self {
            &Chat::User(ref u) => u.encode(e),
            &Chat::Group(ref g) => g.encode(e),
        }
    }
}

// ---------------------------------------------------------------------------
#[derive(Debug, PartialEq, Clone)]
pub struct Message {
    pub message_id: Integer,
    pub from: User,
    pub chat: Chat,
    pub date: Integer,

    // forward_from and forward_date in one
    pub forward: Option<(User, Integer)>,
    pub reply: Option<Box<Message>>,

    pub msg: MessageType,

    pub caption: Option<String>,
}

// We need to implement this on our own, because the field "msg" is not a real
// JSON field.
impl Decodable for Message {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        d.read_struct("", 0, |d| {
            Ok(Message {
                message_id: try_field!(d, "message_id"),
                from: try_field!(d, "from"),
                chat: try_field!(d, "chat"),
                date: try_field!(d, "date"),
                forward: try_field!(d, "forward"),
                reply: try_field!(d, "reply"),
                msg: try!(MessageType::decode(d)),
                caption: try_field!(d, "caption"),
            })
        })
    }
}

#[derive(Debug, PartialEq, Clone)]
pub enum MessageType {
    Text(String),
    Audio(Audio),
    Voice(Voice),
    File(Document),
    Photo(Vec<PhotoSize>),
    Sticker(Sticker),
    Video(Video),
    Contact(Contact),
    Location(Location),
    NewChatParticipant(User),
    LeftChatParticipant(User),
    NewChatTitle(String),
    NewChatPhoto(Vec<PhotoSize>),
    DeleteChatPhoto,
    GroupChatCreated,
}

impl Decodable for MessageType {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        // Tries to decode a field with the given name. If the field does NOT
        // exist: Does nothing. If the field does exist: Return the decoded
        // value. If any other decoder error occured: Return the error.
        macro_rules! maybe_field {
            ($d:ident, $name:expr, $variant:ident) => {{
                if let Some(val) = try!($d.read_struct_field(
                    $name, 0, Decodable::decode)) {
                    return Ok(MessageType::$variant(val));
                };
            }}
        }

        // There is always just one of these fields used, so we can infer the
        // enum variant from it.
        // These are the message types that carry additional data
        maybe_field!(d, "text", Text);
        maybe_field!(d, "audio", Audio);
        maybe_field!(d, "voice", Voice);
        maybe_field!(d, "file", File);
        maybe_field!(d, "photo", Photo);
        maybe_field!(d, "document", File);
        maybe_field!(d, "sticker", Sticker);
        maybe_field!(d, "video", Video);
        maybe_field!(d, "contact", Contact);
        maybe_field!(d, "location", Location);
        maybe_field!(d, "new_chat_participant", NewChatParticipant);
        maybe_field!(d, "left_chat_participant", LeftChatParticipant);
        maybe_field!(d, "new_chat_title", NewChatTitle);
        maybe_field!(d, "new_chat_photo", NewChatPhoto);

        // Message types without additional data
        if let Some(true) = try!(d.read_struct_field(
            "delete_chat_photo", 0, Decodable::decode)) {
            return Ok(MessageType::DeleteChatPhoto);
        };
        if let Some(true) = try!(d.read_struct_field(
            "group_chat_created", 0, Decodable::decode)) {
            return Ok(MessageType::GroupChatCreated);
        };

        // None of the tested fields is present: This is an error
        Err(d.error("No field for inferring message type is set"))
    }
}

// ===========================================================================
// Telegram types directly mapped to Rust types
// ===========================================================================
/// Telegram type "User" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct User {
    pub id: Integer,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

impl_encode!(User, 4,
    [0 => id, 1 => first_name],
    [2 => last_name, 3 => username]);


// ---------------------------------------------------------------------------
/// Telegram type "GroupChat" (directly mapped)
#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Clone)]
pub struct GroupChat {
    pub id: Integer,
    pub title: String,
}

// ---------------------------------------------------------------------------
/// Telegram type "PhotoSize" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct PhotoSize {
    pub file_id: String,
    pub width: Integer,
    pub height: Integer,
    pub file_size: Option<Integer>,
}

impl_encode!(PhotoSize, 4,
    [0 => file_id, 1 => width, 2 => height],
    [3 => file_size]);

// ---------------------------------------------------------------------------
/// Telegram type "Audio" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct Audio {
    pub file_id: String,
    pub duration: Integer,
    pub performer: Option<String>,
    pub title: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

impl_encode!(Audio, 6,
             [0 => file_id, 1 => duration],
             [2 => performer, 3 => title,
              4 => mime_type, 5 => file_size]);

// ---------------------------------------------------------------------------
/// Telegram type "Voice" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct Voice {
    pub file_id: String,
    pub duration: Integer,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

impl_encode!(Voice, 4,
             [0 => file_id, 1 => duration],
             [2 => mime_type, 3 => file_size]);


// ---------------------------------------------------------------------------
/// Telegram type "Document" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct Document {
    pub file_id: String,
    pub thumb: Option<PhotoSize>,
    pub file_name: Option<String>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

impl_encode!(Document, 5,
    [0 => file_id, 1 => thumb],
    [2 => file_name, 3 => mime_type, 4 => file_size]);

// ---------------------------------------------------------------------------
/// Telegram type "Sticker" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct Sticker {
    pub file_id: String,
    pub width: Integer,
    pub height: Integer,
    pub thumb: Option<PhotoSize>,
    pub file_size: Option<Integer>,
}

impl_encode!(Sticker, 5,
    [0 => file_id, 1 => width, 2 => height, 3 => thumb],
    [4 => file_size]);

// ---------------------------------------------------------------------------
/// Telegram type "Video" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct Video {
    pub file_id: String,
    pub width: Integer,
    pub height: Integer,
    pub duration: Integer,
    pub thumb: Option<PhotoSize>,
    pub mime_type: Option<String>,
    pub file_size: Option<Integer>,
}

impl_encode!(Video, 7,
    [0 => file_id, 1 => width, 2 => height, 3 => duration, 4 => thumb],
    [5 => mime_type, 6 => file_size]);

// ---------------------------------------------------------------------------
/// Telegram type "Contact" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct Contact {
    pub phone_number: String,
    pub first_name: String,
    pub last_name: Option<String>,
    pub user_id: Option<String>,
}

impl_encode!(Contact, 4,
    [0 => phone_number, 1 => first_name],
    [2 => last_name, 3 => user_id]);

// ---------------------------------------------------------------------------
/// Telegram type "Location" (directly mapped)
#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Clone, Copy)]
pub struct Location {
    pub longitude: Float,
    pub latitude: Float,
}

// ---------------------------------------------------------------------------
/// Telegram type "Update" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct Update {
    pub update_id: Integer,
    pub message: Option<Message>
}

// impl_encode!(Update, 2,
//     [0 => update_id],
//     [1 => message]);

// ---------------------------------------------------------------------------
/// Telegram type "UserProfilePhotos" (directly mapped)
#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq, Clone)]
pub struct UserProfilePhotos {
    pub total_count: Integer,
    pub photos: Vec<Vec<PhotoSize>>,
}

// ---------------------------------------------------------------------------
/// Telegram type "ReplyKeyboardMarkup" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Clone)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<String>>,
    pub resize_keyboard: Option<bool>,
    pub one_time_keyboard: Option<bool>,
    pub selective: Option<bool>,
}

impl Default for ReplyKeyboardMarkup {
    fn default() -> Self {
        ReplyKeyboardMarkup {
            keyboard: Vec::new(),
            resize_keyboard: None,
            one_time_keyboard: None,
            selective: None,
        }
    }
}

impl_encode!(ReplyKeyboardMarkup, 4,
    [0 => keyboard],
    [1 => resize_keyboard, 2 => one_time_keyboard, 3 => selective]);


// ===========================================================================
// Unit tests (mainly encode & decode)
// ===========================================================================
mod test;
