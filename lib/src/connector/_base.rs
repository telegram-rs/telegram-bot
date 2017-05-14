use std::fmt::Debug;

use future::TelegramFuture;

pub trait Connector: Debug {
    fn post_json(&self, uri: &str, data: Vec<u8>) -> TelegramFuture<Vec<u8>>;
}
