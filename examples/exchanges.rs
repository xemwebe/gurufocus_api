use std::env;
use gurufocus_api as gfapi;
use std::collections::HashMap;

type ExchangeList = HashMap<String, Vec<String> >;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect =  gfapi::GuruFocusConnector::new(token);
    let exchanges = gf_connect.get_exchanges().unwrap();
    let exchange_map: ExchangeList = serde_json::from_value(exchanges).unwrap();
    println!("{:#?}", exchange_map);
}