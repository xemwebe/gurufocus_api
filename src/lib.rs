//! # GuruFocus API
//!
//! This project provides a set of functions to receive data from the
//! the guru focus website via the [GuruFocus API](https://www.gurufocus.com/api.php).
//!
//! # Usage
//! Please note that you need at least a premium account to use this API. There a couple of
//! examples demonstrating how to use the API in your own rust projects. To run this example,
//! you first need to define an environment variable holding the user Token you got from
//! GuruFocus:
//! ```bash
//! export GURUFOCUS_TOKEN='<your user token>'
//! ```
//!
//! The examples can be executed via the command
//! ```dummy
//! cargo test --example <name of example>
//! ```
//! Here, `<name of example>` could be the name of any of the files in the examples folder
//! without the `.rs` extension
//! Please note that running any of the examples increases your API access counter by at least 1.
//!
//! The GuruFocus API provides all data in JSON format, and the basic API functions currently
//! will just return these JSON structures as `serde_json::Value` types without any further
//! processing. The `serde_json::Value` types can be deserialized
//! into more meaningful data structures, as is demonstrated in the `gurulist` example.
//!
//! The GuruFocus API returns numbers sometimes as numbers, sometimes as strings. This is dealt
//! with by introducing a new struct `FloatOrString` containing a float value, but which can
//! be read from either a string or float automatically. The drawback is that `.0` as to be
//! added to the variable name of a specific data structure. I.e., to access the quoted price
//! in a variable of type Quote, i.e. `q: Quote`, the price can be accessed via `q.price.0` instead
//! of `q.price`. In a few cases, the string contains not a number, but an error message, like
//! "Negative Tangible Equity". In such cases, if the string can not be parsed to a number, the
//! value is set to `NAN`.
//!
//! Since version 0.4, all requests using the ```async``` attribute, returning a Future instead of
//! waiting for the response and returning the result. To get the actual results, ```.await``` or ```block_on```
//! or something similar needs to be used. The examples demonstrate how the library could be used.
//!
//! Please note that the library is not yet stable and that the user interface is still subject to change.
//! However, feedback regarding the usability and suggestions for improving the interface are welcome.

use serde_json::{self, Value};
use tokio_compat_02::FutureExt;
use thiserror::Error;

/// Special types for dealing with Gurus.
pub mod gurus;
pub use gurus::*;

/// Special types for dealing with stocks.
pub mod stock;
pub use stock::*;

/// Special types for dealing with financial data.
pub mod financials;
pub use financials::*;

/// Special types for key ratios.
pub mod keyratios;
pub use keyratios::*;

/// Special types for insider tradingey ratios.
pub mod insiders;
pub use insiders::*;

/// Special types for user portfolio.
pub mod portfolio;
pub use portfolio::*;

/// Module for special string / number derserializer
pub mod strnum;

/// Module for special hex num derserializer
pub mod hexnum;

#[derive(Error, Debug)]
pub enum GuruFocusError {
    #[error("Request failure")]
    RequestFailure(#[from] reqwest::Error),
}

/// Container for connection parameters to gurufocus server.
pub struct GuruFocusConnector {
    url: &'static str,
    user_token: String,
}

impl GuruFocusConnector {
    /// Constructor for a new instance of GuruFocusConnector.
    /// token is the user token you get from gurufocus if you subscribe for
    /// a premium or premium plus account.
    pub fn new(token: String) -> GuruFocusConnector {
        GuruFocusConnector {
            url: "https://api.gurufocus.com/public/user/",
            user_token: token,
        }
    }

    /// Returns the full history of financial data for stock symbol given as argument
    pub async fn get_financials(&self, stock: &str) -> Result<Value, GuruFocusError> {
        let args = format!("stock/{}/financials", stock);
        self.send_request(args.as_str()).await
    }

    /// Returns the current key statistic figures for stock symbol given as argument
    pub async fn get_key_ratios(&self, stock: &str) -> Result<Value, GuruFocusError> {
        let args = format!("stock/{}/keyratios", stock);
        self.send_request(args.as_str()).await
    }

    /// Returns the current quote data of a comma separated list of symbols given as argument
    pub async fn get_quotes(&self, stocks: &[&str]) -> Result<Value, GuruFocusError> {
        let args = format!("stock/{}/quote", compact_list(stocks));
        self.send_request(args.as_str()).await
    }

    /// Returns the history of (adjusted) quoted prices for symbol given as argument
    pub async fn get_price_hist(&self, stock: &str) -> Result<Value, GuruFocusError> {
        let args = format!("stock/{}/price", stock);
        self.send_request(args.as_str()).await
    }

    /// Returns the history of (unadjusted) quoted prices for symbol given as argument
    pub async fn get_unadj_price_hist(&self, stock: &str) -> Result<Value, GuruFocusError> {
        let args = format!("stock/{}/unadjusted_price", stock);
        self.send_request(args.as_str()).await
    }

