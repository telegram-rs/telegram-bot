use std::env;

use futures::StreamExt;
use telegram_bot::*;

async fn process(api: Api, message: Message) -> Result<(), Error> {
    if let MessageKind::Text { ref data, .. } = message.kind {
        match data.as_str() {
            "/pin" => {
                if let Some(reply) = message.reply_to_message {
                    api.send(reply.pin()).await?;
                }
            }
            "/unpin" => api.send(message.chat.unpin_message()).await?,

            _ => (),
        }
    }
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let api = Api::new(token);
    let mut stream = api.stream();

    // Fetch new updates via long poll method
    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            process(api.clone(), message).await?
        }
    }

    Ok(())
}
