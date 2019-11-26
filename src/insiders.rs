use serde::Deserialize;

pub use crate::strnum::FloatOrString;

/// Container for the insider trades
#[derive(Deserialize, Debug)]
pub struct InsiderTrade {
    pub change: FloatOrString,
    pub cost: FloatOrString,
    pub date: String,
    pub final_share: FloatOrString,
    pub insider: String,
    pub position: String,
    pub price: FloatOrString,
    pub trans_share: FloatOrString,
    #[serde(rename = "type")]
    pub trade_type: String,
}

/// Container for latest updates on insider trades
#[derive(Deserialize, Debug)]
pub struct InsiderUpdate {
    pub final_share: FloatOrString,
    pub insider: String,
    pub date: String,
    pub position: String,
    pub price: FloatOrString,
    pub symbol: String,
    pub cost: FloatOrString,
    pub exchange: String,
    pub trans_share: FloatOrString,
    #[serde(rename = "type")]
    pub trade_type: String,
}