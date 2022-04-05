/// Data containers for financial data
/// The Gurufocus supports three different major templates for Banks, Insurance companies,
/// and non-financial companies. For REITs, some additional fields are delivered.
/// Balance sheet dater differs further with respect to either direct or indirect method.
/// The struct ```FinancialTemplateParameters``` contains information of the type of template served.
/// In general, if a certain field is only available in one or more templates, but not all, it is
/// an represented as an optional value (e.g. using the Option type).
///
/// A synopsis of all financial data structs could be found here:
/// https://github.com/xemwebe/gurufocus_api/blob/master/FinancialDataSynopsis.ods
///
use serde::Deserialize;
use serde_json::Value;

pub use crate::strnum::FloatOrString;

/// Structure holding the history of financial data for a single stock.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FinancialData {
    pub financials: DataPeriods,
}

/// Structure holding the history of financial for different periods.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct DataPeriods {
    pub financial_template_parameters: FinancialTemplateParameters,
    pub annuals: PeriodData,
    pub quarterly: PeriodData,
}

/// Structure parameters for specific financial data template
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct FinancialTemplateParameters {
    pub ind_template: String,
    #[serde(rename = "REITs")]
    pub reits: String,
    #[serde(rename = "IsDirect")]
    pub is_direct: String,
    pub financial_report_frequency: String,
}

/// Structure holding the history of financial for annual or quarterly period.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PeriodData {
    #[serde(rename = "Fiscal Year")]
    pub fiscal_year: Vec<String>,
    #[serde(rename = "Preliminary")]
    pub preliminary: Vec<FloatOrString>,
    pub per_share_data_array: Value,
    pub common_size_ratios: Value,
    pub income_statement: Value,
    pub balance_sheet: Value,
    pub cashflow_statement: Value,
    pub valuation_ratios: Value,
    pub valuation_and_quality: Value,
}

#[cfg(test)]
mod tests {
    use super::super::*;
    use super::*;
    use std::env;

    #[tokio::test]
    async fn test_financials_non_financial() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let fin_json = gf_connect.get_financials("AMZN").await;
                assert!(fin_json.is_ok());
                let financials = serde_json::from_value::<FinancialData>(fin_json.unwrap());
                assert!(financials.is_ok());
            }
        }
    }

    #[tokio::test]
    async fn test_financials_bank() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let fin_json = gf_connect.get_financials("NYSE:JPM").await;
                assert!(fin_json.is_ok());
                let financials = serde_json::from_value::<FinancialData>(fin_json.unwrap());
                assert!(financials.is_ok());
            }
        }
    }

    #[tokio::test]
    async fn test_financials_insurance() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let fin_json = gf_connect.get_financials("AIG").await;
                assert!(fin_json.is_ok());
                let financials = serde_json::from_value::<FinancialData>(fin_json.unwrap());
                assert!(financials.is_ok());
            }
        }
    }

    #[tokio::test]
    async fn test_financials_reit() {
        if let Ok(token) = env::var("GURUFOCUS_TOKEN") {
            if !token.is_empty() {
                let gf_connect = GuruFocusConnector::new(token);
                let fin_json = gf_connect.get_financials("GOOD").await;
                assert!(fin_json.is_ok());
                let financials = serde_json::from_value::<FinancialData>(fin_json.unwrap());
                assert!(financials.is_ok());
            }
        }
    }
}
