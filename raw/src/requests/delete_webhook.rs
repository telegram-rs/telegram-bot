use requests::*;

/// Use this method to delete webhook
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct DeleteWebhook;

impl Request for DeleteWebhook {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("deleteWebhook"), self)
    }
}

impl DeleteWebhook {
    pub fn new() -> Self {
        DeleteWebhook
    }
}
