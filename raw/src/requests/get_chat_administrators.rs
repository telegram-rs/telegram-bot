use types::*;
use requests::*;

/// Use this method to get a list of administrators in a chat.
/// If the chat is a group or a supergroup and no administrators were appointed,
/// only the creator will be returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct GetChatAdministrators<'c> {
    chat_id: ChatId<'c>
}

impl<'c> Request for GetChatAdministrators<'c> {
    type Response = Vec<ChatMember>;
    type RawResponse = Vec<ChatMember>;

    fn map(raw: Self::RawResponse) -> Self::Response {
        raw
    }

    fn name() -> &'static str {
        "getChatAdministrators"
    }
}

impl<'c> GetChatAdministrators<'c> {
    pub fn new<C>(chat: C) -> Self where C: Into<ChatId<'c>> {
        GetChatAdministrators {
            chat_id: chat.into()
        }
    }
}

pub trait CanGetChatAdministrators<'bc, 'c> {
    fn get_chat_administrators(&'bc self) -> GetChatAdministrators<'c>;
}

impl<'c, 'bc, C: 'bc> CanGetChatAdministrators<'bc, 'c> for C where &'bc C: Into<ChatId<'c>> {
    fn get_chat_administrators(&'bc self) -> GetChatAdministrators<'c> {
        GetChatAdministrators::new(self)
    }
}
