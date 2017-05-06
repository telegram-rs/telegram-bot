use std::ops::Not;
use std::borrow::Cow;

use types::*;
use requests::*;

/// Use this method to send information about a venue.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct SendVenue<'c, 't, 'a, 'f> {
    chat_id: ChatRef<'c>,
    latitude: Float,
    longitude: Float,
    title: Cow<'t, str>,
    address: Cow<'a, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    foursquare_id: Option<Cow<'f, str>>,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<MessageId>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'c, 't, 'a, 'f> Request for SendVenue<'c, 't, 'a, 'f> {
    type Response = Message;
    type RawResponse = Message;

    fn map(raw: Self::RawResponse) -> Self::Response {
        raw
    }

    fn name() -> &'static str {
        "sendVenue"
    }
}

impl<'c, 't, 'a, 'f> SendVenue<'c, 't, 'a, 'f> {
    pub fn new<C, T, A>(chat: C, latitude: Float, longitude: Float, title: T, address: A) -> Self
        where C: ToChatRef<'c>,
              T: Into<Cow<'t, str>>,
              A: Into<Cow<'a, str>>
    {
        SendVenue {
            chat_id: chat.to_chat_ref(),
            latitude: latitude,
            longitude: longitude,
            title: title.into(),
            address: address.into(),
            disable_notification: false,
            foursquare_id: None,
            reply_to_message_id: None,
            reply_markup: None,
        }
    }

    pub fn disable_notification(mut self) -> Self {
        self.disable_notification = true;
        self
    }

    pub fn foursquare_id<F>(mut self, id: F) -> Self
        where F: Into<Cow<'f, str>>
    {
        self.foursquare_id = Some(id.into());
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

pub trait CanSendVenue<'c, 't, 'a, 'f> {
    fn venue<T, A>(&self,
                   latitude: Float,
                   longitude: Float,
                   title: T,
                   address: A)
                   -> SendVenue<'c, 't, 'a, 'f>
        where T: Into<Cow<'t, str>>,
              A: Into<Cow<'a, str>>;
}

impl<'c, 't, 'a, 'f, C> CanSendVenue<'c, 't, 'a, 'f> for C
    where C: ToChatRef<'c>
{
    fn venue<T, A>(&self,
                   latitude: Float,
                   longitude: Float,
                   title: T,
                   address: A)
                   -> SendVenue<'c, 't, 'a, 'f>
        where T: Into<Cow<'t, str>>,
              A: Into<Cow<'a, str>>
    {
        SendVenue::new(self, latitude, longitude, title, address)
    }
}

pub trait CanReplySendVenue {
    fn venue_reply<'c, 't, 'a, 'f, T, A>(&self,
                                         latitude: Float,
                                         longitude: Float,
                                         title: T,
                                         address: A)
                                         -> SendVenue<'c, 't, 'a, 'f>
        where T: Into<Cow<'t, str>>,
              A: Into<Cow<'a, str>>;
}

impl CanReplySendVenue for Message {
    fn venue_reply<'c, 't, 'a, 'f, T, A>(&self,
                                         latitude: Float,
                                         longitude: Float,
                                         title: T,
                                         address: A)
                                         -> SendVenue<'c, 't, 'a, 'f>
        where T: Into<Cow<'t, str>>,
              A: Into<Cow<'a, str>>
    {
        (&self.chat).venue(latitude, longitude, title, address).reply_to(self)
    }
}
