use gurufocus_api as gfapi;
use std::collections::HashMap;
use std::env;

type InsiderTrades = HashMap<String, Vec<gfapi::InsiderTrade>>;

#[tokio::main]
async fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    let stock = "NAS:NVDA";
    let trades = gf_connect.get_insider_trades(stock).await.unwrap();

    let trades: InsiderTrades = serde_json::from_value(trades).unwrap();
    println!("List of real time insider trades in NVDIA\n{:#?}", trades);
}
