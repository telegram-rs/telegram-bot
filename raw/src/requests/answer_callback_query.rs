use std::borrow::Cow;
use std::ops::Not;

use requests::*;
use types::*;

/// Use this method to send answers to callback queries sent from inline keyboards.
/// The answer will be displayed to the user as a notification at the top of
/// the chat screen or as an alert.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct AnswerCallbackQuery<'t> {
    callback_query_id: CallbackQueryId,
    #[serde(skip_serializing_if = "Option::is_none")]
    text: Option<Cow<'t, str>>,
    #[serde(skip_serializing_if = "Not::not")]
    show_alert: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    url: Option<Cow<'t, str>>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<i64>,
}

impl<'i, 't> Request for AnswerCallbackQuery<'t> {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("answerCallbackQuery"), self)
    }
}

impl<'t> AnswerCallbackQuery<'t> {
    fn new<Q, T>(query: Q, text: T) -> Self
    where
        Q: ToCallbackQueryId,
        T: Into<Cow<'t, str>>,
    {
        Self {
            callback_query_id: query.to_callback_query_id(),
            text: Some(text.into()),
            show_alert: false,
            url: None,
            cache_time: None,
        }
    }

    fn acknowledge<Q>(query: Q) -> Self
    where
        Q: ToCallbackQueryId,
    {
        Self {
            callback_query_id: query.to_callback_query_id(),
            text: None,
            show_alert: false,
            url: None,
            cache_time: None,
        }
    }

    /// An alert will be shown by the client instead of a notification
    /// at the top of the chat screen.
    pub fn show_alert(&mut self) -> &mut Self {
        self.show_alert = true;
        self
    }

    /// URL that will be opened by the user's client. If you have created a
    /// Game and accepted the conditions via @Botfather, specify the URL
    /// that opens your game – note that this will only work if the query
    /// comes from a callback_game button.
    ///
    /// Otherwise, you may use links like t.me/your_bot?start=XXXX that open your bot with a parameter.
    pub fn url<T>(&mut self, url: T) -> &mut Self
    where
        T: Into<Cow<'t, str>>,
    {
        self.url = Some(url.into());
        self
    }

    /// The maximum amount of time in seconds that the result of the callback query
    /// may be cached client-side. Telegram apps will support caching starting in
    /// version 3.14. Defaults to 0.
    pub fn cache_time(&mut self, time: i64) -> &mut Self {
        self.cache_time = Some(time);
        self
    }
}

/// Send answers to callback queries sent from inline keyboards.
pub trait CanAnswerCallbackQuery {
    fn answer<'t, T>(&self, text: T) -> AnswerCallbackQuery<'t>
    where
        T: Into<Cow<'t, str>>;
    fn acknowledge<'t>(&self) -> AnswerCallbackQuery<'t>;
}

impl<Q> CanAnswerCallbackQuery for Q
where
    Q: ToCallbackQueryId,
{
    fn answer<'t, T>(&self, text: T) -> AnswerCallbackQuery<'t>
    where
        T: Into<Cow<'t, str>>,
    {
        AnswerCallbackQuery::new(&self, text)
    }
    fn acknowledge<'t>(&self) -> AnswerCallbackQuery<'t> {
        AnswerCallbackQuery::acknowledge(&self)
    }
}
