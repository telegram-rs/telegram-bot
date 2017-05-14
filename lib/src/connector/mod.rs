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

pub trait ConnectorConfig {
    fn take(self, &Handle) -> Box<Connector>;
}

#[derive(Debug, Copy, Clone)]
pub struct DefaultConnector;

impl ConnectorConfig for DefaultConnector {
    fn take(self, handle: &Handle) -> Box<Connector> {
        default_connector(handle)
    }
}

#[derive(Debug)]
pub struct SpecifiedConnector {
    connector: Box<Connector>,
}

impl SpecifiedConnector {
    pub fn new(connector: Box<Connector>) -> Self {
        Self {
            connector: connector
        }
    }
}

impl ConnectorConfig for SpecifiedConnector {
    fn take(self, _handle: &Handle) -> Box<Connector> {
        self.connector
    }
}

#[cfg(feature = "curl_connector")]
fn default_connector(handle: &Handle) -> Box<Connector> {
    curl::default_connector(handle)
}

#[cfg(all(not(feature = "curl_connector"), all(feature = "hyper_connector")))]
fn default_connector(handle: &Handle) -> Box<Connector> {
    hyper::default_connector(handle)
}