use requests::*;
use types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct GetUpdates {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub offset: Option<Integer>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub limit: Option<Integer>, // TODO(knsd): Values between 1—100 are accepted
    #[serde(skip_serializing_if = "Option::is_none")]
    pub timeout: Option<Integer>, // TODO(knsd): Should be positive
    pub allowed_updates: Vec<AllowedUpdate> // TODO(knsd) BitSet? HashSet? BTreeSet?
}

impl Request for GetUpdates {
    type Response = Vec<Update>;

    fn name(&self) -> &'static str {
        "getUpdates"
    }
}

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub enum AllowedUpdate {
    #[serde(rename="message")]
    Message,
    #[serde(rename="edited_message")]
    EditedMessage,
    #[serde(rename="channel_post")]
    ChannelPost,
    #[serde(rename="edited_channel_post")]
    EditedChannelPost,
//    #[serde(rename="inline_query")]
//    InlineQuery,
//    #[serde(rename="chosen_inline_query")]
//    ChosenInlineResult,
//    #[serde(rename="callback_query")]
//    CallbackQuery,
}
