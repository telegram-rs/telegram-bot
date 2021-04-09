use crate::types::*;

/// This object contains information about an incoming pre-checkout query.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct PreCheckoutQuery {
    /// Unique query identifier
    pub id: CallbackQueryId,
    /// User who sent the query
    pub from: User,
    /// Three-letter ISO 4217 currency code
    pub currency: String,
    /// Total price in the smallest units of the currency (integer, not float/double). For example, for a price of US$ 1.45 pass amount = 145. See the exp parameter in currencies.json, it shows the number of digits past the decimal point for each currency (2 for the majority of currencies).
    pub total_amount: Integer,
    /// Bot specified invoice payload
    pub invoice_payload: String,
    /// Optional. Identifier of the shipping option chosen by the user
    pub shipping_option_id: Option<String>,
    /// Optional. Order info provided by the user
    pub order_info: Option<OrderInfo>,
}

/// This object represents information about an order.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct OrderInfo {
    /// Optional. User name
    pub name: Option<String>,
    /// Optional. User's phone number
    pub phone_number: Option<String>,
    /// Optional. User email
    pub email: Option<String>,
    /// Optional. User shipping address
    pub shipping_address: Option<ShippingAddress>,
}

/// This object represents a shipping address.
#[derive(Debug, Clone, PartialEq, PartialOrd, Deserialize)]
pub struct ShippingAddress {
    /// ISO 3166-1 alpha-2 country code
    pub country_code: String,
    /// State, if applicable
    pub state: String,
    /// City
    pub city: String,
    /// First line for the address
    pub street_line1: String,
    /// Second line for the address
    pub street_line2: String,
    /// Address post code
    pub post_code: String,
}
