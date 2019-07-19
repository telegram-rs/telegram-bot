extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use futures::Stream;
use telegram_bot::*;
use tokio_core::reactor::Core;

fn main() {
    let mut core = Core::new().unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::configure(token).build(core.handle()).unwrap();

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {
        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {
            api.spawn(dbg!(message.document_url_reply(
                "https://media.giphy.com/media/QvvtwToKIUUpWXDPK6/giphy.gif"
            )));
        }

        Ok(())
    });

    core.run(future).unwrap();
}
