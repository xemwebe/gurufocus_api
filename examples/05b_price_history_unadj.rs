use gurufocus_api as gfapi;
use std::env;

type PriceHistory = Vec<(String, f64)>;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    let stock = "NYSE:DIS";
    let prices = gf_connect.get_unadj_price_hist(stock).unwrap();

    let prices: PriceHistory = serde_json::from_value(prices).unwrap();
    println!(
        "Unadjusted Price history for Walt Disney\n{:#?}", prices);
}
