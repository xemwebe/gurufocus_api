use gurufocus_api as gfapi;
use std::env;

type PersonalPortfolios = Vec<gfapi::Portfolio>;

#[tokio::main]
async fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);
    let portfolios = gf_connect.get_personal_portfolio().await.unwrap();

    let portfolios: PersonalPortfolios = serde_json::from_value(portfolios).unwrap();
    println!("Personal portfolios overview\n{:#?}", portfolios);
}
