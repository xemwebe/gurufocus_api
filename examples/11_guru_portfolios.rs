use gurufocus_api as gfapi;
use std::collections::HashMap;
use std::env;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    // Bill Ackman and David Einhorn
    let gurus = ["47", "39"];
    let portfolios = gf_connect.get_guru_portfolios(&gurus).unwrap();

    let portfolios: HashMap<String, gfapi::GuruPortfolio> = serde_json::from_value(portfolios).unwrap();
    println!(
        "Aggregated Portfolios of Bill Ackman and David Einhorn\n{:#?}",
        portfolios
    );
}
