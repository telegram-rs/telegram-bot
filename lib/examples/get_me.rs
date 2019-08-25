use std::env;

use telegram_bot::{Api, Error, GetMe};

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let api = Api::new(token);
    let result = api.send(GetMe).await?;
    println!("{:?}", result);
    Ok(())
}
