use std::env;

const TELEGRAM_API_URL_DEFAULT: &str = "https://api.telegram.org/";

/// Obtains URL to the Telegram Bot API. You're able to change this URL to point to fake Telegram server
/// for E2E-testing by setting `TELEGRAM_API_URL` environment variable.
pub fn telegram_api_url() -> String {
    match env::var("TELEGRAM_API_URL") {
        Ok(url) => url,
        Err(_) => String::from(TELEGRAM_API_URL_DEFAULT),
    }
}
