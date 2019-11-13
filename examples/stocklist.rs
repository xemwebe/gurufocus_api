use std::env;
use gurufocus_api as gfapi;

type StockList = Vec<gfapi::Stock>;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect =  gfapi::GuruFocusConnector::new(token);
    // Get all stocks listed at the Oslo stock exchange (OSL) in Norway
    let exchange = "OSL";
    let stocks = gf_connect.get_listed_stocks(exchange).unwrap();
    let stocks: StockList = serde_json::from_value(stocks).unwrap();
    println!("{:#?}", stocks);
}