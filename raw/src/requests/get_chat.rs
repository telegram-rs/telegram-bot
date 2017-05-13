use types::*;
use requests::*;

/// Use this method to get up to date information about the chat.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct GetChat<'c> {
    chat_id: ChatRef<'c>
}

impl<'c> Request for GetChat<'c> {
    type Response = IdResponse<Chat>;

    fn name() -> &'static str {
        "getChat"
    }
}

impl<'c> GetChat<'c> {
    pub fn new<C>(chat: C) -> Self where C: ToChatRef<'c> {
        GetChat {
            chat_id: chat.to_chat_ref()
        }
    }
}

pub trait CanGetChat<'c> {
    fn get_chat(&self) -> GetChat<'c>;
}

impl<'c, C> CanGetChat<'c> for C where C: ToChatRef<'c> {
    fn get_chat(&self) -> GetChat<'c> {
        GetChat::new(self)
    }
}
