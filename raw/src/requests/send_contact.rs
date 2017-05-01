use std::ops::Not;
use std::borrow::Cow;

use types::*;
use requests::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct SendContact<'c, 'p, 'f, 'l> {
    chat_id: ChatId<'c>,
    phone_number: Cow<'p, str>,
    first_name: Cow<'f, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    last_name: Option<Cow<'l, str>>,
    #[serde(skip_serializing_if = "Not::not")]
    disable_notification: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_to_message_id: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    reply_markup: Option<ReplyMarkup>,    
}