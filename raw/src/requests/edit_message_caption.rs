use std::borrow::Cow;

use types::*;
use requests::*;

/// Use this method to edit captions of messages sent by the bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct EditMessageCaption<'c, 's> {
    chat_id: ChatRef<'c>,
    message_id: MessageId,
    caption: Cow<'s, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'c, 's> Request for EditMessageCaption<'c, 's> {
    type Response = Message;
    type RawResponse = Message;

    fn map(raw: Self::RawResponse) -> Self::Response {
        raw
    }

    fn name() -> &'static str {
        "editMessageCaption"
    }
}

impl<'c, 's> EditMessageCaption<'c, 's> {
    pub fn new<C, M, T>(chat: C, message_id: M, caption: T) -> Self
        where C: ToChatRef<'c>, M: ToMessageId, T: Into<Cow<'s, str>> {

        EditMessageCaption {
            chat_id: chat.to_chat_ref(),
            message_id: message_id.to_message_id(),
            caption: caption.into(),
            reply_markup: None,
        }
    }

    pub fn reply_markup<R>(mut self, reply_markup: R) -> Self where R: Into<ReplyMarkup> { // TODO(knsd): nice builder?
        self.reply_markup = Some(reply_markup.into());
        self
    }
}

pub trait CanEditMessageCaption {
    fn edit_caption<'c, 's, T>(&self, caption: T) -> EditMessageCaption<'c, 's> where T: Into<Cow<'s, str>>;
}

impl<M> CanEditMessageCaption for M where M: ToMessageId + ToSourceChat {
    fn edit_caption<'c, 's, T>(&self, caption: T) -> EditMessageCaption<'c, 's> where T: Into<Cow<'s, str>> {
        EditMessageCaption::new(self.to_source_chat(), self.to_message_id(), caption)
    }
}
