use gurufocus_api as gfapi;
use std::env;

fn main() {
    let token = env::var("GURUFOCUS_TOKEN").unwrap();
    let gf_connect = gfapi::GuruFocusConnector::new(token);
    let financials = gf_connect.get_financials("AMZN").unwrap();

    let financials: gfapi::FinancialData = serde_json::from_value(financials).unwrap();
    println!("Annual development of Amazon's Enterprice value\nFY\tEV\n==================");
    let periods = &financials.financials.annuals.fiscal_year;
    let vq = &financials.financials.annuals.valuation_and_quality;
    for (i, p) in periods.iter().enumerate() {
        println!("{}\t{}", p, vq.enterprice_value[i]);
    }
}
