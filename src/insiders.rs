use serde::Deserialize;

pub use crate::strnum::FloatOrString;

/// Container for the insider trades
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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


#[cfg(test)]
mod test {
    use std::env;
    use std::collections::HashMap;
    use super::*;
    use super::super::*;

    #[tokio::test]
    async fn test_insider_trades() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let stock = "NAS:NVDA";
                let trades = gf_connect.get_insider_trades(stock).await;
                assert!(trades.is_ok());
                let trades = serde_json::from_value::<HashMap<String, Vec<InsiderTrade>>>(trades.unwrap());
                assert!(trades.is_ok());
            }
        }
    }   

    #[tokio::test]
    async fn test_insider_updates() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let updates = gf_connect.get_insider_updates().await;
                assert!(updates.is_ok());
                let updates = serde_json::from_value::<Vec<InsiderUpdate>>(updates.unwrap());
                assert!(updates.is_ok());
            }
        }
    }   


}