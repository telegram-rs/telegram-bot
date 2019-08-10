use telegram_bot_raw;

error_chain! {
    foreign_links {
        Url(::hyper::http::uri::InvalidUri);
        Hyper(::hyper::Error);
        Io(::std::io::Error);
        Timeout(::tokio::timer::timeout::Elapsed);
    }

    links {
        Raw(telegram_bot_raw::Error, telegram_bot_raw::ErrorKind);
    }
}
