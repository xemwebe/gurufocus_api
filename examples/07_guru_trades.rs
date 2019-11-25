use gurufocus_api as gfapi;
use std::collections::HashMap;
use std::env;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    let stock = "WMT";
    let trades = gf_connect.get_guru_trades(stock).unwrap();

    let trades: HashMap<String, gfapi::GuruTrades> = serde_json::from_value(trades).unwrap();
    println!("List of real time guru trades in Walmart\n{:#?}", trades);
}
