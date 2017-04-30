use telegram_bot_raw::ResponseParameters;

error_chain! {
    foreign_links {
        Url(::hyper::error::UriError) #[cfg(feature = "hyper_connector")];
        Hyper(::hyper::Error) #[cfg(feature = "hyper_connector")];
        Curl(::curl::Error) #[cfg(feature = "curl_connector")];
        CurlPerformError(::tokio_curl::PerformError) #[cfg(feature = "curl_connector")];
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
