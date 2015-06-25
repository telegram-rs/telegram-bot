use rustc_serialize::{json, Decodable, Decoder};

type Integer = i32;

#[derive(RustcDecodable, Debug)]
pub struct Response<T: Decodable> {
    pub ok: bool,
    pub description: Option<String>,
    pub result: Option<T>,
}

#[derive(RustcDecodable, Debug)]
pub struct User {
    id: Integer,
    first_name: String,
    last_name: Option<String>,
    username: Option<String>,
}

#[derive(RustcDecodable, Debug)]
pub struct GroupChat {
    id: Integer,
    title: String,
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
    update_id: Integer,
    message: Option<Message>
}

#[derive(Debug)]
pub struct Message {
    message_id: Integer,
    from: User,
    chat: Chat,
    date: Integer,

    // forward_from and forward_date in one
    forward: Option<(User, Integer)>,
    reply: Option<Box<Message>>,

    msg: MessageType,
}

impl Decodable for Message {
    fn decode<D: Decoder>(d: &mut D) -> Result<Self, D::Error> {
        macro_rules! try_field {
            ($d:ident, $name:expr) => {
                try!($d.read_struct_field($name, 0, |d| Decodable::decode(d)))
            }
        }

        let decode_msg = |d| {
            MessageType::Text("not implemented".into())
        };


        d.read_struct("", 0, |d| {
            Ok(Message {
                message_id: try_field!(d, "message_id"),
                from:       try_field!(d, "from"),
                chat:       try_field!(d, "chat"),
                date:       try_field!(d, "date"),
                forward:    try_field!(d, "forward"),
                reply:      try_field!(d, "reply"),
                msg: decode_msg(d),
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
