use gurufocus_api as gfapi;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

type StockSummary = HashMap<String, HashMap<String, Value>>;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    let stock = "NYSE:BAC";
    let stock_summary = gf_connect.get_stock_summary(stock).unwrap();

    let stock_summary: StockSummary = serde_json::from_value(stock_summary).unwrap();
    println!("Stock summary for Bank of America\n{:#?}", stock_summary);
}
