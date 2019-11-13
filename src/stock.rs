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
