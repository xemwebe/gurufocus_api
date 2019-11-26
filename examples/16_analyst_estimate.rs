use gurufocus_api as gfapi;
use std::env;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    let stock = "NAS:CSCO";
    let estimates = gf_connect.get_analyst_estimate(stock).unwrap();

    let estimates: gfapi::AnalystEstimates = serde_json::from_value(estimates).unwrap();
    println!("Analyst estimates on Cisco\n{:#?}", estimates);
}
