use std::ops::Not;

use crate::requests::*;
use crate::types::*;

#[derive(Serialize, Debug)]
pub struct AnswerInlineQuery {
    inline_query_id: InlineQueryId,
    results: Vec<InlineQueryResult>,
    #[serde(skip_serializing_if = "Option::is_none")]
    cache_time: Option<Integer>,
    #[serde(skip_serializing_if = "Not::not")]
    is_personal: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    next_offset: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_pm_text: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    switch_pm_parameter: Option<String>,
}

impl Request for AnswerInlineQuery {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("answerInlineQuery"), self)
    }
}

pub trait CanAnswerInlineQuery {
    fn answer(self, results: Vec<InlineQueryResult>) -> AnswerInlineQuery;
}

impl<T> CanAnswerInlineQuery for T
where
    T: Into<InlineQueryId>,
{
    fn answer(self, results: Vec<InlineQueryResult>) -> AnswerInlineQuery {
        AnswerInlineQuery::new(self.into(), results)
    }
}

impl AnswerInlineQuery {
    pub fn new(
        inline_query_id: InlineQueryId,
        results: Vec<InlineQueryResult>,
    ) -> AnswerInlineQuery {
        AnswerInlineQuery {
            inline_query_id,
            results,
            cache_time: None,
            is_personal: false,
            next_offset: None,
            switch_pm_text: None,
            switch_pm_parameter: None,
        }
    }

    pub fn add_inline_result<T: Into<InlineQueryResult>>(&mut self, result: T) {
        self.results.push(result.into());
    }

    pub fn cache_time(&mut self, cache_time: Integer) -> &mut Self {
        self.cache_time = Some(cache_time);
        self
    }

    pub fn is_personal(&mut self) -> &mut Self {
        self.is_personal = true;
        self
    }

    pub fn next_offset(&mut self, next_offset: String) -> &mut Self {
        self.next_offset = Some(next_offset);
        self
    }

    pub fn switch_pm_text(&mut self, switch_pm_text: String) -> &mut Self {
        self.switch_pm_text = Some(switch_pm_text);
        self
    }

    pub fn switch_pm_parameter(&mut self, switch_pm_parameter: String) -> &mut Self {
        self.switch_pm_parameter = Some(switch_pm_parameter);
        self
    }
}
