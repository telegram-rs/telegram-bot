Rust Telegram Bot Library
=========================
[![Build Status](https://img.shields.io/travis/LukasKalbertodt/telegram-bot.svg)](https://travis-ci.org/LukasKalbertodt/telegram-bot)
[![License](https://img.shields.io/github/license/LukasKalbertodt/telegram-bot.svg)]()

[**Documentation**](https://lukaskalbertodt.github.io/telegram-bot/telegram_bot/)

A library for writing your own [Telegram](https://telegram.org/) bots. More information [here](https://core.telegram.org/bots). **Note:** Work in progress!

## Example
Here is a simple example (see [`example/simple.rs`](https://github.com/LukasKalbertodt/telegram-bot/blob/master/examples/simple.rs)):

``` rust
extern crate telegram_bot;

use telegram_bot::*;
use std::env;

fn main() {
    // Fetch environment variable with bot token
    let token = match env::var("TELEGRAM_BOT_TOKEN") {
        Ok(tok) => tok,
        Err(e) =>
            panic!("Environment variable 'TELEGRAM_BOT_TOKEN' missing! {}", e),
    };

    // Create bot, test simple API call and print bot information
    let mut bot = Bot::new(token);
    println!("getMe: {:?}", bot.get_me());

    // Fetch new updates via long poll method
    let res = bot.long_poll(None, |bot, u| {
        // If the received update contains a message...
        if let Some(m) = u.message {
            let name = m.from.first_name;

            // Match message type
            match m.msg {
                MessageType::Text(t) => {
                    // Print received text message to stdout
                    println!("<{}> {}", name, t);

                    if t == "/exit" {
                        return Err(Error::UserInterrupt);
                    }

                    // Answer message with "Hi"
                    try!(bot.send_message(
                        m.chat.id(),
                        format!("Hi, {}! You just wrote '{}'", name, t),
                        None, None, None));
                },
                _ => {}
            }
        }

        // If none of the "try!" statements returned an error: It's Ok!
        Ok(())
    });

    // When the method `long_poll` returns, its due to an error. Check it here.
    if let Err(e) = res {
        println!("An error occured: {}", e);
    }
}
```
You can find a bigger example in the `examples` folder.

## Usage
Will be uploaded to crates.io soon...

## Collaboration
Yes please! Every type of colaboration is welcome: Create issues, hack some code or make suggestions. If you don't know where to start, just contact me (my email is on my github profile).

## Todo

- [x] "getMe"
- [x] Methods without files
  - [x] "getMe"
  - [x] "sendMessage"
  - [x] "forwardMessage"
  - [x] "sendLocation"
  - [x] "sendChatAction"
  - [x] "getUserProfilePhotos"
- [x] "getUpdates" and `long_poll`
- [ ] "setWebhook" and `listen`
- [ ] sending files ("sendAudio", "sendDocument", ...)
- [ ] More good documentation and examples
- [ ] Maybe think about multithreading stuff
