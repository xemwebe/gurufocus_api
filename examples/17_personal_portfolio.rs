use gurufocus_api as gfapi;
use serde_json::Value;
use std::collections::HashMap;
use std::env;

type PersonalPortfolios = Vec<HashMap<String, Value>>;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);
    let portfolios = gf_connect.get_personal_portfolio().unwrap();

    let portfolios: PersonalPortfolios = serde_json::from_value(portfolios).unwrap();
    println!("Personal portfolios overview\n{:#?}", portfolios);
}
