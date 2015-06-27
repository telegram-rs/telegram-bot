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
                    $
(                        if let Some(ref v) = self.$o_field {
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



// ===========================================================================
// Telegram primitive types
// ===========================================================================
/// The Telegram "Integer": Currently i32.
pub type Integer = i32;
/// The Telegram "Float": Currently f32.
pub type Float = f32;

#[derive(RustcDecodable, Debug)]
pub struct Response<T: Decodable> {
    pub ok: bool,
    pub description: Option<String>,
    pub result: Option<T>,
}

#[derive(RustcDecodable, Debug)]
pub struct GroupChat {
    pub id: Integer,
    pub title: String,
}

#[derive(Debug)]
pub enum Chat {
    User(User),
    Group(GroupChat),
}

impl Chat {
    pub fn id(&self) -> Integer {
        match self {
            &Chat::User(ref u) => u.id,
            &Chat::Group(ref g) => g.id,
        }
    }
}

impl Decodable for Chat {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        match User::decode(d) {
            Ok(user) => Ok(Chat::User(user)),
            Err(_) => Ok(Chat::Group(try!(Decodable::decode(d)))),
        }
    }
}

#[derive(RustcDecodable, Debug)]
pub struct Update {
    pub update_id: Integer,
    pub message: Option<Message>
}

#[derive(Debug)]
pub struct Message {
    pub message_id: Integer,
    pub from: User,
    pub chat: Chat,
    pub date: Integer,

    // forward_from and forward_date in one
    pub forward: Option<(User, Integer)>,
    pub reply: Option<Box<Message>>,

    pub msg: MessageType,
}

impl Decodable for Message {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        // Decodes a field with a given name. If successful: Return decoded
        // value. If not: Exist function with error value.
        macro_rules! try_field {
            ($d:ident, $name:expr) => {
                try!($d.read_struct_field($name, 0, |d| Decodable::decode(d)))
            }
        }

        d.read_struct("", 0, |d| {
            Ok(Message {
                message_id: try_field!(d, "message_id"),
                from:       try_field!(d, "from"),
                chat:       try_field!(d, "chat"),
                date:       try_field!(d, "date"),
                forward:    try_field!(d, "forward"),
                reply:      try_field!(d, "reply"),
                msg: try!(MessageType::decode(d)),
            })
        })
    }
}

#[derive(Debug)]
pub enum MessageType {
    Text(String),
    Audio,
    File,
    Photo,
    Sticker,
    Video,
    Contact(Contact),
    Location(Location),
    NewChatParticipant(User),
    LeftChatParticipant(User),
    NewChatTitle(String),
    NewChatPhoto,
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
                    $name, 0, |d| Decodable::decode(d))) {
                    return Ok(MessageType::$variant(val));
                };
            }}
        }

        // There is always just one of these fields used, so we can infer the
        // enum variant from it.
        maybe_field!(d, "text", Text);
        maybe_field!(d, "contact", Contact);
        maybe_field!(d, "location", Location);
        maybe_field!(d, "new_chat_participant", NewChatParticipant);
        maybe_field!(d, "left_chat_participant", LeftChatParticipant);
        maybe_field!(d, "new_chat_title", NewChatTitle);

        // delete_chat_photo
        if let Some(true) = try!(d.read_struct_field(
            "delete_chat_photo", 0, |d| Decodable::decode(d))) {
            return Ok(MessageType::DeleteChatPhoto);
        };
        // group_chat_created
        if let Some(true) = try!(d.read_struct_field(
            "group_chat_created", 0, |d| Decodable::decode(d))) {
            return Ok(MessageType::GroupChatCreated);
        };

        // TODO: Implement missing MessageType's. If no field is set: Error.
        Ok(MessageType::Text("!!! Not implemented !!!".into()))
    }
}

// ===========================================================================
// Telegram types directly mapped to Rust types
// ===========================================================================

/// Telegram type "ReplyKeyboardMarkup" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Eq)]
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


// ---------------------------------------------------------------------------
/// Telegram type "PhotoSize" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Eq)]
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
/// Telegram type "Contact" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Eq)]
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
#[derive(RustcDecodable, RustcEncodable, Debug, PartialEq)]
pub struct Location {
    pub longitude: Float,
    pub latitude: Float,
}

// ---------------------------------------------------------------------------
/// Telegram type "User" (directly mapped)
#[derive(RustcDecodable, Debug, PartialEq, Eq)]
pub struct User {
    pub id: Integer,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
}

impl_encode!(User, 4,
    [0 => id, 1 => first_name],
    [2 => last_name, 3 => username]);


// ===========================================================================
// Unit tests (mainly encode & decode)
// ===========================================================================
mod test;
