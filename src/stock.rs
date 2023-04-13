pub use crate::hexnum::HexNum;
pub use crate::strnum::FloatOrString;
use serde::Deserialize;
use std::collections::HashMap;

type JsonObject = HashMap<String, serde_json::Value>;

/// Container for basic data for a single stock
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Stock {
    /// Name of the company
    pub company: String,
    /// Currency in which the stock is quoted
    pub currency: String,
    /// ID of the exchange the stock is quoted ad
    pub exchange: String,
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
#[serde(deny_unknown_fields)]
pub struct Quote {
    /// Currency of quoted stock price
    #[serde(rename = "Currency")]
    pub currency: String,
    /// Current day's price change
    #[serde(rename = "Day\'s Change %")]
    pub todays_change: FloatOrString,
    /// Current day's trading volume
    #[serde(rename = "Day\'s Volume")]
    pub todays_volume: FloatOrString,
    /// Exchange's symbol
    #[serde(rename = "Exchange")]
    pub exchange: String,
    /// Current price
    #[serde(rename = "Current Price")]
    pub current_price: FloatOrString,
    /// Last quoted price
    #[serde(rename = "Price")]
    pub price: FloatOrString,
    /// Last price change
    #[serde(rename = "Price Change")]
    pub price_change: FloatOrString,
    /// Time of last price update
    #[serde(rename = "Price Updated Time")]
    pub update_time: String,
    /// Stock ticker symbol
    #[serde(rename = "Symbol")]
    pub symbol: String,
    /// Today's highest price
    pub high: FloatOrString,
    /// Today's lowest price
    pub low: FloatOrString,
    /// Today's opening price
    pub open: FloatOrString,
    /// Timestamp
    pub timestamp: i64,
}

/// Container for summary of basic stock data and valuation ratios
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct StockSummary {
    pub summary: StockSummaryByCat,
}

/// Container for summary of basic stock data by category
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct StockSummaryByCat {
    pub general: GeneralData,
    pub chart: Chart,
    pub ratio: JsonObject,
    pub guru: HashMap<String, GuruTransaction>,
    pub insider: HashMap<String, InsiderTransaction>,
    pub company_data: JsonObject,
    pub estimate: Estimate,
}

/// Container for transaction done by Gurus
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GuruTransaction {
    pub buy: i64,
    pub hold: i64,
    pub new_buy: i64,
    pub sell: i64,
    pub sold_out: i64,
}

/// Container for transaction done by Insiders
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct InsiderTransaction {
    pub buy: FloatOrString,
    pub sell: Option<FloatOrString>,
    pub price: Option<FloatOrString>,
}

/// Chart
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Chart {
    #[serde(rename = "Graham Number")]
    pub graham_number: FloatOrString,
    #[serde(rename = "Median P/S Value")]
    pub median_ps_value: FloatOrString,
    #[serde(rename = "Peter Lynch Value")]
    pub peter_lynch_value: FloatOrString,
    #[serde(rename = "Net Current Asset Value")]
    pub net_current_asset_value: FloatOrString,
    #[serde(rename = "Net-Net Working Capital")]
    pub net_net_working_capital: FloatOrString,
    #[serde(rename = "Projected FCF")]
    pub projected_fcf: FloatOrString,
    #[serde(rename = "Tangible Book")]
    pub tangible_book: FloatOrString,
    #[serde(rename = "DCF (Earnings Based)")]
    pub dcf_earnings_based: FloatOrString,
    #[serde(rename = "DCF (FCF Based)")]
    pub dcf_fcf_based: FloatOrString,
    #[serde(rename = "GF Value")]
    pub gf_balue: FloatOrString,
    #[serde(rename = "Earnings Power Value")]
    pub earnings_power_value: FloatOrString,
}

/// Estimate summary
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Estimate {
    #[serde(rename = "LongTermGrowthRateMean")]
    pub longtermgrowthratemean: FloatOrString,
    #[serde(rename = "eps_nri")]
    pub eps_nri: Vec<FloatOrString>,
    #[serde(rename = "count")]
    pub count: FloatOrString,
    #[serde(rename = "per share eps")]
    pub per_share_eps: Vec<FloatOrString>,
    #[serde(rename = "percentage")]
    pub percentage: FloatOrString,
    #[serde(rename = "Dividends Per Share")]
    pub dividends_per_share: Vec<FloatOrString>,
    #[serde(rename = "Revenue")]
    pub revenue: Vec<FloatOrString>,
    #[serde(rename = "quarter")]
    pub quarter: Vec<FloatOrString>,
    #[serde(rename = "LongTermRevenueGrowthRateMean")]
    pub longtermrevenuegrowthratemean: FloatOrString,
}

/// General data for summary
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GeneralData {
    pub industry: String,
    pub company: String,
    pub desc: String,
    pub rank_financial_strength: FloatOrString,
    pub sector: String,
    pub currency: String,
    pub price: FloatOrString,
    pub short_desc: String,
    pub rank_profitability: FloatOrString,
    pub rating: FloatOrString,
    pub country: String,
    pub group: String,
    pub timestamp: String,
    pub gf_score: FloatOrString,
    pub rank_gf_value: FloatOrString,
    pub rank_growth: FloatOrString,
    pub rank_momentum: FloatOrString,
    pub risk_assessment: String,
    pub gf_valuation: String,
}

/// Ratio comparison
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RatioCmp {
    pub his: HistoryCmp,
    pub indu: IndustryCmp,
    pub status: FloatOrString,
    pub value: FloatOrString,
}

/// Comparison to historic data
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct HistoryCmp {
    pub high: FloatOrString,
    pub low: FloatOrString,
    #[serde(default)]
    pub med: FloatOrString,
}

/// Comparison to industry
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct IndustryCmp {
    pub global_rank: FloatOrString,
    pub indu_med: FloatOrString,
    pub indu_tot: FloatOrString,
}

/// Description of range of some ratio
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct RatioRange {
    pub color: HexNum,
    pub current: FloatOrString,
    pub high: FloatOrString,
    pub low: FloatOrString,
}

/// Details to a specific warning
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct WarningDetails {
    pub category: Option<String>,
    pub degree: String,
    pub details: String,
    pub display: String,
    pub name: String,
}

/// Details to a specific warning
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct GoodDetails {
    pub category: Option<String>,
    pub details: String,
    pub display: String,
    pub name: String,
}
/// Address and related company information
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CompanyDescription {
    pub address: String,
    pub descrpt: String,
    pub morn_comp_id: Option<String>,
    pub short_descript: String,
    pub symbol: Option<String>,
    pub website: String,
}

/// Country and exchange information
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Country {
    pub country: String,
    pub exchange: String,
    pub symbol: String,
}

/// Stock price change information
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct StockDynamics {
    pub high: FloatOrString,
    pub low: FloatOrString,
    pub open: FloatOrString,
    pub p_change: FloatOrString,
    pub p_pct_change: FloatOrString,
    pub price: FloatOrString,
    pub stockid: Option<String>,
    pub volumn_day: FloatOrString,
}

/// Industry detail information
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct IndustryDetails {
    pub group: String,
    pub groupcode: i64,
    pub industry: String,
    pub industrycode: i64,
    pub sector: String,
    pub sectorcode: i64,
    pub date: String,
}

/// Historic dividend data
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Dividend {
    pub ex_date: String,
    pub record_date: String,
    pub amount: FloatOrString,
    pub pay_date: String,
    pub currency: String,
    #[serde(rename = "type")]
    pub div_type: String,
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use chrono::{Datelike, NaiveDate, Utc};
    use std::env;

    #[tokio::test]
    async fn test_quotes() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let ticker = ["NAS:AAPL", "FRA:APC", "LTS:0JQ4"];
                let prices = gf_connect.get_quotes(&ticker).await;
                assert!(prices.is_ok());
                let prices = serde_json::from_value::<Vec<Quote>>(prices.unwrap());
                assert!(prices.is_ok());
                assert!(prices.unwrap().len() > 0);
            }
        }
    }

    #[tokio::test]
    async fn test_quote_history() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let stock = "NYSE:DIS";
                let prices = gf_connect.get_price_hist(stock).await;
                assert!(prices.is_ok());
                let prices = serde_json::from_value::<Vec<(String, f64)>>(prices.unwrap());
                assert!(prices.is_ok());
                assert!(prices.unwrap().len() > 0);
            }
        }
    }

    #[tokio::test]
    async fn test_quote_history_unadj() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let stock = "NYSE:DIS";
                let prices = gf_connect.get_unadj_price_hist(stock).await;
                assert!(prices.is_ok());
                let prices = serde_json::from_value::<Vec<(String, f64)>>(prices.unwrap());
                assert!(prices.is_ok());
                assert!(prices.unwrap().len() > 0);
            }
        }
    }

    #[tokio::test]
    async fn test_stock_summary() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let stock = "NYSE:BAC";
                let stock_summary_json = gf_connect.get_stock_summary(stock).await;
                assert!(stock_summary_json.is_ok());
                let stock_summary =
                    serde_json::from_value::<StockSummary>(stock_summary_json.unwrap());
                assert!(stock_summary.is_ok());
            }
        }
    }

    #[tokio::test]
    async fn test_stock_list() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                // Get all stocks listed at the Oslo stock exchange (OSL) in Norway
                let exchange = "OSL";
                let stocks = gf_connect.get_listed_stocks(exchange).await;
                assert!(stocks.is_ok());
                let stocks = serde_json::from_value::<Vec<Stock>>(stocks.unwrap());
                assert!(stocks.is_ok());
            }
        }
    }

    #[tokio::test]
    async fn test_dividend_history() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let stock = "NAS:MSFT";
                let dividends = gf_connect.get_dividend_history(stock).await;
                assert!(dividends.is_ok());
                let dividends = serde_json::from_value::<Vec<Dividend>>(dividends.unwrap());
                assert!(dividends.is_ok());
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
        ).unwrap()
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
    async fn test_fundamental_updates() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);

                let now = Utc::now().naive_local().date();
                let one_months_ago = month_before(now, 6);
                let stocks_json = gf_connect.get_updated_stocks(one_months_ago).await;
                assert!(stocks_json.is_ok());
                let stocks = serde_json::from_value::<Vec<String>>(stocks_json.unwrap());
                assert!(stocks.is_ok());
            }
        }
    }
}
