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
    pub income_statement: Option<IncomeStatement>,
    #[serde(rename = "Valuation")]
    pub valuation: Option<Valuation>,
    #[serde(rename = "Quality")]
    pub quality: Option<Quality>,
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
    pub annual: PeriodAnalystEstimate,
    pub quarterly: PeriodAnalystEstimate,
}

/// Container for analyst estimates for annual periods
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PeriodAnalystEstimate {
    pub long_term_growth_rate_mean: FloatOrString,
    pub long_term_revenue_growth_rate_mean: FloatOrString,
    pub date: Vec<String>,
    pub revenue_estimate: Vec<FloatOrString>,
    pub eps_nri_estimate: Vec<FloatOrString>,
    pub per_share_eps_estimate: Vec<FloatOrString>,
    pub ebit_estimate: Vec<FloatOrString>,
    pub ebitda_estimate: Vec<FloatOrString>,
    pub dividend_estimate: Vec<FloatOrString>,
    pub gross_margin_estimate: Vec<FloatOrString>,
    pub net_income_estimate: Vec<FloatOrString>,
    pub pettm_estimate: Vec<FloatOrString>,
    pub pretax_income_estimate: Vec<FloatOrString>,
    pub roa_estimate: Vec<FloatOrString>,
    pub roe_estimate: Vec<FloatOrString>,
    pub operating_cash_flow_per_share_estimate: Vec<FloatOrString>,
    pub book_value_per_share_estimate: Option<Vec<FloatOrString>>,
    pub future_per_share_eps_estimate_growth: Option<FloatOrString>,
    pub future_eps_nri_estimate_growth: Option<FloatOrString>,
    pub future_revenue_estimate_growth: Option<FloatOrString>,
    pub future_ebit_estimate_growth: Option<FloatOrString>,
    pub future_ebitda_estimate_growth: Option<FloatOrString>,
    pub future_dividend_estimate_growth: Option<FloatOrString>,
    pub future_net_income_estimate_growth: Option<FloatOrString>,
    pub future_pretax_income_estimate_growth: Option<FloatOrString>,
    pub future_book_value_per_share_estimate_growth: Option<FloatOrString>,
    pub future_roa_estimate_growth: Option<FloatOrString>,
    pub future_roe_estimate_growth: Option<FloatOrString>,
    pub future_gross_margin_estimate_growth: Option<FloatOrString>,
    pub future_operating_cash_flow_per_share_estimate_growth: Option<FloatOrString>,
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
                assert!(estimates.is_ok());
            }
        }
    }
}
