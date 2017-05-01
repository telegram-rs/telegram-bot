mod _base;
#[cfg(feature = "curl_connector")]
pub mod curl;
#[cfg(feature = "hyper_connector")]
pub mod hyper;

use tokio_core::reactor::Handle;

pub use self::_base::Connector;
#[cfg(feature = "curl_connector")]
pub use self::curl::CurlConnector;
#[cfg(feature = "hyper_connector")]
pub use self::hyper::HyperConnector;

#[cfg(feature = "curl_connector")]
pub fn default_connector(handle: &Handle) -> Box<Connector> {
    curl::default_connector(handle)
}

#[cfg(all(not(feature = "curl_connector"), all(feature = "hyper_connector")))]
pub fn default_connector(handle: &Handle) -> Box<Connector> {
    hyper::default_connector(handle)
}