    /// Returns companies current price, valuation rations and ranks for symbol given as argument
    pub async fn get_stock_summary(&self, stock: &str) -> Result<Value, GuruFocusError> {
        let args = format!("stock/{}/summary", stock);
        self.send_request(args.as_str()).await
    }

    /// Returns real-time guru trades and holding data for symbol given as argument
    pub async fn get_guru_trades(&self, stock: &str) -> Result<Value, GuruFocusError> {
        let args = format!("stock/{}/gurus", stock);
        self.send_request(args.as_str()).await
    }

    /// Returns real-time insider trades for symbol given as argument
    pub async fn get_insider_trades(&self, stock: &str) -> Result<Value, GuruFocusError> {
        let args = format!("stock/{}/insider", stock);
        self.send_request(args.as_str()).await
    }
    /// Returns lists of all and personalized gurus
    pub async fn get_gurus(&self) -> Result<Value, GuruFocusError> {
        self.send_request("gurulist").await
    }

    /// Returns list of gurus stock picks using list of guru ids since a given start date.
    pub async fn get_guru_picks(
        &self,
        gurus: &[&str],
        start_date: chrono::NaiveDate,
        page: i32,
    ) -> Result<Value, GuruFocusError> {
        let args = format!(
            "guru/{}/picks/{}/{}",
            compact_list(gurus),
            start_date.format("%F"),
            page
        );
        self.send_request(args.as_str()).await
    }

    /// Returns list of aggregated guru portfolios given a slice of guru ids
    pub async fn get_guru_portfolios(&self, gurus: &[&str]) -> Result<Value, GuruFocusError> {
        let args = format!("guru/{}/aggregated", compact_list(gurus));
        self.send_request(args.as_str()).await
    }

    /// Returns list of supported exchanges
    pub async fn get_exchanges(&self) -> Result<Value, GuruFocusError> {
        self.send_request("exchange_list").await
    }

    /// Returns list of all stocks of a particular exchange
    pub async fn get_listed_stocks(&self, exchange: &str) -> Result<Value, GuruFocusError> {
        let args = format!("exchange_stocks/{}", exchange);
        self.send_request(args.as_str()).await
    }

    /// Returns list of latest insider trades ordered by insider transctions time
    pub async fn get_insider_updates(&self) -> Result<Value, GuruFocusError> {
        self.send_request("insider_updates").await
    }

    /// Returns 30 years dividend history data of a stock
    pub async fn get_dividend_history(&self, stock: &str) -> Result<Value, GuruFocusError> {
        let args = format!("stock/{}/dividend", stock);
        self.send_request(args.as_str()).await
    }

    /// Returns analyst estimate data of a stock
    pub async fn get_analyst_estimate(&self, stock: &str) -> Result<Value, GuruFocusError> {
        let args = format!("stock/{}/analyst_estimate ", stock);
        self.send_request(args.as_str()).await
    }

    /// Returns list of personal portfolios
    pub async fn get_personal_portfolio(&self) -> Result<Value, GuruFocusError> {
        self.send_request("portfolio/my_portfolios").await
    }

    /// Returns list of all stocks with updated fundamental data within a week of the given date
    pub async fn get_updated_stocks(&self, date: chrono::NaiveDate) -> Result<Value, GuruFocusError> {
        let args = format!("funda_updated/{}", date);
        self.send_request(args.as_str()).await
    }

    /// Send request to gurufocus server and transform response to JSON value
    async fn send_request(&self, args: &str) -> Result<Value, GuruFocusError> {
        let url: String = format!("{}{}/{}", self.url, self.user_token, args);
        let resp = reqwest::get(url.as_str()).compat().await?;
        println!("{:?}", url);
        Ok(resp.json().await?)
    }
}

/// Compact list as input to url
fn compact_list(a: &[&str]) -> String {
    if a.is_empty() {
        return String::new();
    }
    let mut it = a.iter();
    let mut res = it.next().unwrap().to_string();
    for n in it {
        res.push_str(&format!(",{}", n));
    }
    res
}

#[cfg(test)]
mod tests {
    use std::env;
    use std::collections::HashMap;

    use super::*;
    use serde::Deserialize;

    #[test]
    fn test_compact_list() {
        assert_eq!(compact_list(&["1", "2", "3"]), "1,2,3");
        assert_eq!(compact_list(&[]), "");
        assert_eq!(compact_list(&["3"]), "3");
    }

    #[derive(Deserialize, Debug)]
    #[serde(deny_unknown_fields)]
    struct SimpleStruct {
        value: i32,
    }

    #[test]
    fn deserialize_extra_fields() {
        let data = r#"
        {
            "value": 42,
            "text": "bla"
        }"#;

        let s: serde_json::Result<SimpleStruct> = serde_json::from_str(data);
        // Fails if json has extra fields
        assert!(s.is_err());
    }


    #[tokio::test]
    async fn test_exchanges() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let exchanges = gf_connect.get_exchanges().await;
                assert!(exchanges.is_ok());
                let exchange_map = serde_json::from_value::<HashMap<String, Vec<String>>>(exchanges.unwrap());
                assert!(exchange_map.is_ok());
            }
        }
    }

}
