use serde::Deserialize;
pub use crate::strnum::FloatOrString;


/// Container for a user portfolio
#[derive(Deserialize, Debug)]
pub struct Portfolio {
    pub portid: String,
    pub portname: String,
    pub num_stocks: i32,
    pub description: String,
    pub uid: String,
    pub id: String,
    pub intro: String,
    pub private: i32,
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
}

/// Container for a stock position held by the user
#[derive(Deserialize, Debug)]
pub struct Position {
    pub id: String,
    pub company: String,
    pub cost_per_share: FloatOrString,
    pub shares: FloatOrString,
    pub symbol: String,
    pub volume: FloatOrString,
    pub currency: String,
    pub date_add: String,
    pub price: FloatOrString,
    pub pettm: FloatOrString,
    pub p_change: FloatOrString,
    pub p_pct_change: FloatOrString,
    pub gain: FloatOrString,
    pub gain_p: String,
    pub gain_today: FloatOrString,
    pub open: FloatOrString,
    pub low: FloatOrString,
    pub high: FloatOrString,
    pub in_price: FloatOrString,
    pub ps: FloatOrString,
    pub pb: FloatOrString,
}
