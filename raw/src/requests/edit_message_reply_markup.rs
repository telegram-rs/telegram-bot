use types::*;
use requests::*;

/// Use this method to edit only the reply markup of messages sent by the bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct EditMessageReplyMarkup<'c> {
    chat_id: ChatRef<'c>,
    message_id: MessageId,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,
}

impl<'c> Request for EditMessageReplyMarkup<'c> {
    type Response = IdResponse<Message>;

    fn name() -> &'static str {
        "editMessageReplyMarkup"
    }
}

impl<'c> EditMessageReplyMarkup<'c> {
    pub fn new<C, M, R>(chat: C, message_id: M, reply_markup: Option<R>) -> Self
        where C: ToChatRef<'c>, M: ToMessageId, R: Into<ReplyMarkup> {

        EditMessageReplyMarkup {
            chat_id: chat.to_chat_ref(),
            message_id: message_id.to_message_id(),
            reply_markup: reply_markup.map(|r| r.into()),
        }
    }
}

pub trait CanEditMessageReplyMarkup {
    fn edit_reply_markup<'c, R>(&self, reply_markup: Option<R>) -> EditMessageReplyMarkup<'c> where R: Into<ReplyMarkup>;
}

impl<M> CanEditMessageReplyMarkup for M where M: ToMessageId + ToSourceChat {
    fn edit_reply_markup<'c, R>(&self, reply_markup: Option<R>) -> EditMessageReplyMarkup<'c> where R: Into<ReplyMarkup> {
        EditMessageReplyMarkup::new(self.to_source_chat(), self.to_message_id(), reply_markup)
    }
}
