pub use crate::strnum::FloatOrString;
use serde::Deserialize;

/// Container for a user portfolio
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Portfolio {
    pub portid: String,
    pub portname: String,
    pub num_stocks: FloatOrString,
    pub uid: String,
    pub id: String,
    pub intro: String,
    pub introduction: String,
    pub private: FloatOrString,
    pub settings: String,
    pub created: String,
    pub is_deleted: String,
    pub alert: String,
    pub email: String,
    pub modified: String,
    pub p_1m: FloatOrString,
    pub p_3m: FloatOrString,
    pub p_6m: FloatOrString,
    pub p_12m: FloatOrString,
    pub p_3y: FloatOrString,
    pub p_5y: FloatOrString,
    pub p_10y: FloatOrString,
    pub p_all: FloatOrString,
    pub p_rel_sp500: FloatOrString,
    pub detail: Vec<Position>,
    pub deleted_time: Option<String>,
    pub is_article: Option<String>,
    pub gain: Option<FloatOrString>,
    #[serde(rename = "type")]
    pub portfolio_type: FloatOrString,
    pub value: Option<FloatOrString>,
    pub view_id: String,
    pub stocks: FloatOrString,
    pub description: String,
}

/// Container for a stock position held by the user
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Position {
    pub id: String,
    pub company: String,
    pub cost_per_share: FloatOrString,
    pub shares: FloatOrString,
    pub symbol: String,
    pub volumn: FloatOrString,
    pub currency: String,
    pub date_add: String,
    pub price: FloatOrString,
    pub pettm: FloatOrString,
    pub p_change: FloatOrString,
    #[serde(rename = "p_pct-change")]
    pub p_pct_change: FloatOrString,
    pub gain: FloatOrString,
    pub gain_p: FloatOrString,
    pub gain_today: FloatOrString,
    pub open: FloatOrString,
    pub low: FloatOrString,
    pub high: FloatOrString,
    pub in_price: FloatOrString,
    pub ps: FloatOrString,
    pub pb: FloatOrString,
}

#[cfg(test)]
mod test {
    use super::super::*;
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_portfolio() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let portfolios = gf_connect.get_personal_portfolio().await;
                assert!(portfolios.is_ok());
                let portfolios = serde_json::from_value::<Vec<Portfolio>>(portfolios.unwrap());
                assert!(portfolios.is_ok());
            }
        }
    }
}
