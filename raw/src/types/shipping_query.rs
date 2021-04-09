use crate::types::*;

#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ShippingQuery {
    pub id: CallbackQueryId,
    pub from: User,
    pub invoice_payload: String,
    pub shipping_address: ShippingAddress
}
