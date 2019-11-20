use gurufocus_api as gfapi;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

type DividendHistory = Vec<HashMap<String, Value>>;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    let stock = "NAS:MSFT";
    let dividends = gf_connect.get_dividend_history(stock).unwrap();

    let dividends: DividendHistory = serde_json::from_value(dividends).unwrap();
    println!("Microsoft's dividend history\n{:#?}", dividends);
}
