use std::env;

use futures::StreamExt;

use telegram_bot::prelude::*;
use telegram_bot::{reply_markup, Api, Error, InlineKeyboardMarkup, MessageKind, UpdateKind};

const START_LAT: f32 = 55.753960;
const START_LON: f32 = 37.620393;

const COORD_STEP_LAT: f32 = 0.001;
const COORD_STEP_LON: f32 = 0.002;

fn make_inline_keyboard_markup() -> InlineKeyboardMarkup {
    reply_markup!(inline_keyboard,
        ["" callback "x",      "⬆️" callback "up",    "" callback "x"],
        ["⬅️" callback "left",  "" callback "x",      "➡️" callback "right"],
        ["" callback "x",      "⬇️" callback "down",  "" callback "xx"]
    )
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");

    let api = Api::new(token);
    let mut stream = api.stream();

    let mut lat = START_LAT;
    let mut lon = START_LON;
    let mut location_msg = None;

    while let Some(update) = stream.next().await {
        let update = update?;

        match update.kind {
            UpdateKind::Message(message) => match message.kind {
                MessageKind::Text { ref data, .. } => match data.as_str() {
                    "/start" => {
                        let keyboard = make_inline_keyboard_markup();
                        api.send(message.text_reply("☢ Nuclear Bomb Targeting ☢"))
                            .await?;

                        lat = START_LAT;
                        lon = START_LON;
                        location_msg = Some(
                            api.send(
                                message
                                    .location_reply(lat, lon)
                                    .live_period(10000)
                                    .reply_markup(keyboard),
                            )
                            .await?,
                        );
                    }
                    _ => (),
                },
                _ => (),
            },
            UpdateKind::CallbackQuery(cb) => {
                if location_msg.is_none() {
                    api.send(cb.answer("Please use /start command")).await?;
                } else {
                    match &cb.data {
                        Some(ans) if ans == "up" => lat += COORD_STEP_LAT,
                        Some(ans) if ans == "down" => lat -= COORD_STEP_LAT,
                        Some(ans) if ans == "right" => lon += COORD_STEP_LON,
                        Some(ans) if ans == "left" => lon -= COORD_STEP_LON,
                        _ => (),
                    };
                    api.send(cb.answer("Moved")).await?;

                    let new_keyboard = make_inline_keyboard_markup();
                    if let Some(msg) = &location_msg {
                        api.send(msg.edit_live_location(lat, lon).reply_markup(new_keyboard))
                            .await?;
                    }
                }
            }
            _ => (),
        }
    }

    Ok(())
}
