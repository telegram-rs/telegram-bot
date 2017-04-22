use types::*;
use requests::*;

/// Use this method to edit only the reply markup of messages sent by the bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct EditMessageReplyMarkup<'c> {
    chat_id: ChatId<'c>,
    message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'c> Request for EditMessageReplyMarkup<'c> {
    type Response = Message;
    type RawResponse = Message;

    fn map(raw: Self::RawResponse) -> Self::Response {
        raw
    }

    fn name() -> &'static str {
        "editMessageReplyMarkup"
    }
}

impl<'c> EditMessageReplyMarkup<'c> {
    pub fn new<C, M, R>(chat: C, message_id: M, reply_markup: Option<R>) -> Self
        where C: Into<ChatId<'c>>, M: Into<MessageId>, R: Into<ReplyMarkup> {

        EditMessageReplyMarkup {
            chat_id: chat.into(),
            message_id: message_id.into(),
            reply_markup: reply_markup.map(|r| r.into()),
        }
    }

}

pub trait CanEditMessageReplyMarkup {
    fn edit_reply_markup<'c, R>(&self, reply_markup: Option<R>) -> EditMessageReplyMarkup<'c> where R: Into<ReplyMarkup>;
}

impl CanEditMessageReplyMarkup for Message {
    fn edit_reply_markup<'c, R>(&self, reply_markup: Option<R>) -> EditMessageReplyMarkup<'c> where R: Into<ReplyMarkup> {
        EditMessageReplyMarkup::new(&self.chat, self, reply_markup)
    }
}
