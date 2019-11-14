use serde::Deserialize;

pub use crate::strnum::string_or_num;
pub use crate::strnum::FloatOrString;

/// Container for basic data for a single stock
#[derive(Deserialize, Debug)]
pub struct Stock {
    /// Name of the company
    pub company: String,
    /// Currency in which the stock is quoted
    pub currency: String,
    /// ID of the exchange the stock is quoted ad
    pub exchange: String,
    /// Main industry the company operates in
    pub industry: String,
    /// Industry sector the company belongs to
    pub sector: String,
    /// Main subindustry the company operates in
    pub subindustry: String,
    /// Stock ticker symbol
    pub symbol: String,
}

/// Container for single quote data
#[derive(Deserialize, Debug)]
pub struct Quote {
    /// Currency of quoted stock price
    #[serde(rename = "Currency")]
    pub currency: String,
    /// Current day's price change
    #[serde(rename = "Day\'s Change")]
    #[serde(deserialize_with = "string_or_num")]
    pub todays_change: FloatOrString,
    /// Current day's trading volume
    #[serde(rename = "Day\'s Volume")]
    #[serde(deserialize_with = "string_or_num")]
    pub todays_volume: FloatOrString,
    /// Exchange's symbol
    #[serde(rename = "Exchange")]
    pub exchange: String,
    /// Last quoted price
    #[serde(rename = "Price")]
    #[serde(deserialize_with = "string_or_num")]
    pub price: FloatOrString,
    /// Last price change
    #[serde(rename = "Price Change")]
    #[serde(deserialize_with = "string_or_num")]
    pub price_change: FloatOrString,
    /// Time of last price update
    #[serde(rename = "Price Updated Time")]
    pub update_time: String,
    /// Stock ticker symbol
    #[serde(rename = "Symbol")]
    pub symbol: String,
    /// Today's highest price
    #[serde(deserialize_with = "string_or_num")]
    pub high: FloatOrString,
    /// Today's lowest price
    #[serde(deserialize_with = "string_or_num")]
    pub low: FloatOrString,
    /// Today's opening price
    #[serde(deserialize_with = "string_or_num")]
    pub open: FloatOrString,
    /// Timestamp
    pub timestamp: i32,
}
