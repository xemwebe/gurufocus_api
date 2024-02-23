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
    pub holdings: Vec<GuruHoldings>,
    /// Array of recent guru picks in the stock
    pub picks: Vec<GuruPicks2>,
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

// #[derive(Deserialize, Debug)]
// #[serde(deny_unknown_fields)]
// pub struct GuruPicks {
//     /// Portfolio of guru picks
//     pub picks: Vec<GuruPick>,
// }

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

/// Politicians
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Politician {
    /// Unique identifier of politician
    pub id: u32,
    /// Full name of politician
    pub full_name: String,
    /// Politicians current position
    pub position: String,
    /// The political party the politician is a member of
    pub party: String,
    /// Politicians election district
    pub district: Option<String>,
    /// Politicians state
    pub state: String,
}

/// Politicians transactions
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PoliticianTransaction {
    pub symbol: String,
    pub company: String,
    pub exchange: String,
    pub industry: usize,
    pub class: AssetType,
    pub stockid: String,
    pub option_type: Option<String>,
    pub strike_price: Option<FloatOrString>,
    pub trans_type: String,
    pub amount: String,
    pub disclosure_date: String,
    pub transaction_date: String,
    pub expiration_date: Option<String>,
    pub id: u32,
    pub full_name: String,
    pub official_full: Option<String>,
    pub position: String,
    pub state: String,
    pub party: String,
}

/// Asset type traded by politicians
#[derive(Deserialize, Debug)]
pub enum AssetType {
    #[serde(rename = "Common Stock")]
    CommonStock,
    Option,
    #[serde(rename = "ETF")]
    Etf,
    #[serde(rename = "Preferred Stock")]
    PreferredStock,
    Bond,
    Units,
    Warrant,
    Other,
}

/// List of politician transactions
#[derive(Deserialize, Debug)]
pub struct PoliticianTransactionList {
    pub count: usize,
    #[serde(rename = "currentPage")]
    pub current_page: u32,
    #[serde(rename = "lastPage")]
    pub last_page: u32,
    pub total: u32,
    pub data: Vec<PoliticianTransaction>,
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use crate::serde_json::Error;
    use chrono::{Datelike, NaiveDate, Utc};
    use std::env;

    #[tokio::test]
    async fn test_guru_trades() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let stock = "WMT";
                let trades = gf_connect.get_guru_trades(stock).await;
                assert!(trades.is_ok());
                let trades = serde_json::from_value::<HashMap<String, GuruTrades>>(trades.unwrap());
                assert!(trades.is_ok());
            }
        }
    }

    fn get_days_from_month(year: i32, month: u32) -> u32 {
        NaiveDate::from_ymd_opt(
            match month {
                12 => year + 1,
                _ => year,
            },
            match month {
                12 => 1,
                _ => month + 1,
            },
            1,
        )
        .unwrap()
        .signed_duration_since(NaiveDate::from_ymd_opt(year, month, 1).unwrap())
        .num_days() as u32
    }

    fn month_before(date: NaiveDate, period: u32) -> NaiveDate {
        let mut day = date.day();
        let mut month = date.month();
        let mut year = date.year();
        if month <= period {
            year -= 1;
            month += 12 - period;
        } else {
            month -= period;
        }

        if day > 28 {
            let last_date_of_month = get_days_from_month(year, month);
            day = std::cmp::max(day, last_date_of_month);
        }
        NaiveDate::from_ymd_opt(year, month, day).unwrap()
    }

    #[tokio::test]
    async fn test_guru_picks() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                // Buffett, Soros and Klarman
                let gurus = ["7", "16", "28"];
                let now = Utc::now().naive_local().date();
                let three_months_ago = month_before(now, 3);
                let page = 1;
                let trades = gf_connect
                    .get_guru_picks(&gurus, three_months_ago, page)
                    .await;
                assert!(trades.is_ok());
            }
        }
    }

    #[tokio::test]
    async fn test_gurulist() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let guru_data = gf_connect.get_gurus().await;
                assert!(guru_data.is_ok());
            }
        }
    }

    #[tokio::test]
    async fn test_guru_portfolios() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                // Bill Ackman and David Einhorn
                let gurus = ["47", "39"];
                let portfolios = gf_connect.get_guru_portfolios(&gurus).await;
                assert!(portfolios.is_ok());
                let portfolios =
                    serde_json::from_value::<HashMap<String, GuruPortfolio>>(portfolios.unwrap());
                assert!(portfolios.is_ok());
            }
        }
    }

    #[tokio::test]
    async fn test_politicianlist() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let politicians = gf_connect.get_politicians().await;
                assert!(politicians.is_ok());
            }
        }
    }

    #[tokio::test]
    async fn test_politiciantransactions() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let politician_transactions = gf_connect.get_politician_transactions(1, None).await;
                assert!(politician_transactions.is_ok());
                let politician_transactions: Result<PoliticianTransactionList, Error> =
                    serde_json::from_value(politician_transactions.unwrap());
                assert!(politician_transactions.is_ok());
            }
        }
    }
}
