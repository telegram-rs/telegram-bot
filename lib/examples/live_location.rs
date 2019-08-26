use std::env;
use std::time::{Duration, Instant};

use futures::StreamExt;
use telegram_bot::*;
use tokio::timer::Delay;

async fn test(api: Api, message: Message) -> Result<(), Error> {
    let when = Instant::now() + Duration::from_secs(2);

    let mut reply = message.location_reply(0.0, 0.0);
    api.send(reply.live_period(60)).await?;
    Delay::new(when).await;

    api.send(message.edit_live_location(10.0, 10.0)).await?;
    Delay::new(when).await;

    api.send(message.edit_live_location(20.0, 20.0)).await?;
    Delay::new(when).await;

    api.send(message.edit_live_location(30.0, 30.0)).await?;
    Delay::new(when).await;

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
