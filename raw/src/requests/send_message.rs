use std::ops::Not;
use std::borrow::Cow;

use types::*;
use requests::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct SendMessage<'c, 's> {
    pub chat_id: ChatId<'c>,
    pub text: Cow<'s, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_web_page_preview: bool,
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub reply_markup: Option<ReplyMarkup>,
}

impl<'c, 's> Request for SendMessage<'c, 's> {
    type Response = Message;

    fn name(&self) -> &'static str {
        "sendMessage"
    }
}

impl<'c, 's> SendMessage<'c, 's> {
    pub fn new<C, T>(chat: C, text: T) -> Self where C: Into<ChatId<'c>>, T: Into<Cow<'s, str>> {
        SendMessage {
            chat_id: chat.into(),
            text: text.into(),
            parse_mode: None,
            disable_web_page_preview: false,
            disable_notification: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn parse_mode(mut self, parse_mode: ParseMode) -> Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn disable_web_page_preview(mut self) -> Self { // TODO(knsd): Rename to disable_preview?
        self.disable_web_page_preview = true;
        self
    }

    pub fn disable_notification(mut self) -> Self {
        self.disable_notification = true;
        self
    }

    pub fn reply_to<R>(mut self, to: R) -> Self where R: Into<ReplyTo> {
        self.reply_to_message_id = Some(to.into().0);
        self
    }

    pub fn reply_markup<R>(mut self, reply_markup: R) -> Self where R: Into<ReplyMarkup> { // TODO(knsd): nice builder?
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

pub trait CanSendMessage {
    fn text<'c, 's, T>(&self, text: T) -> SendMessage<'c, 's> where T: Into<Cow<'s, str>>;
}

pub trait CanReplySendMessage {
    fn text_reply<'c, 's, T>(&self, text: T) -> SendMessage<'c, 's> where T: Into<Cow<'s, str>>;
}

impl CanSendMessage for Chat {
    fn text<'c, 's, T>(&self, text: T) -> SendMessage<'c, 's> where T: Into<Cow<'s, str>> {
        SendMessage::new(self, text)
    }
}

macro_rules! send_chat_type {
    ($chat: ident) => {
        impl CanSendMessage for $chat {
            fn text<'c, 's, T>(&self, text: T) -> SendMessage<'c, 's> where T: Into<Cow<'s, str>> {
                SendMessage::new(self, text)
            }
        }
    };
}

send_chat_type!(User);
send_chat_type!(Group);
send_chat_type!(Supergroup);
send_chat_type!(Channel);

impl CanSendMessage for ForwardFrom {
    fn text<'c, 's, T>(&self, text: T) -> SendMessage<'c, 's> where T: Into<Cow<'s, str>> {
        let id = match *self {
            ForwardFrom::User {ref user, ..} => user.id,
            ForwardFrom::Channel {ref channel, ..} => channel.id,
        };

        SendMessage::new(id, text)
    }
}

impl CanReplySendMessage for Message {
    fn text_reply<'c, 's, T>(&self, text: T) -> SendMessage<'c, 's> where T: Into<Cow<'s, str>> {
        self.chat.text(text).reply_to(self)
    }
}
