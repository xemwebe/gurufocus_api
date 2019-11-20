use gurufocus_api as gfapi;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

type InsiderTrades = HashMap<String, HashMap<String, Value>>;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    let stock = "NAS:NVDA";
    let trades = gf_connect.get_guru_trades(stock).unwrap();

    let trades: InsiderTrades = serde_json::from_value(trades).unwrap();
    println!("List of real time insider trades in NVDIA\n{:#?}", trades);
}
