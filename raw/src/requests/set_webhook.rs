use requests::*;
use types::Integer;

use std::borrow::Cow;

/// Use this method to set webhook
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetWebhook<'t> {
    url: Cow<'t, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections: Option<Integer>,
    #[serde(skip_serializing_if = "Vec::is_empty")]
    allowed_updates: Vec<AllowedUpdate>,
}

impl<'t> Request for SetWebhook<'t> {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("setWebhook"), self)
    }
}

impl<'t> SetWebhook<'t> {
    pub fn new<T>(url: T) -> Self
    where
        T: Into<Cow<'t, str>>,
    {
        SetWebhook {
            url: url.into(),
            max_connections: None,
            allowed_updates: Vec::new(),
        }
    }

    pub fn allowed_updates<T>(&mut self, updates: &[AllowedUpdate]) -> &mut Self {
        self.allowed_updates = updates.into();
        self
    }

    pub fn max_connections(&mut self, n: Integer) -> &mut Self {
        self.max_connections = Some(n);
        self
    }
}
