use gurufocus_api as gfapi;
use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    // Get key ratios of Berkshire Hathaway
    let stock = "NYSE:BRK.A";
    let key_ratios = gf_connect.get_key_ratios(stock).await.unwrap();

    let key_ratios: gfapi::KeyRatios = serde_json::from_value(key_ratios).unwrap();
    println!(
        "List of key ratios for Berkshire Hathaway\n{:#?}",
        key_ratios
    );
}
