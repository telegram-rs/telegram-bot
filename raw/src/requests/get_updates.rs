use types::primitive::*;

#[derive(Debug, Clone, PartialEq, PartialOrd)]
pub struct GetUpdates {
    pub offset: Option<Integer>,
    pub limit: Option<Integer>, // TODO(knsd): Values between 1—100 are accepted
    pub timeout: Option<Integer>, // TODO(knsd): Should be positive
    pub allowed_updates: Vec<AllowedUpdate> // TODO(knsd) BitSet? HashSet? BTreeSet?
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
