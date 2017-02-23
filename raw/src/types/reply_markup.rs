use serde::ser::{Serialize, Serializer};

use types::primitive::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ReplyKeyboardMarkup {
    pub keyboard: Vec<Vec<KeyboardButton>>,
    pub resize_keyboard: bool,
    pub one_time_keyboard: bool,
    pub selective: bool
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct KeyboardButton {
    pub text: String,
    pub request_contact: bool,
    pub request_location: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ReplyKeyboardRemove {
    pub remove_keyboard: True,
    pub selective: bool,
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct InlineKeyboardMarkup {
    pub inline_keyboard: Vec<Vec<InlineKeyboardButton>>
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct InlineKeyboardButton {
    pub text: String,
    pub kind: InlineKeyboardButtonKind
}

impl Serialize for InlineKeyboardButton {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error> where S: Serializer {
        use self::InlineKeyboardButtonKind::*;

        let mut raw = InlineKeyboardButtonRaw {
            text: &self.text,
            url: None,
            callback_data: None,
            switch_inline_query: None,
            switch_inline_query_current_chat: None,
//            callback_game: None,
        };

        match self.kind {
            Url(ref data) => raw.url = Some(data),
            CallbackData(ref data) => raw.callback_data = Some(data),
            SwitchInlineQuery(ref data) => raw.switch_inline_query = Some(data),
            SwitchInlineQueryCurrentChat(ref data) => raw.switch_inline_query_current_chat = Some(data),
//            CallbackGame(ref data) => raw.callback_game = Some(data),
        }

        Serialize::serialize(&raw, serializer)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum InlineKeyboardButtonKind {
    Url(String), // TODO(knsd): Url?
    CallbackData(String),  //TODO(knsd) Validate size?
    SwitchInlineQuery(String),
    SwitchInlineQueryCurrentChat(String),
//    CallbackGame(CallbackGame),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
struct InlineKeyboardButtonRaw<'a> {
    text: &'a str,
    url: Option<&'a str>, // TODO(knsd): Url?
    callback_data: Option<&'a str>, //TODO(knsd) Validate size?
    switch_inline_query: Option<&'a str>,
    switch_inline_query_current_chat: Option<&'a str>,
//    callback_game: Option<CallbackGame>,
}


#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ForceReply {
    pub force_reply: True,
    pub selective: Option<bool>,
}
