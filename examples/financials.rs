use gurufocus_api as gfapi;
use std::env;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);
    let financials = gf_connect.get_financials("AMZN").unwrap();

    let financials: gfapi::FinancialData = serde_json::from_value(financials).unwrap();
    println!("Financial data of Amazon:");
    println!("{:?}", financials);
}
