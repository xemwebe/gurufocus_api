use std::env;
use gurufocus_api as gfapi;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect =  gfapi::GuruFocusConnector::new(token);
    let ticker = ["NAS:AAPL", "FRA:APC", "LTS:0JQ4"];
    let prices = gf_connect.get_quotes(&ticker).unwrap();

    println!("Compare latest quotes of Apple stock prices and three different exchanges:");
    println!("{:#?}", prices);
}