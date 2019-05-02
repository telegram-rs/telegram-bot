use crate::types::*;
use std::ops::Not;

#[derive(Serialize, Debug)]
#[serde(tag = "type")]
pub enum InlineQueryResult {
    #[serde(rename = "article")]
    InlineQueryResultArticle(InlineQueryResultArticle),
    Unknown, // TODO: Rest of the fields
}

#[derive(Serialize, Debug)]
pub struct InlineQueryResultArticle {
    id: String,
    title: String,
    input_message_content: InputMessageContent,
    // TODO: Rest of the fields
}

impl InlineQueryResultArticle {
    pub fn new(
        id: String,
        title: String,
        input_message_content: InputMessageContent,
    ) -> InlineQueryResultArticle {
        InlineQueryResultArticle {
            id,
            title,
            input_message_content,
        }
    }
}

impl From<InlineQueryResultArticle> for InlineQueryResult {
    fn from(article: InlineQueryResultArticle) -> Self {
        InlineQueryResult::InlineQueryResultArticle(article)
    }
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum InputMessageContent {
    InputTextMessageContent(InputTextMessageContent),
    InputLocationMessageContent(InputLocationMessageContent),
    InputVenueMessageContent(InputVenueMessageContent),
    InputContactMessageContent(InputContactMessageContent),
}

#[derive(Serialize, Debug)]
pub struct InputTextMessageContent {
    pub message_text: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub parse_mode: Option<ParseMode>,
    #[serde(skip_serializing_if = "Not::not")]
    pub disable_web_page_preview: bool,
}

impl From<InputTextMessageContent> for InputMessageContent {
    fn from(value: InputTextMessageContent) -> Self {
        InputMessageContent::InputTextMessageContent(value)
    }
}

// TODO
#[derive(Serialize, Debug)]
pub struct InputContactMessageContent {}

// TODO
#[derive(Serialize, Debug)]
pub struct InputVenueMessageContent {}

// TODO
#[derive(Serialize, Debug)]
pub struct InputLocationMessageContent {}
