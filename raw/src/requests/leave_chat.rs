use types::*;
use requests::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct LeaveChat<'c> {
    pub chat_id: ChatId<'c>
}

impl<'c> Request for LeaveChat<'c> {
    type Response = True;

    fn name(&self) -> &'static str {
        "leaveChat"
    }
}

impl<'c> LeaveChat<'c> {
    pub fn new<C>(chat: C) -> Self where C: Into<ChatId<'c>> {
        LeaveChat {
            chat_id: chat.into()
        }
    }
}

pub trait CanLeaveChat<'bc, 'c> {
    fn leave_chat(&'bc self) -> LeaveChat<'c>;
}

impl<'c, 'bc, C: 'bc> CanLeaveChat<'bc, 'c> for C where &'bc C: Into<ChatId<'c>> {
    fn leave_chat(&'bc self) -> LeaveChat<'c> {
        LeaveChat::new(self)
    }
}
