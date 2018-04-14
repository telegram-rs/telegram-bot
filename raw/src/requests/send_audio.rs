use std::borrow::Cow;

use types::*;
use requests::*;

/// Use this method to send an audio
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[must_use = "requests do nothing unless sent"]
pub struct SendAudio<'s, 'c, 'p, 't> {
    chat_id: ChatRef,
    audio: Cow<'s, str>,
    caption: Option<Cow<'c, str>>,
    parse_mode: Option<ParseMode>,
    duration: Option<i64>,
    performer: Option<Cow<'p, str>>,
    title: Option<Cow<'t, str>>,
    reply_to_message_id: Option<MessageId>,
    disable_notification: bool,
    reply_markup: Option<ReplyMarkup>,
}

impl<'s, 'c, 'p, 't> ToMultipart for SendAudio<'s, 'c, 'p, 't> {
    fn to_multipart(&self) -> Multipart {
        multipart_map! {
            self,
            (chat_id (text));
            (audio (file));
            (caption (text), optional);
            (parse_mode (text), optional);
            (duration (text), optional);
            (performer (text), optional);
            (title (text), optional);
            (reply_to_message_id (text), optional);
            (disable_notification (text), when_true);
            (reply_markup (json), optional);
        }
    }
}

impl<'s, 'c, 'p, 't> Request for SendAudio<'s, 'c, 'p, 't> {
    type Type = MultipartRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendAudio"), self)
    }
}

impl<'s, 'c, 'p, 't> SendAudio<'s, 'c, 'p, 't> {
    pub fn with_url<C, T>(chat: C, url: T) -> Self
    where
        C: ToChatRef,
        T: Into<Cow<'s, str>>,
    {
        Self {
            chat_id: chat.to_chat_ref(),
            audio: url.into(),
            caption: None,
            parse_mode: None,
            duration: None,
            performer: None,
            title: None,
            reply_to_message_id: None,
            reply_markup: None,
            disable_notification: false,
        }
    }

    pub fn caption<T>(&mut self, caption: T) -> &mut Self
    where
        T: Into<Cow<'c, str>>,
    {
        self.caption = Some(caption.into());
        self
    }

    pub fn parse_mode(&mut self, parse_mode: ParseMode) -> &mut Self {
        self.parse_mode = Some(parse_mode);
        self
    }

    pub fn duration(&mut self, duration: i64) -> &mut Self {
        self.duration = Some(duration);
        self
    }

    pub fn performer<T>(&mut self, performer: T) -> &mut Self
    where
        T: Into<Cow<'p, str>>,
    {
        self.performer = Some(performer.into());
        self
    }

    pub fn title<T>(&mut self, title: T) -> &mut Self
    where
        T: Into<Cow<'t, str>>,
    {
        self.title = Some(title.into());
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

/// Can reply with an audio
pub trait CanReplySendAudio {
    fn audio_url_reply<'s, 'c, 'p, 't, T>(&self, url: T) -> SendAudio<'s, 'c, 'p, 't>
    where
        T: Into<Cow<'s, str>>;
}

impl<M> CanReplySendAudio for M
where
    M: ToMessageId + ToSourceChat,
{
    fn audio_url_reply<'s, 'c, 'p, 't, T>(&self, url: T) -> SendAudio<'s, 'c, 'p, 't>
    where
        T: Into<Cow<'s, str>>,
    {
        let mut req = SendAudio::with_url(self.to_source_chat(), url);
        req.reply_to(self);
        req
    }
}

/// Send an audio
pub trait CanSendAudio {
    fn audio_url<'s, 'c, 'p, 't, T>(&self, url: T) -> SendAudio<'s, 'c, 'p, 't>
    where
        T: Into<Cow<'s, str>>;
}

impl<M> CanSendAudio for M
where
    M: ToChatRef,
{
    fn audio_url<'s, 'c, 'p, 't, T>(&self, url: T) -> SendAudio<'s, 'c, 'p, 't>
    where
        T: Into<Cow<'s, str>>,
    {
        SendAudio::with_url(self.to_chat_ref(), url)
    }
}
