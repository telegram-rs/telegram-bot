use requests::*;
use types::*;

/// Use this method to get a list of profile pictures for a user.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetUserProfilePhotos {
    user_id: UserId,
    offset: Option<Integer>,
    limit: Option<Integer>,
}

impl Request for GetUserProfilePhotos {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<UserProfilePhotos>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getUserProfilePhotos"), self)
    }
}

impl GetUserProfilePhotos {
    pub fn new<U>(user: U) -> Self
    where
        U: ToUserId,
    {
        GetUserProfilePhotos {
            user_id: user.to_user_id(),
            offset: None,
            limit: None,
        }
    }

    pub fn offset(&mut self, offset: Integer) -> &mut Self {
        self.offset = Some(offset);
        self
    }

    pub fn limit(&mut self, limit: Integer) -> &mut Self {
        self.limit = Some(limit);
        self
    }
}

/// Get a list of profile pictures for a user.
pub trait CanGetUserProfilePhotos {
    fn get_user_profile_photos(&self) -> GetUserProfilePhotos;
}

impl<'b, U> CanGetUserProfilePhotos for U
where
    U: ToUserId,
{
    fn get_user_profile_photos(&self) -> GetUserProfilePhotos {
        GetUserProfilePhotos::new(self)
    }
}
