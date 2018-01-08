use std::borrow::Cow;
use std::ops::Not;

use types::*;
use requests::*;

/// Use this method to send answers to callback queries sent from inline keyboards.
/// The answer will be displayed to the user as a notification at the top of
/// the chat screen or as an alert.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct AnswerCallbackQuery<'t> {
    callback_query_id: CallbackQueryId,
    text: Cow<'t, str>,
    #[serde(skip_serializing_if = "Not::not")]
    show_alert: bool,
}

impl<'i, 't> Request for AnswerCallbackQuery<'t> {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("answerCallbackQuery"), self)
    }
}

impl<'t> AnswerCallbackQuery<'t> {
    fn new<Q, T>(query: Q, text: T) -> Self where Q: ToCallbackQueryId, T: Into<Cow<'t, str>> {
        Self {
            callback_query_id: query.to_callback_query_id(),
            text: text.into(),
            show_alert: false,
        }
    }

    /// An alert will be shown by the client instead of a notification
    /// at the top of the chat screen.
    pub fn show_alert(&mut self) -> &mut Self {
        self.show_alert = true;
        self
    }
}

/// Send answers to callback queries sent from inline keyboards.
pub trait CanAnswerCallbackQuery {
    fn answer<'t, T>(&self, text: T) -> AnswerCallbackQuery<'t> where T: Into<Cow<'t, str>>;
}

impl<Q> CanAnswerCallbackQuery for Q where Q: ToCallbackQueryId {
    fn answer<'t, T>(&self, text: T) -> AnswerCallbackQuery<'t> where T: Into<Cow<'t, str>> {
        AnswerCallbackQuery::new(&self, text)
    }
}
