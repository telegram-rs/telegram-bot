use future::TelegramFuture;

pub trait Connector {
    fn post(&self, uri: &str, data: Vec<u8>) -> TelegramFuture<Vec<u8>>;
}
