use gurufocus_api as gfapi;
use std::env;

/// Display annual development of enterprice value of given stock
async fn enterprice_value_development(ticker: &str, gf_connect: &gfapi::GuruFocusConnector) {
    let financials = gf_connect.get_financials(ticker).await.unwrap();
    let financials: gfapi::FinancialData = serde_json::from_value(financials).unwrap();
    println!("Annual development of Amazon's Enterprice value\nFY\tEV\n==================");
    let periods = &financials.financials.annuals.fiscal_year;
    let vq = &financials.financials.annuals.valuation_and_quality;
    for (i, p) in periods.iter().enumerate() {
        println!("{}\t{}", p, vq.enterprice_value[i]);
    }
}

#[tokio::main]
async fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);
    // non-financial
    enterprice_value_development("AMZN", &gf_connect);
    // bank
    enterprice_value_development("NYSE:JPM", &gf_connect);
    // insurance
    enterprice_value_development("AIG", &gf_connect);
    // REIT
    enterprice_value_development("GOOD", &gf_connect);
}
