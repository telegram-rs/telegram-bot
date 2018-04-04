use requests::*;

use std::borrow::Cow;

/// Use this method to set webhook
#[derive(Debug, Clone, PartialEq, PartialOrd, Serialize)]
#[must_use = "requests do nothing unless sent"]
pub struct SetWebhook<'t, 'u> {
    url: Cow<'t, str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    max_connections: Option<i32>,
    #[serde(skip_serializing_if = "Option::is_none")]
    allowed_updates: Option<Vec<Cow<'u, str>>>,
}

impl<'t, 'u> Request for SetWebhook<'t, 'u> {
    type Type = JsonRequestType<Self>;
    type Response = JsonTrueToUnitResponse;

    fn serialize(&self) -> Result<HttpRequest, Error> {
        Self::Type::serialize(RequestUrl::method("setWebhook"), self)
    }
}

impl<'t, 'u> SetWebhook<'t, 'u> {
    pub fn new<T>(url: T) -> Self
    where
        T: Into<Cow<'t, str>>,
    {
        SetWebhook {
            url: url.into(),
            max_connections: None,
            allowed_updates: None,
        }
    }

    pub fn allowed_updates<T>(&mut self, updates: Vec<T>) -> &mut Self
    where
        T: Into<Cow<'u, str>>,
    {
        let updates = updates.into_iter().map(|x| x.into()).collect();
        self.allowed_updates = Some(updates);
        self
    }

    pub fn max_connections(&mut self, n: i32) -> &mut Self {
        self.max_connections = Some(n);
        self
    }
}
