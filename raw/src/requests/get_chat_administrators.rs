use types::*;
use requests::*;

/// Use this method to get a list of administrators in a chat.
/// If the chat is a group or a supergroup and no administrators were appointed,
/// only the creator will be returned.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct GetChatAdministrators<'c> {
    chat_id: ChatRef<'c>
}

impl<'c> Request for GetChatAdministrators<'c> {
    type Response = IdResponse<Vec<ChatMember>>;

    fn name() -> &'static str {
        "getChatAdministrators"
    }
}

impl<'c> GetChatAdministrators<'c> {
    pub fn new<C>(chat: C) -> Self where C: ToChatRef<'c> {
        GetChatAdministrators {
            chat_id: chat.to_chat_ref()
        }
    }
}

pub trait CanGetChatAdministrators<'c> {
    fn get_administrators(&self) -> GetChatAdministrators<'c>;
}

impl<'c, C> CanGetChatAdministrators<'c> for C where C: ToChatRef<'c> {
    fn get_administrators(&self) -> GetChatAdministrators<'c> {
        GetChatAdministrators::new(self)
    }
}
