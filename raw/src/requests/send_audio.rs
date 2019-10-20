use std::borrow::Cow;

use crate::requests::*;
use crate::types::*;

/// Use this method to send an audio
#[derive(Debug, Clone, PartialEq, PartialOrd)]
#[must_use = "requests do nothing unless sent"]
pub struct SendAudio<'c, 'p, 't> {
    chat_id: ChatRef,
    audio: InputFile,
    caption: Option<Cow<'c, str>>,
    parse_mode: Option<ParseMode>,
    duration: Option<Integer>,
    performer: Option<Cow<'p, str>>,
    title: Option<Cow<'t, str>>,
    thumb: Option<InputFile>,
    reply_to_message_id: Option<MessageId>,
    disable_notification: bool,
    reply_markup: Option<ReplyMarkup>,
}

impl<'c, 'p, 't> ToMultipart for SendAudio<'c, 'p, 't> {
    fn to_multipart(&self) -> Result<Multipart, Error> {
        multipart_map! {
            self,
            (chat_id (text));
            (audio (raw));
            (caption (text), optional);
            (parse_mode (text), optional);
            (duration (text), optional);
            (performer (text), optional);
            (title (text), optional);
            (thumb (raw), optional);
            (reply_to_message_id (text), optional);
            (disable_notification (text), when_true);
            (reply_markup (json), optional);
        }
    }
}

impl<'c, 'p, 't> Request for SendAudio<'c, 'p, 't> {
    type Type = MultipartRequestType<Self>;
    type Response = JsonIdResponse<Message>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("sendAudio"), self)
    }
}

impl<'c, 'p, 't> SendAudio<'c, 'p, 't> {
    pub fn new<C, V>(chat: C, audio: V) -> Self
    where
        C: ToChatRef,
        V: Into<InputFile>,
    {
        Self {
            chat_id: chat.to_chat_ref(),
            audio: audio.into(),
            caption: None,
            parse_mode: None,
            duration: None,
            performer: None,
            title: None,
            thumb: None,
            reply_to_message_id: None,
            reply_markup: None,
            disable_notification: false,
        }
    }

    pub fn thumb<V>(&mut self, thumb: V) -> &mut Self
    where
        V: Into<InputFileUpload>,
    {
        self.thumb = Some(thumb.into().into());
        self
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

    pub fn duration(&mut self, duration: Integer) -> &mut Self {
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

    pub fn disable_notification(&mut self) -> &mut Self {
        self.disable_notification = true;
        self
    }
}

/// Can reply with an audio
pub trait CanReplySendAudio {
    fn audio_reply<'c, 'p, 't, T>(&self, audio: T) -> SendAudio<'c, 'p, 't>
    where
        T: Into<InputFile>;
}

impl<M> CanReplySendAudio for M
where
    M: ToMessageId + ToSourceChat,
{
    fn audio_reply<'c, 'p, 't, T>(&self, audio: T) -> SendAudio<'c, 'p, 't>
    where
        T: Into<InputFile>,
    {
        let mut req = SendAudio::new(self.to_source_chat(), audio);
        req.reply_to(self);
        req
    }
}

/// Send an audio
pub trait CanSendAudio {
    fn audio<'c, 'p, 't, T>(&self, audio: T) -> SendAudio<'c, 'p, 't>
    where
        T: Into<InputFile>;
}

impl<M> CanSendAudio for M
where
    M: ToChatRef,
{
    fn audio<'c, 'p, 't, T>(&self, audio: T) -> SendAudio<'c, 'p, 't>
    where
        T: Into<InputFile>,
    {
        SendAudio::new(self.to_chat_ref(), audio)
    }
}
