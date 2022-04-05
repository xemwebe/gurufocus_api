use serde::Deserialize;
use serde_json::Value;

pub use crate::strnum::FloatOrString;

/// Structure holding all key ratios for a single stock.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct KeyRatios {
    #[serde(rename = "Basic")]
    pub basic: Basic,
    #[serde(rename = "Fundamental")]
    pub fundamental: Value,
    #[serde(rename = "Valuation Ratio")]
    pub valuation_ratio: Value,
    #[serde(rename = "Profitability")]
    pub profitability: Value,
    #[serde(rename = "Growth")]
    pub growth: Value,
    #[serde(rename = "Price")]
    pub price: Value,
    #[serde(rename = "Dividends")]
    pub dividends: Value,
    #[serde(rename = "Income Statement")]
    pub income_statement: IncomeStatement,
    #[serde(rename = "Valuation")]
    pub valuation: Valuation,
    #[serde(rename = "Quality")]
    pub quality: Quality,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct IncomeStatement {
    #[serde(rename = "Selling, General, & Admin. Expense")]
    pub selling_general_and_admin_expense: FloatOrString,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Basic {
    #[serde(rename = "Price Updated Time")]
    pub price_updated_time: String,
    #[serde(rename = "Company")]
    pub company: String,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Valuation {
    #[serde(rename = "Earnings Power Value (EPV)")]
    pub epv: FloatOrString,
}

#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct Quality {
    #[serde(rename = "Predictability Rank")]
    pub predictability_rank: FloatOrString,
}

/// Container for analyst estimates for all periods
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AnalystEstimates {
    pub annual: AnnualAnalystEstimate,
    pub quarter: QuarterlyAnalystEstimate,
}

/// Container for analyst estimates for annual periods
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct AnnualAnalystEstimate {
    pub long_term_growth_rate_mean: FloatOrString,
    pub long_term_revenue_growth_rate_mean: FloatOrString,
    pub date: Vec<String>,
    pub revenue_estimate: Vec<FloatOrString>,
    pub eps_nri_estimate: Vec<FloatOrString>,
    pub per_share_eps_estimate: Vec<FloatOrString>,
    pub ebit_estimate: Vec<FloatOrString>,
    pub ebitda_estimate: Vec<FloatOrString>,
    pub dividend_estimate: Vec<FloatOrString>,
}

/// Container for analyst estimates for quarterly periods
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct QuarterlyAnalystEstimate {
    pub long_term_growth_rate_mean: FloatOrString,
    pub date: Vec<String>,
    pub revenue_estimate: Vec<FloatOrString>,
    pub eps_nri_estimate: Vec<FloatOrString>,
    pub per_share_eps_estimate: Vec<FloatOrString>,
    pub ebit_estimate: Vec<FloatOrString>,
    pub ebitda_estimate: Vec<FloatOrString>,
    pub dividend_estimate: Vec<FloatOrString>,
    pub pettm_estimate: Vec<FloatOrString>,
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_key_ratios() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);

                // Get key ratios of Berkshire Hathaway
                let stock = "NYSE:BRK.A";
                let key_ratios_json = gf_connect.get_key_ratios(stock).await;
                assert!(key_ratios_json.is_ok());

                let key_ratios = serde_json::from_value::<KeyRatios>(key_ratios_json.unwrap());
                assert!(key_ratios.is_ok());
            }
        }
    }

    #[tokio::test]
    async fn test_analyst_estimates() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);

                let stock = "NAS:CSCO";
                let estimates_json = gf_connect.get_analyst_estimate(stock).await;
                assert!(estimates_json.is_ok());

                let estimates = serde_json::from_value::<AnalystEstimates>(estimates_json.unwrap());
                println!("{:?}", estimates);
                assert!(estimates.is_ok());
            }
        }
    }

}


