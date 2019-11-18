use gurufocus_api as gfapi;
use std::collections::HashMap;
use serde_json::Value;
use std::env;

type AnalystEstimates = HashMap<String, HashMap<String, Value>>;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    let stock = "NAS:CSCO";
    let estimates = gf_connect.get_analyst_estimate(stock).unwrap();

    let estimates: AnalystEstimates = serde_json::from_value(estimates).unwrap();
    println!(
        "Analyst estimates on Cisco\n{:#?}", estimates);
}
