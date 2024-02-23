use gurufocus_api as gfapi;
use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);
    let page = 1;
    let asset_type = None;

    let transactions = gf_connect
        .get_politician_transactions(page, asset_type)
        .await
        .unwrap();
    let transactions: gfapi::gurus::PoliticianTransactionList =
        serde_json::from_value(transactions).unwrap();

    println!("List of politician transactions");
    for t in transactions.data {
        println!("{t:?}");
    }
}
