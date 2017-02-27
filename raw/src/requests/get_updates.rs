use requests::*;
use types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GetUpdates {
    pub offset: Option<Integer>,
    pub limit: Option<Integer>, // TODO(knsd): Values between 1—100 are accepted
    pub timeout: Option<Integer>, // TODO(knsd): Should be positive
    pub allowed_updates: Vec<AllowedUpdate> // TODO(knsd) BitSet? HashSet? BTreeSet?
}

impl Request for GetUpdates {
    type Response = Vec<Update>;
}

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub enum AllowedUpdate {
    Message,
    EditedMessage,
    ChannelPost,
    EditedChannelPost,
//    InlineQuery,
//    ChosenInlineResult,
//    CallbackQuery,
}
