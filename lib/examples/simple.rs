extern crate futures;
extern crate telegram_bot;
extern crate tokio_core;

use std::env;

use futures::Stream;
use tokio_core::reactor::Core;
use telegram_bot::*;

fn main() {
    let mut core = Core::new().unwrap();

    let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
    let api = Api::configure(token).build(core.handle());

    // Fetch new updates via long poll method
    let future = api.stream().for_each(|update| {

        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {

            // Get sender's first name if available.
            let first_name = match message.from.as_ref() {
                Some(from) => &from.first_name,
                None => return Ok(()) // Skip a message.
            };

            if let MessageKind::Text {ref data, ..} = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", first_name, data);

                // Answer message with "Hi".
                api.spawn(message.text_reply(
                    format!("Hi, {}! You just wrote '{}'", first_name, data)
                ));
            }
        }

        Ok(())
    });

    core.run(future).unwrap();
}
