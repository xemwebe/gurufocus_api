use gurufocus_api as gfapi;
use std::env;

type DividendHistory = Vec<gfapi::Dividend>;

#[tokio::main]
async fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);

    let stock = "NAS:MSFT";
    let dividends = gf_connect.get_dividend_history(stock).await.unwrap();

    let dividends: DividendHistory = serde_json::from_value(dividends).unwrap();
    println!("Microsoft's dividend history\n");
    for div in dividends {
        println!("{:?}", div);
    }
}
