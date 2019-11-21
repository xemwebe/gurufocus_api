use gurufocus_api as gfapi;
use std::env;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    let stock = "NYSE:BAC";
    let stock_summary = gf_connect.get_stock_summary(stock).unwrap();

    let stock_summary: gfapi::StockSummary = serde_json::from_value(stock_summary).unwrap();
    println!("Stock summary for Bank of America\n{:#?}", stock_summary.summary);
}
