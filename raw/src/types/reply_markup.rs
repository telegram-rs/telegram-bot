use std::ops::Not;

use serde::ser::{Serialize, Serializer};

use types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum ReplyMarkup {
    InlineKeyboardMarkup(InlineKeyboardMarkup),
    ReplyKeyboardMarkup(ReplyKeyboardMarkup),
    ReplyKeyboardRemove(ReplyKeyboardRemove),
    ForceReply(ForceReply),
}

impl From<InlineKeyboardMarkup> for ReplyMarkup {
    fn from(value: InlineKeyboardMarkup) -> ReplyMarkup {
        ReplyMarkup::InlineKeyboardMarkup(value)
    }
}

impl From<ReplyKeyboardMarkup> for ReplyMarkup {
    fn from(value: ReplyKeyboardMarkup) -> ReplyMarkup {
        ReplyMarkup::ReplyKeyboardMarkup(value)
    }
}

impl From<ReplyKeyboardRemove> for ReplyMarkup {
    fn from(value: ReplyKeyboardRemove) -> ReplyMarkup {
        ReplyMarkup::ReplyKeyboardRemove(value)
    }
}

impl From<ForceReply> for ReplyMarkup {
    fn from(value: ForceReply) -> ReplyMarkup {
        ReplyMarkup::ForceReply(value)
    }
}

impl Serialize for ReplyMarkup {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        match *self {
            ReplyMarkup::InlineKeyboardMarkup(ref value) => value.serialize(serializer),
            ReplyMarkup::ReplyKeyboardMarkup(ref value) => value.serialize(serializer),
            ReplyMarkup::ReplyKeyboardRemove(ref value) => value.serialize(serializer),
            ReplyMarkup::ForceReply(ref value) => value.serialize(serializer),
        }
    }
}

/// This object represents a custom keyboard with reply options.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ReplyKeyboardMarkup {
    keyboard: Vec<Vec<KeyboardButton>>,
    #[serde(skip_serializing_if = "Not::not")]
    resize_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    one_time_keyboard: bool,
    #[serde(skip_serializing_if = "Not::not")]
    selective: bool,
}

impl ReplyKeyboardMarkup {
    pub fn new() -> Self {
        ReplyKeyboardMarkup {
            keyboard: Vec::new(),
            resize_keyboard: false,
            one_time_keyboard: false,
            selective: false,
        }
    }

    fn init(rows: Vec<Vec<KeyboardButton>>) -> Self {
        let mut keyboard = Self::new();
        keyboard.keyboard = rows;
        keyboard
    }

    /// Requests clients to resize the keyboard vertically for
    /// optimal fit (e.g., make the keyboard smaller if there
    /// are just two rows of buttons). Defaults to false, in which case
    /// the custom keyboard is always of the same height as the app's standard keyboard.
    pub fn resize_keyboard(&mut self) -> &mut Self {
        self.resize_keyboard = true;
        self
    }

    /// Requests clients to hide the keyboard as soon as it's been used.
    /// The keyboard will still be available, but clients will automatically
    /// display the usual letter-keyboard in the chat – the user can
    /// press a special button in the input field to see the custom
    /// keyboard again. Defaults to false.
    pub fn one_time_keyboard(&mut self) -> &mut Self {
        self.one_time_keyboard = true;
        self
    }

    /// Use this method if you want to force reply from specific users only.
    /// Targets: 1) users that are @mentioned in the text of
    /// the Message object; 2) if the bot's message is a reply (has reply_to_message_id),
    /// sender of the original message.
    pub fn selective(&mut self) -> &mut Self {
        self.selective = true;
        self
    }

    pub fn add_row(&mut self, row: Vec<KeyboardButton>) -> &mut Vec<KeyboardButton> {
        self.keyboard.push(row);
        self.keyboard.last_mut().unwrap()
    }

    pub fn add_empty_row(&mut self) -> &mut Vec<KeyboardButton> {
        self.add_row(Default::default())
    }
}

impl From<Vec<Vec<KeyboardButton>>> for ReplyKeyboardMarkup {
    fn from(value: Vec<Vec<KeyboardButton>>) -> Self {
        Self::init(value)
    }
}

/// This object represents one button of the reply keyboard.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct KeyboardButton {
    text: String,
    #[serde(skip_serializing_if = "Not::not")]
    request_contact: bool,
    #[serde(skip_serializing_if = "Not::not")]
    request_location: bool,
}

impl KeyboardButton {
    pub fn new<S: AsRef<str>>(text: S) -> Self {
        Self {
            text: text.as_ref().to_string(),
            request_contact: false,
            request_location: false,
        }
    }

    /// The user's phone number will be sent as a contact when the
    /// button is pressed. Available in private chats only
    pub fn request_contact(&mut self) -> &mut Self {
        self.request_location = false;
        self.request_contact = true;
        self
    }

    /// The user's current location will be sent when the
    /// button is pressed. Available in private chats only
    pub fn request_location(&mut self) -> &mut Self {
        self.request_contact = false;
        self.request_location = true;
        self
    }
}

