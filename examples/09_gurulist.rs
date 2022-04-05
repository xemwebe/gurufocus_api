use gurufocus_api as gfapi;
use std::env;

#[tokio::main]
async fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);
    let gurus = gf_connect.get_gurus().await.unwrap();

    // Print list of gurus
    println!("List of all Gurus: {}", gurus);
}
