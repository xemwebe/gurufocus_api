use gurufocus_api as gfapi;
use std::env;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);
    let guru_data = gf_connect.get_gurus().unwrap();

    let gurus: gfapi::Gurus = serde_json::from_value(guru_data).unwrap();
    // Print list of gurus
    println!("List of all Gurus\nID\tName");
    for country in gurus.all {
        for guru in country.1 {
            println!("{}\t{}", guru.id, guru.name);
        }
    }
}
