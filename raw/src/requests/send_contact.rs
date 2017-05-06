use std::ops::Not;
use std::borrow::Cow;

use types::*;
use requests::*;

/// Use this method to send phone contacts.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct SendContact<'c, 'p, 'f, 'l> {
    chat_id: ChatRef<'c>,
    phone_number: Cow<'p, str>,
    first_name: Cow<'f, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<Cow<'l, str>>,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'c, 'p, 'f, 'l> Request for SendContact<'c, 'p, 'f, 'l> {
    type Response = Message;
    type RawResponse = Message;

    fn map(raw: Self::RawResponse) -> Self::Response {
        raw
    }

    fn name() -> &'static str {
        "sendContact"
    }
}

impl<'c, 'p, 'f, 'l> SendContact<'c, 'p, 'f, 'l> {
    pub fn new<C, P, F>(chat: C, phone_number: P, first_name: F) -> Self
        where C: ToChatRef<'c>,
              P: Into<Cow<'p, str>>,
              F: Into<Cow<'f, str>>
    {
        SendContact {
            chat_id: chat.to_chat_ref(),
            phone_number: phone_number.into(),
            first_name: first_name.into(),
            last_name: None,
            disable_notification: false,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn last_name<F>(mut self, last_name: F) -> Self
        where F: Into<Cow<'l, str>>
    {
        self.last_name = Some(last_name.into());
        self
    }

    pub fn disable_notification(mut self) -> Self {
        self.disable_notification = true;
        self
    }

    pub fn reply_to<R>(mut self, to: R) -> Self
        where R: ToMessageId
    {
        self.reply_to_message_id = Some(to.to_message_id());
        self
    }

    pub fn reply_markup<R>(mut self, reply_markup: R) -> Self
        where R: Into<ReplyMarkup>
    {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

pub trait CanSendContact<'c, 'p, 'f, 'l> {
    fn contact<P, F>(&self, phone_number: P, first_name: F) -> SendContact<'c, 'p, 'f, 'l>
        where P: Into<Cow<'p, str>>,
              F: Into<Cow<'f, str>>;
}

impl<'c, 'p, 'f, 'l, C> CanSendContact<'c, 'p, 'f, 'l> for C
    where C: ToChatRef<'c>
{
    fn contact<P, F>(&self, phone_number: P, first_name: F) -> SendContact<'c, 'p, 'f, 'l>
        where P: Into<Cow<'p, str>>,
              F: Into<Cow<'f, str>>
    {
        SendContact::new(self, phone_number, first_name)
    }
}

pub trait CanReplySendContact {
    fn contact_reply<'c, 'p, 'f, 'l, P: 'p, F: 'f>(&self,
                                                   phone_number: P,
                                                   first_name: F)
                                                   -> SendContact<'c, 'p, 'f, 'l>
        where P: Into<Cow<'p, str>>,
              F: Into<Cow<'f, str>>;
}

impl CanReplySendContact for Message {
    fn contact_reply<'c, 'p, 'f, 'l, P: 'p, F: 'f>(&self,
                                                   phone_number: P,
                                                   first_name: F)
                                                   -> SendContact<'c, 'p, 'f, 'l>
        where P: Into<Cow<'p, str>>,
              F: Into<Cow<'f, str>>
    {
        self.chat.contact(phone_number, first_name).reply_to(self)
    }
}
