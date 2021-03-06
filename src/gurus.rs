use serde::Deserialize;
use std::collections::HashMap;

pub use crate::strnum::FloatOrString;

/// Structure holding basic data for a single Guru.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct Gurus {
    /// Map holding the list of Gurus per country.
    pub all: HashMap<String, Vec<Guru>>,
    /// For each personal list of Gurus (e.g. default and custom list),
    /// a vector of Guru IDs is stored.
    pub my: HashMap<String, Vec<String>>,
}

/// Container for the guru holdings
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GuruTrades {
    /// Array of gurus position in the stock
    holdings: Vec<GuruHoldings>,
    /// Array of recent guru picks in the stock
    picks: Vec<GuruPicks2>,
}

/// Container for the guru holdings
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
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
#[serde(deny_unknown_fields)]
pub struct GuruPicks {
    /// Portfolio of guru picks
    port: Vec<GuruPick>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
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
    pub exchange: String,
    pub industry: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GuruPortfolio {
    pub summary: GuruPortSummary,
    pub port: Vec<GuruPosition>,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GuruPortSummary {
    pub country: String,
    pub date: String,
    pub equity: FloatOrString,
    pub firm: String,
    pub num_new: FloatOrString,
    pub number_of_stocks: FloatOrString,
    pub turnover: FloatOrString,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GuruPosition {
    #[serde(rename = "13f_date")]
    pub date_13f: String,
    #[serde(rename = "52h")]
    pub num_52h: FloatOrString,
    #[serde(rename = "52l")]
    pub num_52l: FloatOrString,
    pub change: FloatOrString,
    pub company: String,
    pub currency: String,
    pub currency_txt: String,
    pub exchange: String,
    pub impact: FloatOrString,
    pub industry: String,
    pub mktcap: FloatOrString,
    pub pct: FloatOrString,
    pub pe: FloatOrString,
    pub position: FloatOrString,
    pub price: FloatOrString,
    pub sector: String,
    pub share: FloatOrString,
    pub symbol: String,
    pub symbol_ori: String,
    pub value: FloatOrString,
    #[serde(rename = "yield")]
    pub transaction_yield: FloatOrString,
}
