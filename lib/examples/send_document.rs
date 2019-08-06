#![feature(async_await)]
use std::env;

use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    let mut stream = api.stream();

    // Fetch new updates via long poll method
    while let Some(update) = stream.next().await {
        let update = update?;
        // If the received update contains a new message...
        if let UpdateKind::Message(message) = update.kind {
            api.send(
                message.document_url_reply(
                    "https://media.giphy.com/media/QvvtwToKIUUpWXDPK6/giphy.gif",
                ),
            )
            .await?;
        }
    }

    Ok(())
}
