use rustc_serialize::{Decodable, Decoder};
// use std::collections::BTreeMap;

pub type Integer = i32;

#[derive(RustcDecodable, Debug)]
pub struct Response<T: Decodable> {
    pub ok: bool,
    pub description: Option<String>,
    pub result: Option<T>,
}

#[derive(RustcDecodable, Debug)]
pub struct User {
    pub id: Integer,
    pub first_name: String,
    pub last_name: Option<String>,
    pub username: Option<String>,
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
    Contact,
    Location,
    GroupAdd(User),
    GroupRemove(User),
    GroupTitleChange(String),
    GroupPhotoChange,
    GroupPhotoDelete,
    GroupCreate,
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
        maybe_field!(d, "new_chat_participant", GroupAdd);
        maybe_field!(d, "left_chat_participant", GroupRemove);
        maybe_field!(d, "new_chat_title", GroupTitleChange);

        // TODO: Implement missing MessageType's. If no field is set: Error.
        Ok(MessageType::Text("!!! Not implemented !!!".into()))
    }
}
