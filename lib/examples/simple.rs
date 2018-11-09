extern crate futures;
extern crate telegram_bot;
extern crate tokio;

use std::env;

use futures::{future::lazy, Stream};

use telegram_bot::*;

fn main() {
    let mut runtime = tokio::runtime::current_thread::Runtime::new().unwrap();
    runtime
        .block_on(lazy(|| {
            let token = env::var("TELEGRAM_BOT_TOKEN").unwrap();
            let api = Api::configure(token).build().unwrap();

            let stream = api.stream().then(|mb_update| {
                let res: Result<Result<Update, Error>, ()> = Ok(mb_update);
                res
            });

            tokio::executor::current_thread::spawn(stream.for_each(move |update| {
                match update {
                    Ok(update) => {
                        // If the received update contains a new message...
                        if let UpdateKind::Message(message) = update.kind {
                            if let MessageKind::Text { ref data, .. } = message.kind {
                                // Print received text message to stdout.
                                println!("<{}>: {}", &message.from.first_name, data);

                                // Answer message with "Hi".
                                api.spawn(message.text_reply(format!(
                                    "Hi, {}! You just wrote '{}'",
                                    &message.from.first_name, data
                                )));
                            }
                        }
                    }
                    Err(_) => {}
                }

                Ok(())
            }));

            Ok::<_, ()>(())
        }))
        .unwrap();

    runtime.run().unwrap();
}
