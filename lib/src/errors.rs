use telegram_bot_raw;

error_chain! {
    foreign_links {
        Url(::hyper::error::UriError) #[cfg(feature = "hyper_connector")];
        Hyper(::hyper::Error) #[cfg(feature = "hyper_connector")];
        Curl(::curl::Error) #[cfg(feature = "curl_connector")];
        CurlPerformError(::tokio_curl::PerformError) #[cfg(feature = "curl_connector")];
        Io(::std::io::Error);
    }

    links {
        Raw(telegram_bot_raw::Error, telegram_bot_raw::ErrorKind);
    }
}
