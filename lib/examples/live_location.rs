use std::env;
use std::time::Duration;

use futures::StreamExt;
use telegram_bot::*;
use tokio::time::sleep;

const DELAY_DURATION: Duration = Duration::from_secs(2);

async fn test(api: Api, message: Message) -> Result<(), Error> {
    let reply = api
        .send(message.location_reply(0.0, 0.0).live_period(60))
        .await?;

    sleep(DELAY_DURATION).await;
    api.send(reply.edit_live_location(10.0, 10.0)).await?;

    sleep(DELAY_DURATION).await;
    api.send(reply.edit_live_location(20.0, 20.0)).await?;

    sleep(DELAY_DURATION).await;
    api.send(reply.edit_live_location(30.0, 30.0)).await?;

    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let api = Api::new(token);
    let mut stream = api.stream();

    while let Some(update) = stream.next().await {
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                match data.as_str() {
                    "/livelocation" => test(api.clone(), message.clone()).await?,
                    _ => (),
                }
            }
        }
    }
    Ok(())
}
