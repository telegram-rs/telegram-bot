//! Connector with hyper backend.

pub mod hyper;

use std::fmt::Debug;
use std::pin::Pin;

use futures::Future;
use telegram_bot_raw::{HttpRequest, HttpResponse};

use crate::errors::Error;

pub trait Connector: Debug + Send + Sync {
    fn request(
        &self,
        token: &str,
        req: HttpRequest,
    ) -> Pin<Box<dyn Future<Output = Result<HttpResponse, Error>> + Send>>;
}

pub fn default_connector() -> Box<dyn Connector> {
    hyper::default_connector().unwrap()
}
