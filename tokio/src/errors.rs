use telegram_bot_raw::ResponseParameters;

error_chain! {
    foreign_links {
        Url(::url::ParseError);
        Hyper(::hyper::Error);
        Json(::serde_json::Error);
        Io(::std::io::Error);
    }

    errors {
        TelegramError {
            description: String,
            parameters: Option<ResponseParameters>
        }
    }
}
