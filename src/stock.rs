use serde::Deserialize;

/// Container for basic data for a single stock
#[derive(Deserialize, Debug)]
pub struct Stock {
    /// Name of the company
    pub company: String,
    /// Currency in which the stock is quoted
    pub currency: String,
    /// ID of the exchange the stock is quoted ad
    pub exchange:  String,
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
    currency: String,
    /// Current day's price change
    #[serde(rename = "Day\'s Change")]
    todays_change: String,
    /// Current day's trading volume
    #[serde(rename = "Day\'s Volume")]
    todays_volume: i32,
    /// Exchange's symbol
    #[serde(rename = "Exchange")]
    exchange: String,
    /// Last quoted price
    #[serde(rename = "price")]    
    price: String,
    /// Last price change
    #[serde(rename = "Price Change")]
    price_change: String,
    /// Time of last price update
    #[serde(rename = "Price Updated Time")]
    update_time: String,
    /// Stock ticker symbol
    #[serde(rename = "Symbol")]
    symbol: String,
    /// Today's highest price
    high: String,
    /// Today's lowest price
    low: String,
    /// Today's opening price
    open: String,
    /// Timestamp
    timestamp: i32,
}
