use gurufocus_api as gfapi;
use std::collections::HashMap;
use std::env;

type ExchangeList = HashMap<String, Vec<String>>;

#[tokio::main]
async fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);
    let exchanges = gf_connect.get_exchanges().await.unwrap();
    let exchange_map: ExchangeList = serde_json::from_value(exchanges).unwrap();
    println!("{:#?}", exchange_map);
}
