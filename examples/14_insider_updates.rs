use gurufocus_api as gfapi;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

type InsiderUpdaters = Vec<HashMap<String, Value>>;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    let updates = gf_connect.get_insider_updates().unwrap();

    let updates: InsiderUpdaters = serde_json::from_value(updates).unwrap();
    println!("List of lasted insider updates\n{:#?}", updates);
}