impl<'a> From<&'a str> for KeyboardButton {
    fn from(value: &'a str) -> KeyboardButton {
        KeyboardButton::new(value)
    }
}

impl From<String> for KeyboardButton {
    fn from(value: String) -> KeyboardButton {
        KeyboardButton::new(value)
    }
}

/// Upon receiving a message with this object, Telegram clients will remove
/// the current custom keyboard and display the default letter-keyboard.
/// By default, custom keyboards are displayed until a new keyboard is sent
/// by a bot. An exception is made for one-time keyboards that are hidden
/// immediately after the user presses a button (see ReplyKeyboardMarkup).
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ReplyKeyboardRemove {
    remove_keyboard: True,
    #[serde(skip_serializing_if = "Not::not")]
    selective: bool,
}

impl ReplyKeyboardRemove {
    pub fn new() -> Self {
        Self {
            remove_keyboard: True,
            selective: false,
        }
    }

    /// Use this method if you want to force reply from specific users only.
    /// Targets: 1) users that are @mentioned in the text of
    /// the Message object; 2) if the bot's message is a reply (has reply_to_message_id),
    /// sender of the original message.
    pub fn selective(&mut self) -> &mut Self {
        self.selective = true;
        self
    }
}

/// This object represents an inline keyboard that appears right next to the message it belongs to.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct InlineKeyboardMarkup {
    inline_keyboard: Vec<Vec<InlineKeyboardButton>>,
}

impl InlineKeyboardMarkup {
    pub fn new() -> Self {
        Self {
            inline_keyboard: Default::default(),
        }
    }

    fn init(inline_keyboard: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self { inline_keyboard }
    }

    pub fn add_row(&mut self, row: Vec<InlineKeyboardButton>) -> &mut Vec<InlineKeyboardButton> {
        self.inline_keyboard.push(row);
        self.inline_keyboard.last_mut().unwrap()
    }

    pub fn add_empty_row(&mut self) -> &mut Vec<InlineKeyboardButton> {
        self.add_row(Default::default())
    }
}

impl From<Vec<Vec<InlineKeyboardButton>>> for InlineKeyboardMarkup {
    fn from(value: Vec<Vec<InlineKeyboardButton>>) -> Self {
        Self::init(value)
    }
}

/// This object represents one button of an inline keyboard.
#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct InlineKeyboardButton {
    text: String,
    kind: InlineKeyboardButtonKind,
}

impl InlineKeyboardButton {
    /// Data to be sent in a callback query to the bot when button is pressed, 1-64 bytes
    pub fn callback<T: AsRef<str>, C: AsRef<str>>(text: T, callback: C) -> Self {
        Self {
            text: text.as_ref().to_string(),
            kind: InlineKeyboardButtonKind::CallbackData(callback.as_ref().to_string()),
        }
    }
}

impl Serialize for InlineKeyboardButton {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
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
            //            Url(ref data) => raw.url = Some(data),
            CallbackData(ref data) => raw.callback_data = Some(data),
            //            SwitchInlineQuery(ref data) => raw.switch_inline_query = Some(data),
            //            SwitchInlineQueryCurrentChat(ref data) => raw.switch_inline_query_current_chat = Some(data),
            //            CallbackGame(ref data) => raw.callback_game = Some(data),
        }

        Serialize::serialize(&raw, serializer)
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum InlineKeyboardButtonKind {
    //    Url(String), // TODO(knsd): Url?
    CallbackData(String), //TODO(knsd) Validate size?
                          //    SwitchInlineQuery(String),
                          //    SwitchInlineQueryCurrentChat(String),
                          //    CallbackGame(CallbackGame),
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
struct InlineKeyboardButtonRaw<'a> {
    text: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<&'a str>, // TODO(knsd): Url?
    #[serde(skip_serializing_if = "Option::is_none")]
    callback_data: Option<&'a str>, //TODO(knsd) Validate size?
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_inline_query: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_inline_query_current_chat: Option<&'a str>,
    //    callback_game: Option<CallbackGame>,
}

/// Upon receiving a message with this object, Telegram clients will
/// display a reply interface to the user (act as if the user has
/// selected the bot‘s message and tapped ’Reply'). This can be
/// extremely useful if you want to create user-friendly step-by-step
/// interfaces without having to sacrifice privacy mod
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct ForceReply {
    force_reply: True,
    #[serde(skip_serializing_if = "Not::not")]
    selective: bool,
}

impl ForceReply {
    pub fn new() -> Self {
        Self {
            force_reply: True,
            selective: false,
        }
    }

    /// Use this method if you want to force reply from specific users only.
    /// Targets: 1) users that are @mentioned in the text of
    /// the Message object; 2) if the bot's message is a reply (has reply_to_message_id),
    /// sender of the original message.
    pub fn selective(&mut self) -> &mut Self {
        self.selective = true;
        self
    }
}
