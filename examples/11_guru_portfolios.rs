use gurufocus_api as gfapi;
use std::collections::HashMap;
use serde_json::Value;
use std::env;

type GuruPortfolios = HashMap<String, HashMap<String, Value>>;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    // Bill Ackman and David Einhorn
    let gurus = ["47","39"];
    let portfolios = gf_connect.get_guru_portfolios(&gurus).unwrap();

    let portfolios: GuruPortfolios = serde_json::from_value(portfolios).unwrap();
    println!(
        "Aggregated Portfolios of Bill Ackman and David Einhorn\n{:#?}", portfolios);
}
