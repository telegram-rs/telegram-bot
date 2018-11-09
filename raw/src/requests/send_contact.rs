use std::borrow::Cow;
use std::ops::Not;

use requests::*;
use types::*;

/// Use this method to send phone contacts.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SendContact<'p, 'f, 'l> {
    chat_id: ChatRef,
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

impl<'p, 'f, 'l> Request for SendContact<'p, 'f, 'l> {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendContact"), self)
    }
}

impl<'p, 'f, 'l> SendContact<'p, 'f, 'l> {
    pub fn new<C, P, F>(chat: C, phone_number: P, first_name: F) -> Self
    where
        C: ToChatRef,
        P: Into<Cow<'p, str>>,
        F: Into<Cow<'f, str>>,
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

    pub fn last_name<F>(&mut self, last_name: F) -> &mut Self
    where
        F: Into<Cow<'l, str>>,
    {
        self.last_name = Some(last_name.into());
        self
    }

    pub fn disable_notification(&mut self) -> &mut Self {
        self.disable_notification = true;
        self
    }

    pub fn reply_to<R>(&mut self, to: R) -> &mut Self
    where
        R: ToMessageId,
    {
        self.reply_to_message_id = Some(to.to_message_id());
        self
    }

    pub fn reply_markup<R>(&mut self, reply_markup: R) -> &mut Self
    where
        R: Into<ReplyMarkup>,
    {
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

/// Send phone contact.
pub trait CanSendContact<'p, 'f, 'l> {
    fn contact<P, F>(&self, phone_number: P, first_name: F) -> SendContact<'p, 'f, 'l>
    where
        P: Into<Cow<'p, str>>,
        F: Into<Cow<'f, str>>;
}

impl<'p, 'f, 'l, C> CanSendContact<'p, 'f, 'l> for C
where
    C: ToChatRef,
{
    fn contact<P, F>(&self, phone_number: P, first_name: F) -> SendContact<'p, 'f, 'l>
    where
        P: Into<Cow<'p, str>>,
        F: Into<Cow<'f, str>>,
    {
        SendContact::new(self, phone_number, first_name)
    }
}

/// Reply with phone contact.
pub trait CanReplySendContact {
    fn contact_reply<'p, 'f, 'l, P: 'p, F: 'f>(
        &self,
        phone_number: P,
        first_name: F,
    ) -> SendContact<'p, 'f, 'l>
    where
        P: Into<Cow<'p, str>>,
        F: Into<Cow<'f, str>>;
}

impl<M> CanReplySendContact for M
where
    M: ToMessageId + ToSourceChat,
{
    fn contact_reply<'p, 'f, 'l, P: 'p, F: 'f>(
        &self,
        phone_number: P,
        first_name: F,
    ) -> SendContact<'p, 'f, 'l>
    where
        P: Into<Cow<'p, str>>,
        F: Into<Cow<'f, str>>,
    {
        let mut rq = self.to_source_chat().contact(phone_number, first_name);
        rq.reply_to(self.to_message_id());
        rq
    }
}

impl<'b> ToRequest<'b> for Contact {
    type Request = SendContact<'b, 'b, 'b>;

    fn to_request<C>(&'b self, chat: C) -> Self::Request
    where
        C: ToChatRef,
    {
        let mut rq = chat.contact(self.phone_number.as_str(), self.first_name.as_str());
        if let Some(ref last_name) = self.last_name {
            rq.last_name(last_name.as_str());
        }
        rq
    }
}

impl<'b> ToReplyRequest<'b> for Contact {
    type Request = SendContact<'b, 'b, 'b>;

    fn to_reply_request<M>(&'b self, message: M) -> Self::Request
    where
        M: ToMessageId + ToSourceChat,
    {
        let mut rq = message.contact_reply(self.phone_number.as_str(), self.first_name.as_str());
        if let Some(ref last_name) = self.last_name {
            rq.last_name(last_name.as_str());
        }
        rq
    }
}
