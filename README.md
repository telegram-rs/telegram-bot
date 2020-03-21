Rust Telegram Bot Library
=========================
[![Build Status](https://img.shields.io/travis/telegram-rs/telegram-bot/master.svg)](https://travis-ci.org/telegram-rs/telegram-bot)
[![Tests](https://github.com/telegram-rs/telegram-bot/workflows/Tests/badge.svg)](https://github.com/telegram-rs/telegram-bot/actions?workflow=Tests)
[![Tests](https://github.com/telegram-rs/telegram-bot/workflows/Fmt/badge.svg)](https://github.com/telegram-rs/telegram-bot/actions?workflow=Fmt)
[![License](https://img.shields.io/github/license/telegram-rs/telegram-bot.svg)]()
[![Crates.io](https://img.shields.io/crates/v/telegram-bot.svg)](https://crates.io/crates/telegram-bot)

<table>
  <tbody>
    <tr>
      <td><b>Documentation:</b></td>
      <td><a href="https://docs.rs/telegram-bot/">Latest crates.io version</a></td>
      <td><a href="https://telegram-rs.github.io/telegram-bot/telegram_bot/"><code>master</code></a></td>
    </tr>
  </tbody>
</table>

A library for writing your own [Telegram](https://telegram.org/) bots. More information [here](https://core.telegram.org/bots). Official API [here](https://core.telegram.org/bots/api).

## Example
Here is a simple example (see [`example/simple.rs`](https://github.com/telegram-rs/telegram-bot/blob/master/lib/examples/simple.rs)):

``` rust
use std::env;

use futures::StreamExt;
use telegram_bot::*;

#[tokio::main]
async fn main() -> Result<(), Error> {
    let token = env::var("TELEGRAM_BOT_TOKEN").expect("TELEGRAM_BOT_TOKEN not set");
    let api = Api::new(token);

    // Fetch new updates via long poll method
    let mut stream = api.stream();
    while let Some(update) = stream.next().await {
        // If the received update contains a new message...
        let update = update?;
        if let UpdateKind::Message(message) = update.kind {
            if let MessageKind::Text { ref data, .. } = message.kind {
                // Print received text message to stdout.
                println!("<{}>: {}", &message.from.first_name, data);

                // Answer message with "Hi".
                api.send(message.text_reply(format!(
                    "Hi, {}! You just wrote '{}'",
                    &message.from.first_name, data
                )))
                .await?;
            }
        }
    }
    Ok(())
}
```
You can find a bigger examples in the `examples`.

## Usage
This library is available via `crates.io`. In order to use it, just add this to your `Cargo.toml`:

```
telegram-bot = "0.7"
```

The library allows you to do E2E-testing of your bot easily: just specify `TELEGRAM_API_URL` environment variable to point to your fake Telegram test server.
A lot of diagnostic information can be collected with [tracing](https://crates.io/crates/tracing) framework, see [`example/tracing.rs`](https://github.com/telegram-rs/telegram-bot/blob/master/lib/examples/tracing.rs)).

## Collaboration
Yes please! Every type of contribution is welcome: Create issues, hack some code or make suggestions. Don't know where to start? Good first issues are tagged with [up for grab](https://github.com/telegram-rs/telegram-bot/issues?q=is%3Aissue+is%3Aopen+label%3A%22up+for+grab%22).
