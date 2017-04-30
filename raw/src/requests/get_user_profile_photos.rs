use types::*;
use requests::*;

/// Use this method to get a list of profile pictures for a user.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
pub struct GetUserProfilePhotos {
    user_id: UserId,
    offset: Option<Integer>,
    limit: Option<Integer>,
}

impl Request for GetUserProfilePhotos {
    type Response = UserProfilePhotos;
    type RawResponse = UserProfilePhotos;

    fn map(raw: Self::RawResponse) -> Self::Response {
        raw
    }

    fn name() -> &'static str {
        "getUserProfilePhotos"
    }
}

impl GetUserProfilePhotos {
    pub fn new<U>(user: U) -> Self where U: Into<UserId> {
        GetUserProfilePhotos {
            user_id: user.into(),
            offset: None,
            limit: None,
        }
    }

    pub fn offset(mut self, offset: Integer) -> Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(mut self, limit: Integer) -> Self {
        self.limit = Some(limit);
        self
    }
}

pub trait CanGetUserProfilePhotos<'b> {
    fn get_user_profile_photos(&'b self) -> GetUserProfilePhotos;
}

impl<'b, U: 'b> CanGetUserProfilePhotos<'b> for U where &'b U: Into<UserId> {
    fn get_user_profile_photos(&'b self) -> GetUserProfilePhotos {
        GetUserProfilePhotos::new(self)
    }
}
