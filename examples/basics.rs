#![allow(unused_imports)]
extern crate rustc_serialize;
extern crate telegram_bot;

use telegram_bot::*;
use std::env;
use rustc_serialize::json;

fn main() {
    let token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(tok) => tok,
        Err(e) =>
            panic!("Environment variable 'TELEGRAM_BOT_TOKEN' missing! {}", e),
    };

    let mut bot = Bot::new(token);
    println!("{:?}", bot.get_me());

    // let keyboard = ReplyKeyboardMarkup {
    //     keyboard: vec![vec!["Hi".into()],
    //                    vec!["A".into(), "B".into()]],
    //     .. Default::default()
    // };

    // println!("{}", json::encode(&keyboard).unwrap());

    let res = bot.long_poll(None, |bot, u| {
        if let Some(m) = u.message {
            let name = m.from.first_name + &*m.from.last_name
                .map_or("".to_string(), |mut n| { n.insert(0, ' '); n });

            match m.msg {
                MessageType::Text(t) => {
                    println!("Received Update[{}]: <{}> {}",
                    u.update_id, name, t);
                    let keyboard = ReplyKeyboardMarkup {
                        keyboard: vec![vec![t],
                                       vec!["A".into(), "B".into()]],
                       one_time_keyboard: Some(true),
                        .. Default::default()
                    };

                    let res = bot.send_message(
                        m.chat.id(),
                        format!("Hi, {}", name),
                        None,
                        None,
                        Some(keyboard));
                },
                MessageType::Location(loc) => {
                    // Log
                    println!("{} is here: (lng: {}, lat: {})",
                             name, loc.longitude, loc.latitude);

                    // Calculate and send the location on the other side of the
                    // earth.
                    let lat = -loc.latitude;
                    let lng = if loc.longitude > 0.0 {
                        loc.longitude - 180.0
                    } else {
                        loc.longitude + 180.0
                    };

                    bot.send_location(m.chat.id(), lat, lng, None, None);
                },
                _ => {}
            }
        }
    });

    if let Err(e) = res {
        println!("An error occured: {}", e);
    }

    // println!("{}", u.unwrap_err());
    // println!("{:?}", );

    // let u : Result<Update, _> = json::decode(r#"
    //     {
    //   "update_id": 102520066,
    //   "message": {
    //     "message_id": 6,
    //     "from": {
    //       "id": 10742080,
    //       "first_name": "Lukas",
    //       "last_name": "Kalbertodt",
    //       "username": "LukasKalbertodt"
    //     },
    //     "chat": {
    //       "id": 10742080,
    //       "first_name": "Lukas",
    //       "last_name": "Kalbertodt",
    //       "username": "LukasKalbertodt"
    //     },
    //     "date": 1435244203,
    //     "forward_from": {
    //       "id": 10742080,
    //       "first_name": "Lukas",
    //       "last_name": "Kalbertodt",
    //       "username": "LukasKalbertodt"
    //     },
    //     "forward_date": 1435243061
    //   }
    // }
    // "#);
    // println!("{:?}", u);
}
