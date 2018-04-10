use types::*;
use requests::*;

/// Use this method to kick a user from a group or a supergroup.
/// In the case of supergroups, the user will not be able to return to the group on
/// their own using invite links, etc., unless unbanned first.
/// The bot must be an administrator in the group for this to work.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct RestrictChatMember {
    chat_id: ChatRef,
    user_id: UserId,
    until_date: Option<i32>,
    can_send_messages: Option<bool>,
    can_send_media_messages: Option<bool>,
    can_send_other_messages: Option<bool>,
    can_add_web_page_previews: Option<bool>,
}

impl Request for RestrictChatMember {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("restrictChatMember"), self)
    }
}

impl RestrictChatMember {
    pub fn new<C, U>(chat: C, user: U, until_date: Option<i32>, can_send_messages: Option<bool>, can_send_media_messages: Option<bool>, can_send_other_messages: Option<bool>, can_add_web_page_previews: Option<bool>) -> Self where C: ToChatRef, U: ToUserId {
        RestrictChatMember {
            chat_id: chat.to_chat_ref(),
            user_id: user.to_user_id(),
            until_date: until_date,
            can_send_messages: can_send_messages,
            can_send_media_messages: can_send_media_messages,
            can_send_other_messages: can_send_other_messages,
            can_add_web_page_previews: can_add_web_page_previews,
        }
    }
}

/// Restrict a user from a group or a supergroup.
pub trait CanRestrictChatMemberForChat {
    fn restrict<O>(&self, other: O, until_date: Option<i32>, can_send_messages: Option<bool>, can_send_media_messages: Option<bool>, can_send_other_messages: Option<bool>, can_add_web_page_previews: Option<bool>) -> RestrictChatMember where O: ToUserId;
}

impl<C> CanRestrictChatMemberForChat for C where C: ToChatRef {
    fn restrict<O>(&self, other: O, until_date: Option<i32>, can_send_messages: Option<bool>, can_send_media_messages: Option<bool>, can_send_other_messages: Option<bool>, can_add_web_page_previews: Option<bool>) -> RestrictChatMember where O: ToUserId {
        RestrictChatMember::new(self, other, until_date, can_send_messages, can_send_media_messages, can_send_other_messages, can_add_web_page_previews)
    }
}

/// Restrict a user from a group or a supergroup.
pub trait CanRestrictChatMemberForUser {
    fn restrict_from<O>(&self, other: O, until_date: Option<i32>, can_send_messages: Option<bool>, can_send_media_messages: Option<bool>, can_send_other_messages: Option<bool>, can_add_web_page_previews: Option<bool>) -> RestrictChatMember where O: ToChatRef;
}

impl<U> CanRestrictChatMemberForUser for U where U: ToUserId {
    fn restrict_from<O>(&self, other: O, until_date: Option<i32>, can_send_messages: Option<bool>, can_send_media_messages: Option<bool>, can_send_other_messages: Option<bool>, can_add_web_page_previews: Option<bool>) -> RestrictChatMember where O: ToChatRef {
        RestrictChatMember::new(other, self, until_date, can_send_messages, can_send_media_messages, can_send_other_messages, can_add_web_page_previews)
    }
}
