use std::ops::Not;
use std::borrow::Cow;

use types::*;
use requests::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct SendMessage<'c, 's> {
    chat_id: ChatId<'c>,
    text: Cow<'s, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Not::not")]
    disable_web_page_preview: bool,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'c, 's> Request for SendMessage<'c, 's> {
    type Response = Message;
    type RawResponse = Message;

    fn map(raw: Self::RawResponse) -> Self::Response {
        raw
    }

    fn name() -> &'static str {
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

    pub fn reply_to<R>(mut self, to: R) -> Self where R: Into<MessageId> {
        self.reply_to_message_id = Some(to.into().0);
        self
    }

    pub fn reply_markup<R>(mut self, reply_markup: R) -> Self where R: Into<ReplyMarkup> { // TODO(knsd): nice builder?
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

pub trait CanSendMessage<'bc, 'c> {
    fn text<'s, T>(&'bc self, text: T) -> SendMessage<'c, 's> where T: Into<Cow<'s, str>>;
}

impl<'c, 'bc, C: 'bc> CanSendMessage<'bc, 'c> for C where &'bc C: Into<ChatId<'c>> {
    fn text<'s, T>(&'bc self, text: T) -> SendMessage<'c, 's> where T: Into<Cow<'s, str>> {
        SendMessage::new(self.into(), text)
    }
}

pub trait CanReplySendMessage {
    fn text_reply<'c, 's, T>(&self, text: T) -> SendMessage<'c, 's> where T: Into<Cow<'s, str>>;
}

impl CanReplySendMessage for Message {
    fn text_reply<'c, 's, T>(&self, text: T) -> SendMessage<'c, 's> where T: Into<Cow<'s, str>> {
        self.chat.text(text).reply_to(self)
    }
}
