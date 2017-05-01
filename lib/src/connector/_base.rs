use future::TelegramFuture;

pub trait Connector {
    fn post_json(&self, uri: &str, data: Vec<u8>) -> TelegramFuture<Vec<u8>>;
}
