use serde::Deserialize;
use std::collections::HashMap;

pub use crate::strnum::FloatOrString;

/// Structure holding basic data for a single Guru.
#[derive(Deserialize, Debug)]
pub struct Guru {
    /// Unique identifier for a Guru.
    pub id: String,
    /// The Gurus name, could a person or an institution
    pub name: String,
    /// A link to the Guru's homepage, if available, otherwise None
    pub url: Option<String>,
    /// The organisation the Guru is working for.
    pub company: String,
    /// The number of stock holdings tracked by GuruFocus.
    pub num_of_stocks: FloatOrString,
    /// Total investment value in million US$.
    pub value: FloatOrString,
    /// Turnover rate in %.
    pub turnover: FloatOrString,
    /// Date of latest update of this data.
    pub latest_update: String,
}

/// Container for all basic Guru data.
#[derive(Deserialize, Debug)]
pub struct Gurus {
    /// Map holding the list of Gurus per country.
    pub all: HashMap<String, Vec<Guru>>,
    /// For each personal list of Gurus (e.g. default and custom list),
    /// a vector of Guru IDs is stored.
    pub my: HashMap<String, Vec<String>>,
}

/// Container for the guru holdings
#[derive(Deserialize, Debug)]
pub struct GuruTrades {
    /// Array of gurus position in the stock
    holdings : Vec<GuruHoldings>,
    /// Array of recent guru picks in the stock
    picks: Vec<GuruPicks2>
}

/// Container for the guru holdings
#[derive(Deserialize, Debug)]
pub struct GuruHoldings {
    pub change: FloatOrString,
    pub current_shares: FloatOrString,
    pub date: String,
    pub guru: String,
    pub guru_id: String,
    pub perc_assets: FloatOrString,
    pub perc_shares: FloatOrString,
}

/// Container for the guru holdings
#[derive(Deserialize, Debug)]
pub struct GuruPicks2 {
    #[serde(rename = "Avg")]
    pub avg: FloatOrString,
    pub action: String,
    pub comment: String,
    pub current_shares: FloatOrString,
    pub date: String,
    pub guru: String,
    pub guru_id: String,
    pub impact: FloatOrString,
    pub price_max: FloatOrString,
    pub price_min: FloatOrString,
}

#[derive(Deserialize, Debug)]
pub struct GuruPicks {
    port: Vec<GuruPick>,
}

#[derive(Deserialize, Debug)]
pub struct GuruPick {
    #[serde(rename = "GuruName")]
    pub guru_name: String,
    #[serde(rename = "RecmAction")]
    pub recm_action: String,
    #[serde(rename = "RecmDate")]
    pub recm_date: String,
    #[serde(rename = "RecmPrice")]
    pub recm_price: FloatOrString,
    pub change: FloatOrString,
    pub comment: String,
    pub company: String,
    pub currency: String,
    pub currency_txt: String,
    pub price: FloatOrString,
    pub price_max: FloatOrString,
    pub price_min: FloatOrString,
    pub sector: String,
    pub share_current: FloatOrString,
    pub symbol: String,
    pub symbol_ori: String,
    pub trans_share: FloatOrString,
    #[serde(rename = "type")]
    pub transaction_type: String,
}
