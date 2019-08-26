use std::env;

use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let mut stream = Api::new(token).stream();

    // Print update or error for each update.
    while let Some(mb_update) = stream.next().await {
        println!("{:?}", mb_update);
    }
}
