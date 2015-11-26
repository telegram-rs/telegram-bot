extern crate telegram_bot;

use telegram_bot::*;

fn main() {
    // Create bot, test simple API call and print bot information
    let api = Api::from_env("TELEGRAM_BOT_TOKEN").unwrap();
    println!("getMe: {:?}", api.get_me());
    let mut listener = api.listener(ListeningMethod::LongPoll(None));

    // Fetch new updates via long poll method
    let res = listener.listen(|u| {
        // If the received update contains a message...
        if let Some(m) = u.message {
            let name = m.from.first_name;

            // Match message type
            match m.msg {
                MessageType::Text(t) => {
                    // Print received text message to stdout
                    println!("<{}> {}", name, t);

                    if t == "/exit" {
                        return Ok(ListeningAction::Stop);
                    }

                    // Answer message with "Hi"
                    try!(api.send_message(
                        m.chat.id(),
                        format!("Hi, {}! You just wrote '{}'", name, t),
                        None, None, None, None));
                },
                _ => {}
            }
        }

        // If none of the "try!" statements returned an error: It's Ok!
        Ok(ListeningAction::Continue)
    });

    if let Err(e) = res {
        println!("An error occured: {}", e);
    }
}
