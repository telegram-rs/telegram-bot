use telegram_bot_raw;

error_chain! {
    foreign_links {
        Url(::hyper::http::uri::InvalidUri);
        Hyper(::hyper::Error);
        Http(::hyper::http::Error);
        InvalidHeaderValue(::hyper::http::header::InvalidHeaderValue);
        Io(::std::io::Error);
    }

    links {
        Raw(telegram_bot_raw::Error, telegram_bot_raw::ErrorKind);
    }

    errors {
        InvalidMultipartFilename
    }
}
