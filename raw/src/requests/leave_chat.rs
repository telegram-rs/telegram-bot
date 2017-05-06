use types::*;
use requests::*;

/// Use this method for your bot to leave a group, supergroup or channel.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct LeaveChat<'c> {
    chat_id: ChatRef<'c>
}

impl<'c> Request for LeaveChat<'c> {
    type Response = ();
    type RawResponse = True;

    fn map(_raw: Self::RawResponse) -> Self::Response {
        ()
    }

    fn name() -> &'static str {
        "leaveChat"
    }
}

impl<'c> LeaveChat<'c> {
    pub fn new<C>(chat: C) -> Self where C: ToChatRef<'c> {
        LeaveChat {
            chat_id: chat.to_chat_ref()
        }
    }
}

pub trait CanLeaveChat<'c> {
    fn leave_chat(&self) -> LeaveChat<'c>;
}

impl<'c, C> CanLeaveChat<'c> for C where C: ToChatRef<'c> {
    fn leave_chat(&self) -> LeaveChat<'c> {
        LeaveChat::new(self)
    }
}
