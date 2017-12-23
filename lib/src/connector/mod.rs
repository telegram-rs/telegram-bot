//! IO backend.
//!
//! `CurlConnector` is default connector unless feature `curl_connector` is disabled and
//! feature `hyper_connector` is enabled. This behaviour will change after hyper release.

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

use errors::Error;

/// Returns default connector.
///
/// See module level documentation for details.
#[cfg(feature = "curl_connector")]
pub fn default_connector(handle: &Handle) -> Result<Box<Connector>, Error> {
    curl::default_connector(handle)
}

/// Returns default connector.
///
/// See module level documentation for details.
#[cfg(all(not(feature = "curl_connector"), all(feature = "hyper_connector")))]
pub fn default_connector(handle: &Handle) -> Result<Box<Connector>, Error> {
    hyper::default_connector(handle)
}