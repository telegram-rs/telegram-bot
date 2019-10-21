use crate::types::*;

/// This object represents the result of an inline query that was chosen by a
/// user and sent to their chat partner. It is only sent if inline feedback is
/// enabled for the bot.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ChosenInlineResult {
    /// Unique identifier for the result that was chosen
    pub result_id: String,
    /// The user that chose the result
    pub from: User,
    /// Sender location, only for bots that require user location
    pub location: Option<Location>,
    /// Identifier of the sent inline message. Available only if there is an
    /// inline keyboard attached to the message. Will be also received in
    /// callback queries and can be used to edit the message
    pub inline_message_id: Option<String>,
    /// The query that was used to obtain the result
    pub query: String,
}
