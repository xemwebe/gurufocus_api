use gurufocus_api as gfapi;
use std::env;

/// Display annual development of enterprice value of given stock
async fn show_financials(ticker: &str, gf_connect: &gfapi::GuruFocusConnector) {
    let financials = gf_connect.get_financials(ticker).await.unwrap();
    let financials: gfapi::FinancialData = serde_json::from_value(financials).unwrap();
    println!(
        "Financial figures of {}'s Enterprice value\n==================\n{:#?}\n\n",
        ticker, financials
    );
}

#[tokio::main]
async fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);
    // non-financial
    show_financials("AMZN", &gf_connect).await;
    // bank
    show_financials("NYSE:JPM", &gf_connect).await;
    // insurance
    show_financials("AIG", &gf_connect).await;
    // REIT
    show_financials("GOOD", &gf_connect).await;
}
