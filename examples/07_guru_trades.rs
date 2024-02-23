use gurufocus_api as gfapi;
use std::collections::HashMap;
use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    let stock = "WMT";
    let trades = gf_connect.get_guru_trades(stock).await.unwrap();

    let trades: HashMap<String, gfapi::gurus::GuruTrades> = serde_json::from_value(trades).unwrap();
    println!("List of real time guru trades in Walmart\n{:#?}", trades);
}
