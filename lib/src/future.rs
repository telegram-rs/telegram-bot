use futures::{Future, Poll};

use errors::Error;

#[must_use = "futures do nothing unless polled"]
pub struct TelegramFuture<T> {
    pub inner: Box<Future<Item=T, Error=Error>>
}

impl<T> Future for TelegramFuture<T> {
    type Item = T;
    type Error = Error;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.inner.poll()
    }
}
