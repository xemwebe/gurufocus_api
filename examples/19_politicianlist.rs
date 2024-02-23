use gurufocus_api as gfapi;
use std::env;
use serde_json::Value;

#[tokio::main]
async fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);
    let politicians = gf_connect.get_politicians().await.unwrap();
    let politicians: Vec<gfapi::gurus::Politician> = serde_json::from_value(politicians).unwrap();

    // Print list of politicians
    println!("List of politicians:");
    for p in politicians {
        println!("{}, {}, {}", p.full_name, p.party, p.position);
    }
}
