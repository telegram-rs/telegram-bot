use crate::types::*;

/// This object contains information about an incoming shipping query.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ShippingQuery {
    /// Unique query identifier
    pub id: CallbackQueryId,
    /// User who sent the query
    pub from: User,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// User specified shipping address
    pub shipping_address: ShippingAddress,
}
