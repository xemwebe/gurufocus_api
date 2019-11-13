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
//! Please note that the GuruFocus API provides sometimes numerical data as strings rather 
//! than numbers. Currently, these strings will not be transformed into numbers, though this is 
//! planned for a future release. Therefore, although the basic API wrapper will not changes, the
//! custom types like `Guru` are subject to such changes.


extern crate chrono;
extern crate reqwest;
extern crate serde_json;

use reqwest::StatusCode;
use serde_json::Value;

/// Special types for dealing with Gurus.
pub mod gurus;
pub use gurus::*;

/// Special types for dealing with Stocks.
pub mod stock;
pub use stock::*;

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
    pub fn get_financials(&self, stock: &str) -> Result<Value, String> {
        let args = format!("stock/{}/financials", stock);
        self.send_request(args.as_str())
    }

    /// Returns the current key statistic figures for stock symbol given as argument
    pub fn get_key_ratios(&self, stock: &str) -> Result<Value, String> {
        let args = format!("stock/{}/keyratios", stock);
        self.send_request(args.as_str())
    }

    /// Returns the current quote data of a comma separated list of symbols given as argument
    pub fn get_quotes(&self, stocks: &str) -> Result<Value, String> {
        let args = format!("stock/{}/quote", stocks);
        self.send_request(args.as_str())
    }

    /// Returns the history of (adjusted) quoted prices for symbol given as argument
    pub fn get_price_hist(&self, stock: &str) -> Result<Value, String> {
        let args = format!("stock/{}/price", stock);
        self.send_request(args.as_str())
    }

    /// Returns the history of (unadjusted) quoted prices for symbol given as argument
    pub fn get_unadj_price_hist(&self, stock: &str) -> Result<Value, String> {
        let args = format!("stock/{}/unadjusted_price", stock);
        self.send_request(args.as_str())
    }

    /// Returns companies current price, valuation rations and ranks for symbol given as argument
    pub fn get_stock_summary(&self, stock: &str) -> Result<Value, String> {
        let args = format!("stock/{}/summary", stock);
        self.send_request(args.as_str())
    }

    /// Returns real-time guru trades and holding data for symbol given as argument
    pub fn get_guru_trades(&self, stock: &str) -> Result<Value, String> {
        let args = format!("stock/{}/gurus", stock);
        self.send_request(args.as_str())
    }

    /// Returns lists of all and personalized gurus
    pub fn get_gurus(&self) -> Result<Value, String> {
        self.send_request("gurulist")
    }

    /// Returns list of gurus stock picks using list of guru ids since a given start date. 
    pub fn get_guru_picks(&self, gurus: &[&str], start_date: chrono::DateTime<chrono::Utc>) -> Result<Value, String> {
        let args = format!("guru/{}/picks/{}", GuruFocusConnector::compact_list(&gurus), start_date.format("%F"));
        self.send_request(args.as_str())
    }

    /// Returns list of aggregated guru portfolios given a slice of guru ids 
    pub fn get_guru_portfolios(&self, gurus: &[&str]) -> Result<Value, String> {
        let args = format!("guru/{}/aggregated", GuruFocusConnector::compact_list(&gurus));
        self.send_request(args.as_str())
    }

    /// Returns list of supported exchanges 
    pub fn get_exchanges(&self) -> Result<Value, String> {
        self.send_request("exchange_list")
    }

    /// Returns list of all stocks of a particular exchange
    pub fn get_listed_stocks(&self, exchange: &str) -> Result<Value, String> {
        let args = format!("exchange_stocks/{}", exchange);
        self.send_request(args.as_str())
    }

    /// Returns list of latest insider trades ordered by insider transctions time
    pub fn get_insider_trades(&self) -> Result<Value, String> {
        self.send_request("insider_updates")
    }

    /// Returns 30 years dividend history data of a stock
    pub fn get_dividend_history(&self, stock: &str) -> Result<Value, String> {
        let args = format!("stock/{}/dividend", stock);
        self.send_request(args.as_str())
    }

    /// Returns analyst estimate data of a stock
    pub fn get_analyst_estimate(&self, stock: &str) -> Result<Value, String> {
        let args = format!("stock/{}/analyst_estimate ", stock);
        self.send_request(args.as_str())
    }

    /// Returns list of personal portfolios
    pub fn get_personal_portfolio(&self) -> Result<Value, String> {
        self.send_request("portfolio/my_portfolios")
    }

    /// Returns list of all stocks with updated fundamental data within a week of the given date
    pub fn get_updated_stocks(&self, date: chrono::DateTime<chrono::Utc>) -> Result<Value, String> {
        let args = format!("funda_updated/{}", date);
        self.send_request(args.as_str())
    }

    /// Send request to gurufocus server and transform response to JSON value
    fn send_request(&self, args: &str) -> Result<Value, String> {
        let url: String = format!("{}{}/{}", self.url, self.user_token, args);
        let resp = reqwest::get(url.as_str());
        if resp.is_err() {
            return Err(String::from("Connection to server failed."));
        }
        let mut resp = resp.unwrap();
        match resp.status() {
            StatusCode::OK => match resp.json() {
                Ok(json) => Ok(json),
                err => Err(format!("Parsing json failed: {:?}", err)),
            },
            err => Err(format!("Received bad response from server: {}", err)),
        }
    }

    /// Compact list as input to url
    fn compact_list(a: &[&str]) -> String {
        if a.len() == 0 { 
            return String::new();
        }
        let mut it = a.iter();
        let mut res = format!("{}", it.next().unwrap());
        for n in it {
            res.push_str(&format!(",{}", n));
        }
        res        
    }

}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compact_list() {
        assert_eq!(GuruFocusConnector::compact_list(&["1","2","3"]), "1,2,3");
        assert_eq!(GuruFocusConnector::compact_list(&[]), "");
        assert_eq!(GuruFocusConnector::compact_list(&["3"]), "3");
    }
}
