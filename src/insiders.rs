use serde::Deserialize;

pub use crate::strnum::FloatOrString;

/// Container for the guru holdings
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
