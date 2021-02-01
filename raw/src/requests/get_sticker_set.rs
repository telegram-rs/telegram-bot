use crate::requests::*;
use crate::types::*;

/// Use this method to get a sticker set.
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct GetStickerSet {
    name: FileRef,
}

impl Request for GetStickerSet {
    type Type = JsonRequestType<Self>;
    type Response = JsonIdResponse<File>;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("getStickerSet"), self)
    }
}

impl GetStickerSet {
    pub fn new<S>(set: S) -> Self
    where
        S: ToFileRef,
    {
        Self {
            name: set.to_file_ref(),
        }
    }
}

/// Get basic info about a sticker set and prepare it for downloading.
pub trait CanGetStickerSet {
    fn get_set(&self) -> GetStickerSet;
}

impl<S> CanGetStickerSet for S
where
    S: ToFileRef,
{
    fn get_set(&self) -> GetStickerSet {
        GetStickerSet::new(self)
    }
}
