use serde::Deserialize;

pub use crate::strnum::FloatOrString;

/// Structure holding all key ratios for a single stock.
#[derive(Deserialize, Debug)]
pub struct KeyRatios {
    #[serde(rename = "Valuation Ratio")]
    pub valuation_ratio: ValuationRatio,
    #[serde(rename = "Profitability")]
    pub profitability: Profitability,
    #[serde(rename = "Income Statement")]
    pub income_statement: IncomeStatement,
    #[serde(rename = "Growth")]
    pub growth: Growth,
    #[serde(rename = "Basic")]
    pub basic: Basic,
    #[serde(rename = "Dividends")]
    pub dividends: Dividends,
    #[serde(rename = "Valuation")]
    pub valuation: Valuation,
    #[serde(rename = "Price")]
    pub price: Price,
    #[serde(rename = "Quality")]
    pub quality: Quality,
    #[serde(rename = "Fundamental")]
    pub fundamental: Fundamental,
}

/// Structure holding valuation ratios for a single stock.
#[derive(Deserialize, Debug)]
pub struct ValuationRatio {
    #[serde(rename = "Price-to-Net-Current-Asset-Value")]
    pub price_to_net_current_asset_value: FloatOrString,
    #[serde(rename = "Price-to-EBIT (10y Median)")]
    pub price_to_ebit_10y_median: FloatOrString,
    #[serde(rename = "Forward Rate of Return (Yacktman) % (10y Low)")]
    pub forward_rate_of_return_yacktman_pct_10y_low: FloatOrString,
    #[serde(rename = "Price-to-Median-PS-Value")]
    pub price_to_median_ps_value: FloatOrString,
    #[serde(rename = "Price-to-Projected-FCF (10y High)")]
    pub price_to_projected_fcf_10y_high: FloatOrString,
    #[serde(rename = "Forward Rate of Return (Yacktman) %")]
    pub forward_rate_of_return_yacktman_pct: FloatOrString,
    #[serde(rename = "Price-to-Tangible-Book (10y Median)")]
    pub price_to_tangible_book_10y_median: FloatOrString,
    #[serde(rename = "PE Ratio (10y Low)")]
    pub pe_ratio_10y_low: FloatOrString,
    #[serde(rename = "PE Ratio (10y High)")]
    pub pe_ratio_10y_high: FloatOrString,
    #[serde(rename = "Intrinsic Value: DCF (FCF Based)")]
    pub dcf_fcf_based: FloatOrString,
    #[serde(rename = "PE Ratio")]
    pub pe_ratio: FloatOrString,
    #[serde(rename = "Median PB Value")]
    pub median_pb_value: FloatOrString,
    #[serde(rename = "Peter Lynch Fair Value")]
    pub peter_lynch_fair_value: FloatOrString,
    #[serde(rename = "EV-to-EBITDA (10y Low)")]
    pub ev_to_ebitda_10y_low: FloatOrString,
    #[serde(rename = "PE Ratio (10y Median)")]
    pub pe_ratio_10y_median: FloatOrString,
    #[serde(rename = "Price-to-Owner-Earnings (10y Median)")]
    pub price_to_owner_earnings_10y_median: FloatOrString,
    #[serde(rename = "Price-to-Median-PB-Value")]
    pub price_to_median_pb_value: FloatOrString,
    #[serde(rename = "EV-to-EBIT")]
    pub ev_to_ebit: FloatOrString,
    #[serde(rename = "PEG Ratio")]
    pub peg_ratio: FloatOrString,
    #[serde(rename = "Price-to-Graham-Number")]
    pub price_to_graham_number: FloatOrString,
    #[serde(rename = "Price-to-Operating-Cash-Flow")]
    pub price_to_operating_cash_flow: FloatOrString,
    #[serde(rename = "PS Ratio (10y High)")]
    pub ps_ratio_10y_high: FloatOrString,
    #[serde(rename = "Price-to-Projected-FCF (10y Median)")]
    pub price_to_projected_fcf_10y_median: FloatOrString,
    #[serde(rename = "Price-to-DCF (FCF Based)")]
    pub price_to_dcf_fcf_based: FloatOrString,
    #[serde(rename = "Price-to-Median-PB-Value (10y Low)")]
    pub price_to_median_pb_value_10y_low: FloatOrString,
    #[serde(rename = "Net-Net Working Capital")]
    pub net_net_working_capital: FloatOrString,
    #[serde(rename = "Price-to-Net-Cash (10y Low)")]
    pub price_to_net_cash_10y_low: FloatOrString,
    #[serde(rename = "Earnings Yield (Joel Greenblatt) %")]
    pub earnings_yield_joel_greenblatt_pct: FloatOrString,
    #[serde(rename = "PE Ratio without NRI (10y Low)")]
    pub pe_ratio_without_nri_10y_low: FloatOrString,
    #[serde(rename = "Price-to-Earnings-Power-Value")]
    pub price_to_earnings_power_value: FloatOrString,
    #[serde(rename = "Price-to-Graham-Number (10y Median)")]
    pub price_to_graham_number_10y_median: FloatOrString,
    #[serde(rename = "Price-to-Peter-Lynch-Fair-Value")]
    pub price_to_peter_lynch_fair_value: FloatOrString,
    #[serde(rename = "Price-to-DCF (Earnings Based) (10y High)")]
    pub price_to_dcf_earnings_based_10y_high: FloatOrString,
    #[serde(rename = "Price-to-Operating-Cash-Flow (10y High)")]
    pub price_to_operating_cash_flow_10y_high: FloatOrString,
    #[serde(rename = "EV-to-EBITDA (10y High)")]
    pub ev_to_ebitda_10y_high: FloatOrString,
    #[serde(rename = "PE Ratio without NRI")]
    pub pe_ratio_without_nri: FloatOrString,
    #[serde(rename = "Price-to-DCF (Earnings Based) (10y Low)")]
    pub price_to_dcf_earnings_based_10y_low: FloatOrString,
    #[serde(rename = "Earnings Yield %")]
    pub earnings_yield_pct: FloatOrString,
    #[serde(rename = "Margin of Safty % (DCF Earnings Based)")]
    pub margin_of_safty_pct_dcf_earnings_based: FloatOrString,
    #[serde(rename = "Price-to-Free-Cash-Flow (10y Low)")]
    pub price_to_free_cash_flow_10y_low: FloatOrString,
    #[serde(rename = "PE Ratio without NRI (10y High)")]
    pub pe_ratio_without_nri_10y_high: FloatOrString,
    #[serde(rename = "Forward Rate of Return (Yacktman) % (10y Median)")]
    pub forward_rate_of_return_yacktman_pct_10y_median: FloatOrString,
    #[serde(rename = "Price-to-DCF (FCF Based) (10y Low)")]
    pub price_to_dcf_fcf_based_10y_low: FloatOrString,
    #[serde(rename = "Net Current Asset Value")]
    pub net_current_asset_value: FloatOrString,
    #[serde(rename = "PB Ratio (10y High)")]
    pub pb_ratio_10y_high: FloatOrString,
    #[serde(rename = "PEG Ratio (10y Median)")]
    pub peg_ratio_10y_median: FloatOrString,
    #[serde(rename = "PS Ratio")]
    pub ps_ratio: FloatOrString,
    #[serde(rename = "Price-to-Peter-Lynch-Fair-Value (10y Low)")]
    pub price_to_peter_lynch_fair_value_10y_low: FloatOrString,
    #[serde(rename = "Price-to-Owner-Earnings (10y Low)")]
    pub price_to_owner_earnings_10y_low: FloatOrString,
    #[serde(rename = "Price-to-Median-PS-Value (10y High)")]
    pub price_to_median_ps_value_10y_high: FloatOrString,
    #[serde(rename = "Price-to-DCF (Earnings Based)")]
    pub price_to_dcf_earnings_based: FloatOrString,
    #[serde(rename = "FCF Yield % (5y Median)")]
    pub fcf_yield_pct_5y_median: FloatOrString,
    #[serde(rename = "Price-to-Median-PB-Value (10y Median)")]
    pub price_to_median_pb_value_10y_median: FloatOrString,
    #[serde(rename = "Forward PE Ratio")]
    pub forward_pe_ratio: FloatOrString,
    #[serde(rename = "Shiller PE Ratio (10y Low)")]
    pub shiller_pe_ratio_10y_low: FloatOrString,
    #[serde(rename = "PE Ratio without NRI (10y Median)")]
    pub pe_ratio_without_nri_10y_median: FloatOrString,
    #[serde(rename = "Price-to-DCF (Earnings Based) (10y Median)")]
    pub price_to_dcf_earnings_based_10y_median: FloatOrString,
    #[serde(rename = "Price-to-DCF (FCF Based) (10y Median)")]
    pub price_to_dcf_fcf_based_10y_median: FloatOrString,
    #[serde(rename = "E10")]
    pub e10: FloatOrString,
    #[serde(rename = "Intrinsic Value: Projected FCF")]
    pub intrinsic_value_projected_fcf: FloatOrString,
    #[serde(rename = "Price-to-Owner-Earnings")]
    pub price_to_owner_earnings: FloatOrString,
    #[serde(rename = "DCF (Earnings Based)")]
    pub dcf_earnings_based: FloatOrString,
    #[serde(rename = "Price-to-Median-PS-Value (10y Median)")]
    pub price_to_median_ps_value_10y_median: FloatOrString,
    #[serde(rename = "Price-to-Tangible-Book (10y High)")]
    pub price_to_tangible_book_10y_high: FloatOrString,
    #[serde(rename = "Shiller PE Ratio (10y High)")]
    pub shiller_pe_ratio_10y_high: FloatOrString,
    #[serde(rename = "EV-to-Pretax-Income")]
    pub ev_to_pretax_income: FloatOrString,
    #[serde(rename = "EV-to-EBIT (10y Low)")]
    pub ev_to_ebit_10y_low: FloatOrString,
    #[serde(rename = "Price-to-Free-Cash-Flow")]
    pub price_to_free_cash_flow: FloatOrString,
    #[serde(rename = "Price-to-Peter-Lynch-Fair-Value (10y High)")]
    pub price_to_peter_lynch_fair_value_10y_high: FloatOrString,
    #[serde(rename = "PB Ratio (10y Median)")]
    pub pb_ratio_10y_median: FloatOrString,
    #[serde(rename = "Price-to-Operating-Cash-Flow (10y Low)")]
    pub price_to_operating_cash_flow_10y_low: FloatOrString,
    #[serde(rename = "Price-to-Net-Current-Asset-Value (10y High)")]
    pub price_to_net_current_asset_value_10y_high: FloatOrString,
    #[serde(rename = "EV-to-EBITDA (10y Median)")]
    pub ev_to_ebitda_10y_median: FloatOrString,
    #[serde(rename = "Earnings Yield (Joel Greenblatt) % (10y High)")]
    pub earnings_yield_joel_greenblatt_pct_10y_high: FloatOrString,
    #[serde(rename = "Forward Rate of Return (Yacktman) % (10y High)")]
    pub forward_rate_of_return_yacktman_pct_10y_high: FloatOrString,
    #[serde(rename = "Price-to-Net-Current-Asset-Value (10y Low)")]
    pub price_to_net_current_asset_value_10y_low: FloatOrString,
    #[serde(rename = "EV-to-Revenue")]
    pub ev_to_revenue: FloatOrString,
    #[serde(rename = "Price-to-Net-Net-Working-Capital")]
    pub price_to_net_net_working_capital: FloatOrString,
    #[serde(rename = "Price-to-Projected-FCF")]
    pub price_to_projected_fcf: FloatOrString,
    #[serde(rename = "EV-to-EBITDA")]
    pub ev_to_ebitda: FloatOrString,
    #[serde(rename = "Price-to-EBITDA (10y Median)")]
    pub price_to_ebitda_10y_median: FloatOrString,
    #[serde(rename = "Price-to-Net-Cash")]
    pub price_to_net_cash: FloatOrString,
    #[serde(rename = "Shiller PE Ratio (10y Median)")]
    pub shiller_pe_ratio_10y_median: FloatOrString,
    #[serde(rename = "Earnings Yield (Joel Greenblatt) % (10y Median)")]
    pub earnings_yield_joel_greenblatt_pct_10y_median: FloatOrString,
    #[serde(rename = "Earnings Power Value (EPV)")]
    pub earnings_power_value_epv: FloatOrString,
    #[serde(rename = "Price-to-DCF (FCF Based) (10y High)")]
    pub price_to_dcf_fcf_based_10y_high: FloatOrString,
    #[serde(rename = "Price-to-Free-Cash-Flow (10y High)")]
    pub price_to_free_cash_flow_10y_high: FloatOrString,
    #[serde(rename = "Price-to-Free-Cash-Flow (10y Median)")]
    pub price_to_free_cash_flow_10y_median: FloatOrString,
    #[serde(rename = "Price-to-Graham-Number (10y High)")]
    pub price_to_graham_number_10y_high: FloatOrString,
    #[serde(rename = "Price-to-Owner-Earnings (10y High)")]
    pub price_to_owner_earnings_10y_high: FloatOrString,
    #[serde(rename = "Price-to-Projected-FCF (10y Low)")]
    pub price_to_projected_fcf_10y_low: FloatOrString,
    #[serde(rename = "Price-to-Tangible-Book")]
    pub price_to_tangible_book: FloatOrString,
    #[serde(rename = "Price-to-Graham-Number (10y Low)")]
    pub price_to_graham_number_10y_low: FloatOrString,
    #[serde(rename = "PEG Ratio (10y Low)")]
    pub peg_ratio_10y_low: FloatOrString,
    #[serde(rename = "PEG Ratio (10y High)")]
    pub peg_ratio_10y_high: FloatOrString,
    #[serde(rename = "PB Ratio")]
    pub pb_ratio: FloatOrString,
    #[serde(rename = "Earnings Yield (Joel Greenblatt) % (10y Low)")]
    pub earnings_yield_joel_greenblatt_pct_10y_low: FloatOrString,
    #[serde(rename = "PS Ratio (10y Low)")]
    pub ps_ratio_10y_low: FloatOrString,
    #[serde(rename = "Price-to-Net-Cash (10y Median)")]
    pub price_to_net_cash_10y_median: FloatOrString,
    #[serde(rename = "Tangible Book per Share")]
    pub tangible_book_per_share: FloatOrString,
    #[serde(rename = "Owner Earnings per Share (TTM)")]
    pub owner_earnings_per_share_ttm: FloatOrString,
    #[serde(rename = "EV-to-Revenue (10y Low)")]
    pub ev_to_revenue_10y_low: FloatOrString,
    #[serde(rename = "Price-to-Median-PB-Value (10y High)")]
    pub price_to_median_pb_value_10y_high: FloatOrString,
    #[serde(rename = "Price-to-Net-Cash (10y High)")]
    pub price_to_net_cash_10y_high: FloatOrString,
    #[serde(rename = "EV-to-Revenue (10y Median)")]
    pub ev_to_revenue_10y_median: FloatOrString,
    #[serde(rename = "Price-to-Peter-Lynch-Fair-Value (10y Median)")]
    pub price_to_peter_lynch_fair_value_10y_median: FloatOrString,
    #[serde(rename = "Price-to-Tangible-Book (10y Low)")]
    pub price_to_tangible_book_10y_low: FloatOrString,
    #[serde(rename = "Graham Number")]
    pub graham_number: FloatOrString,
    #[serde(rename = "Price-to-Net-Current-Asset-Value (10y Median)")]
    pub price_to_net_current_asset_value_10y_median: FloatOrString,
    #[serde(rename = "EV-to-EBIT (10y Median)")]
    pub ev_to_ebit_10y_median: FloatOrString,
    #[serde(rename = "EV-to-Revenue (10y High)")]
    pub ev_to_revenue_10y_high: FloatOrString,
    #[serde(rename = "PB Ratio (10y Low)")]
    pub pb_ratio_10y_low: FloatOrString,
    #[serde(rename = "Median PS Value")]
    pub median_ps_value: FloatOrString,
    #[serde(rename = "Price-to-Median-PS-Value (10y Low)")]
    pub price_to_median_ps_value_10y_low: FloatOrString,
    #[serde(rename = "Price-to-Operating-Cash-Flow (10y Median)")]
    pub price_to_operating_cash_flow_10y_median: FloatOrString,
    #[serde(rename = "Shiller PE Ratio")]
    pub shiller_pe_ratio: FloatOrString,
    #[serde(rename = "PS Ratio (10y Median)")]
    pub ps_ratio_10y_median: FloatOrString,
    #[serde(rename = "EV-to-EBIT (10y High)")]
    pub ev_to_ebit_10y_high: FloatOrString,
    #[serde(rename = "FCF Yield %")]
    pub fcf_yield_pct: FloatOrString,
}

#[derive(Deserialize, Debug)]
pub struct Profitability {
    #[serde(rename = "Net Margin %")]
    pub net_margin_pct: FloatOrString,
    #[serde(rename = "Net Margin % (10y Low)")]
    pub net_margin_pct_10y_low: FloatOrString,
    #[serde(rename = "Pretax Margin % (10y Low)")]
    pub pretax_margin_pct_10y_low: FloatOrString,
    #[serde(rename = "Gross Margin %")]
    pub gross_margin_pct: FloatOrString,
    #[serde(rename = "Operating Margin % (10y High)")]
    pub operating_margin_pct_10y_high: FloatOrString,
    #[serde(rename = "Gross Margin % (10y High)")]
    pub gross_margin_pct_10y_high: FloatOrString,
    #[serde(rename = "FCF Margin %")]
    pub fcf_margin_pct: FloatOrString,
    #[serde(rename = "Gross Margin % (10y Low)")]
    pub gross_margin_pct_10y_low: FloatOrString,
    #[serde(rename = "Operating Margin % (10y Median)")]
    pub operating_margin_pct_10y_median: FloatOrString,
    #[serde(rename = "Operating Margin % (5y Median)")]
    pub operating_margin_pct_5y_median: FloatOrString,
    #[serde(rename = "Pretax Margin %")]
    pub pretax_margin_pct: FloatOrString,
    #[serde(rename = "Gross Margin % (10y Median)")]
    pub gross_margin_pct_10y_median: FloatOrString,
    #[serde(rename = "Net Interest Margin (Bank Only) %")]
    pub net_interest_margin_bank_only_pct: FloatOrString,
    #[serde(rename = "Net Margin % (10y Median)")]
    pub net_margin_pct_10y_median: FloatOrString,
    #[serde(rename = "Num Of Years Of Profitability Over The Past 10 Years")]
    pub num_of_years_of_profitability_over_the_past_10_years: FloatOrString,
    #[serde(rename = "Operating Margin %")]
    pub operating_margin_pct: FloatOrString,
    #[serde(rename = "Pretax Margin % (10y High)")]
    pub pretax_margin_pct_10y_high: FloatOrString,
    #[serde(rename = "FCF Margin % (5y Median)")]
    pub fcf_margin_pct_5y_median: FloatOrString,
    #[serde(rename = "Net Margin % (10y High)")]
    pub net_margin_pct_10y_high: FloatOrString,
    #[serde(rename = "Operating Margin % (10y Low)")]
    pub operating_margin_pct_10y_low: FloatOrString,
    #[serde(rename = "Pretax Margin % (10y Median)")]
    pub pretax_margin_pct_10y_median: FloatOrString,
}

#[derive(Deserialize, Debug)]
pub struct IncomeStatement {
    #[serde(rename = "Selling, General, & Admin. Expense")]
    pub selling_general_and_admin_expense: FloatOrString,
}

#[derive(Deserialize, Debug)]
pub struct Growth {
    #[serde(rename = "10-Year Dividend Growth Rate (Per Share)")]
    pub _10_year_dividend_growth_rate_per_share: FloatOrString,
    #[serde(rename = "2nd Latest Q EBIT YoY Growth")]
    pub _2nd_latest_q_ebit_yoy_growth: FloatOrString,
    #[serde(rename = "3-Year Book Growth Rate (10y Low)")]
    pub _3_year_book_growth_rate_10y_low: FloatOrString,
    #[serde(rename = "3-Year Dividend Growth Rate (10y Low)")]
    pub _3_year_dividend_growth_rate_10y_low: FloatOrString,
    #[serde(rename = "3-Year Revenue Growth Rate (10y High)")]
    pub _3_year_revenue_growth_rate_10y_high: FloatOrString,
    #[serde(rename = "3-Year Total Revenue Growth Rate (10y Low)")]
    pub _3_year_total_revenue_growth_rate_10y_low: FloatOrString,
    #[serde(rename = "5-Year Gross Margin Growth Rate")]
    pub _5_year_gross_margin_growth_rate: FloatOrString,
    #[serde(rename = "5-Year Operating Margin Growth Rate")]
    pub _5_year_operating_margin_growth_rate: FloatOrString,
    #[serde(rename = "3-Year EPS without NRI Growth Rate (10y High)")]
    pub _3_year_eps_without_nri_growth_rate_10y_high: FloatOrString,
    #[serde(rename = "Reversed DCF Growth Rate")]
    pub reversed_dcf_growth_rate: FloatOrString,
    #[serde(rename = "3rd Latest Q EBIT YoY Growth")]
    pub _3rd_latest_q_ebit_yoy_growth: FloatOrString,
    #[serde(rename = "5-Year Debt-to-Revenue Growth Rate")]
    pub _5_year_debt_to_revenue_growth_rate: FloatOrString,
    #[serde(rename = "3-Year Total EBITDA Growth Rate")]
    pub _3_year_total_ebitda_growth_rate: FloatOrString,
    #[serde(rename = "10-Year Operating Income Growth Rate (Per Share)")]
    pub _10_year_operating_income_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year FCF Growth Rate (10y Low)")]
    pub _3_year_fcf_growth_rate_10y_low: FloatOrString,
    #[serde(rename = "1-Year Total Revenue Growth Rate")]
    pub _1_year_total_revenue_growth_rate: FloatOrString,
    #[serde(rename = "3-Year Market Cap Change %")]
    pub _3_year_market_cap_change_pct: FloatOrString,
    #[serde(rename = "3-Year Net Income Growth Rate (10y Median)")]
    pub _3_year_net_income_growth_rate_10y_median: FloatOrString,
    #[serde(rename = "3-Year FCF Growth Rate (Per Share)")]
    pub _3_year_fcf_growth_rate_per_share: FloatOrString,
    #[serde(rename = "1-Year Total EBITDA Growth Rate")]
    pub _1_year_total_ebitda_growth_rate: FloatOrString,
    #[serde(rename = "3-Year Revenue Growth Rate (10y Low)")]
    pub _3_year_revenue_growth_rate_10y_low: FloatOrString,
    #[serde(rename = "3-Year FCF Growth Rate (10y High)")]
    pub _3_year_fcf_growth_rate_10y_high: FloatOrString,
    #[serde(rename = "1-Year Book Growth Rate (Per Share)")]
    pub _1_year_book_growth_rate_per_share: FloatOrString,
    #[serde(rename = "5-Year FCF Growth Rate (Per Share)")]
    pub _5_year_fcf_growth_rate_per_share: FloatOrString,
    #[serde(rename = "5-Year Market Cap Change %")]
    pub _5_year_market_cap_change_pct: FloatOrString,
    #[serde(rename = "3-Year Operating Income Growth Rate (10y Low)")]
    pub _3_year_operating_income_growth_rate_10y_low: FloatOrString,
    #[serde(rename = "3-Year Total Revenue Growth Rate (10y Median)")]
    pub _3_year_total_revenue_growth_rate_10y_median: FloatOrString,
    #[serde(rename = "5-Year Long-Term Debt Change %")]
    pub _5_year_long_term_debt_change_pct: FloatOrString,
    #[serde(rename = "5-Year ROC (Joel Greenblatt) % Growth Rate")]
    pub _5_year_roc_joel_greenblatt_pct_growth_rate: FloatOrString,
    #[serde(rename = "1st Latest Q EBIT YoY Growth")]
    pub _1st_latest_q_ebit_yoy_growth: FloatOrString,
    #[serde(rename = "3-Year Operating Income Growth Rate (10y High)")]
    pub _3_year_operating_income_growth_rate_10y_high: FloatOrString,
    #[serde(rename = "3-Year FCF Growth Rate (10y Median)")]
    pub _3_year_fcf_growth_rate_10y_median: FloatOrString,
    #[serde(rename = "5-Year Revenue Growth Rate (Per Share)")]
    pub _5_year_revenue_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year Revenue Growth Rate (10y Median)")]
    pub _3_year_revenue_growth_rate_10y_median: FloatOrString,
    #[serde(rename = "1-Year Long-Term Debt Change %")]
    pub _1_year_long_term_debt_change_pct: FloatOrString,
    #[serde(rename = "3-Year Asset Growth Rate (10y Median)")]
    pub _3_year_asset_growth_rate_10y_median: FloatOrString,
    #[serde(rename = "3-Year Total EBITDA Growth Rate (10y High)")]
    pub _3_year_total_ebitda_growth_rate_10y_high: FloatOrString,
    #[serde(rename = "3-Year EPS without NRI Growth Rate (10y Low)")]
    pub _3_year_eps_w_nri_growth_rate_10y_low: FloatOrString,
    #[serde(rename = "5-Year EBITDA Growth Rate (Per Share)")]
    pub _5_year_ebitda_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year Total EBITDA Growth Rate (10y Median)")]
    pub _3_year_total_ebitda_growth_rate_10y_median: FloatOrString,
    #[serde(rename = "EPS Growth Rate (Future 3Y To 5Y Estimate)")]
    pub eps_growth_rate_future_3y_to_5y_estimate: FloatOrString,
    #[serde(rename = "1-Year Dividend Growth Rate (Per Share)")]
    pub _1_year_dividend_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year Book Growth Rate (Per Share)")]
    pub _3_year_book_growth_rate_per_share: FloatOrString,
    #[serde(rename = "10-Year Total EBITDA Growth Rate")]
    pub _10_year_total_ebitda_growth_rate: FloatOrString,
    #[serde(rename = "3-Year Operating Income Growth Rate (10y Median)")]
    pub _3_year_operating_income_growth_rate_10y_median: FloatOrString,
    #[serde(rename = "3-Year Operating Income Growth Rate (Per Share)")]
    pub _3_year_operating_income_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year Total Revenue Growth Rate (10y High)")]
    pub _3_year_total_revenue_growth_rate_10y_high: FloatOrString,
    #[serde(rename = "1-Year Debt-to-Revenue Growth Rate")]
    pub _1_year_debt_to_revenue_growth_rate: FloatOrString,
    #[serde(rename = "1-Year Operating Income Growth Rate (Per Share)")]
    pub _1_year_operating_income_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year Revenue Growth Rate (Per Share)")]
    pub _3_year_revenue_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year Total EBITDA Growth Rate (10y Low)")]
    pub _3_year_total_ebitda_growth_rate_10y_low: FloatOrString,
    #[serde(rename = "3-Year EPS without NRI Growth Rate (10y Median)")]
    pub _3_year_eps_w_nri_growth_rate_10y_median: FloatOrString,
    #[serde(rename = "3-Year EPS without NRI Growth Rate")]
    pub _3_year_eps_w_nri_growth_rate: FloatOrString,
    #[serde(rename = "1-Year Asset Growth Rate")]
    pub _1_year_asset_growth_rate: FloatOrString,
    #[serde(rename = "1-Year EBITDA Growth Rate (Per Share)")]
    pub _1_year_ebitda_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year Asset Growth Rate (10y Low)")]
    pub _3_year_asset_growth_rate_10y_low: FloatOrString,
    #[serde(rename = "1-Year Piotroski F-Score Change")]
    pub _1_year_piotroski_f_score_change: FloatOrString,
    #[serde(rename = "10-Year Debt-to-Revenue Growth Rate")]
    pub _10_year_debt_to_revenue_growth_rate: FloatOrString,
    #[serde(rename = "10-Year EBITDA Growth Rate (Per Share)")]
    pub _10_year_ebitda_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year EBITDA Growth Rate (Per Share)")]
    pub _3_year_ebitda_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year Net Income Growth Rate (10y High)")]
    pub _3_year_net_income_growth_rate_10y_high: FloatOrString,
    #[serde(rename = "5-Year Total EBITDA Growth Rate")]
    pub _5_year_total_ebitda_growth_rate: FloatOrString,
    #[serde(rename = "3-Year EBITDA Growth Rate (10y Median)")]
    pub _3_year_ebitda_growth_rate_10y_median: FloatOrString,
    #[serde(rename = "5-Year Dividend Growth Rate (Per Share)")]
    pub _5_year_dividend_growth_rate_per_share: FloatOrString,
    #[serde(rename = "5-Year Total Revenue Growth Rate")]
    pub _5_year_total_revenue_growth_rate: FloatOrString,
    #[serde(rename = "5-Year EPS without NRI Growth Rate")]
    pub _5_year_eps_w_nri_growth_rate: FloatOrString,
    #[serde(rename = "3-Year Book Growth Rate (10y High)")]
    pub _3_year_book_growth_rate_10y_high: FloatOrString,
    #[serde(rename = "1-Year FCF Growth Rate (Per Share)")]
    pub _1_year_fcf_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year Asset Growth Rate")]
    pub _3_year_asset_growth_rate: FloatOrString,
    #[serde(rename = "5-Year Operating Income Growth Rate (Per Share)")]
    pub _5_year_operating_income_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year Total Revenue Growth Rate")]
    pub _3_year_total_revenue_growth_rate: FloatOrString,
    #[serde(rename = "10-Year Revenue Growth Rate (Per Share)")]
    pub _10_year_revenue_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year Net Income Growth Rate (10y Low)")]
    pub _3_year_net_income_growth_rate_10y_low: FloatOrString,
    #[serde(rename = "10-Year Total Revenue Growth Rate")]
    pub _10_year_total_revenue_growth_rate: FloatOrString,
    #[serde(rename = "3-Year Long-Term Debt Change %")]
    pub _3_year_long_term_debt_change_pct: FloatOrString,
    #[serde(rename = "5-Year Asset Growth Rate")]
    pub _5_year_asset_growth_rate: FloatOrString,
    #[serde(rename = "3-Year Dividend Growth Rate (10y Median)")]
    pub _3_year_dividend_growth_rate_10y_median: FloatOrString,
    #[serde(rename = "1-Year Market Cap Change %")]
    pub _1_year_market_cap_change_pct: FloatOrString,
    #[serde(rename = "10-Year EPS without NRI Growth Rate")]
    pub _10_year_eps_w_nri_growth_rate: FloatOrString,
    #[serde(rename = "3-Year Dividend Growth Rate (Per Share)")]
    pub _3_year_dividend_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year Net Income Growth Rate")]
    pub _3_year_net_income_growth_rate: FloatOrString,
    #[serde(rename = "10-Year Asset Growth Rate")]
    pub _10_year_asset_growth_rate: FloatOrString,
    #[serde(rename = "5-Year Book Growth Rate (Per Share)")]
    pub _5_year_book_growth_rate_per_share: FloatOrString,
    #[serde(rename = "4th Latest Q EBIT YoY Growth")]
    pub _4th_latest_q_ebit_yoy_growth: FloatOrString,
    #[serde(rename = "3-Year Dividend Growth Rate (10y High)")]
    pub _3_year_dividend_growth_rate_10y_high: FloatOrString,
    #[serde(rename = "1-Year Net Income Growth Rate")]
    pub _1_year_net_income_growth_rate: FloatOrString,
    #[serde(rename = "1-Year Revenue Growth Rate (Per Share)")]
    pub _1_year_revenue_growth_rate_per_share: FloatOrString,
    #[serde(rename = "3-Year Book Growth Rate (10y Median)")]
    pub _3_year_book_growth_rate_10y_median: FloatOrString,
    #[serde(rename = "3-Year EBITDA Growth Rate (10y Low)")]
    pub _3_year_ebitda_growth_rate_10y_low: FloatOrString,
    #[serde(rename = "5-Year Net Income Growth Rate")]
    pub _5_year_net_income_growth_rate: FloatOrString,
    #[serde(rename = "10-Year Book Growth Rate (Per Share)")]
    pub _10_year_book_growth_rate_per_share: FloatOrString,
    #[serde(rename = "10-Year FCF Growth Rate (Per Share)")]
    pub _10_year_fcf_growth_rate_per_share: FloatOrString,
    #[serde(rename = "10-Year Net Income Growth Rate")]
    pub _10_year_net_income_growth_rate: FloatOrString,
    #[serde(rename = "1-Year EPS without NRI Growth Rate")]
    pub _1_year_eps_wo_nri_growth_rate: FloatOrString,
    #[serde(rename = "3-Year EBITDA Growth Rate (10y High)")]
    pub _3_year_ebitda_growth_rate_10y_high: FloatOrString,
    #[serde(rename = "3-Year Asset Growth Rate (10y High)")]
    pub _3_year_asset_growth_rate_10y_high: FloatOrString,
}

#[derive(Deserialize, Debug)]
pub struct Basic {
    #[serde(rename = "Price Updated Time")]
    pub price_updated_time: String,
}

#[derive(Deserialize, Debug)]
pub struct Dividends {
    #[serde(rename = "Dividend-Payout-to-FFO")]
    pub dividend_payout_to_ffo: FloatOrString,
    #[serde(rename = "Forward Dividend Yield %")]
    pub forward_dividend_yield_pct: FloatOrString,
    #[serde(rename = "Dividend Payout Ratio (10y Median)")]
    pub dividend_payout_ratio_10y_median: FloatOrString,
    #[serde(rename = "Yield-on-Cost (5-Year) %")]
    pub yield_on_cost_5_year_pct: FloatOrString,
    #[serde(rename = "Dividend Payout Ratio")]
    pub dividend_payout_ratio: FloatOrString,
    #[serde(rename = "Dividend Payment Months")]
    pub dividend_payment_months: FloatOrString,
    #[serde(rename = "Dividend Yield % (10y Low)")]
    pub dividend_yield_pct_10y_low: FloatOrString,
    #[serde(rename = "Yield-on-Cost % (10y High)")]
    pub yield_on_cost_pct_10y_high: FloatOrString,
    #[serde(rename = "Dividend Frequency")]
    pub dividend_frequency: FloatOrString,
    #[serde(rename = "Dividend Payout Ratio (10y High)")]
    pub dividend_payout_ratio_10y_high: FloatOrString,
    #[serde(rename = "Dividend Yield % (10y High)")]
    pub dividend_yield_pct_10y_high: FloatOrString,
    #[serde(rename = "Dividend Payout Ratio (10y Low)")]
    pub dividend_payout_ratio_10y_low: FloatOrString,
    #[serde(rename = "Dividend Start Year")]
    pub dividend_start_year: FloatOrString,
    #[serde(rename = "Yield-on-Cost % (10y Median)")]
    pub yield_on_cost_pct_10y_median: FloatOrString,
    #[serde(rename = "Forward 12M Dividend")]
    pub forward_12m_dividend: FloatOrString,
    #[serde(rename = "Yield-on-Cost % (10y Low)")]
    pub yield_on_cost_pct_10y_low: FloatOrString,
    #[serde(rename = "Dividend Yield % (10y Median)")]
    pub dividend_yield_pct_10y_median: FloatOrString,
    #[serde(rename = "Trailing 12-Month Dividend")]
    pub trailing_12_month_dividend: FloatOrString,
}

#[derive(Deserialize, Debug)]
pub struct Valuation {
    #[serde(rename = "Earnings Power Value (EPV)")]
    pub epv: FloatOrString,
}

#[derive(Deserialize, Debug)]
pub struct Price {
    #[serde(rename = "50-Day SMA")]
    pub _50_day_sma: FloatOrString,
    #[serde(rename = "Beta")]
    pub beta: FloatOrString,
    #[serde(rename = "3-Month Relative to S&P Total Return %")]
    pub _3_month_relative_to_sp_total_return_pct: FloatOrString,
    #[serde(rename = "Float")]
    pub float: FloatOrString,
    #[serde(rename = "6-Month Total Return %")]
    pub _6_month_total_return_pct: FloatOrString,
    #[serde(rename = "3-Year Annualized Total Return %")]
    pub _3_year_annualized_total_return_pct: FloatOrString,
    #[serde(rename = "Day's Change")]
    pub days_change: FloatOrString,
    #[serde(rename = "1-Month Relative to S&P Total Return %")]
    pub _1_month_relative_to_sp_total_return_pct: FloatOrString,
    #[serde(rename = "12-Month Relative to S&P Total Return %")]
    pub _12_month_relative_to_sp_total_return_pct: FloatOrString,
    #[serde(rename = "Price (3y High)")]
    pub price_3y_high: FloatOrString,
    #[serde(rename = "1-Month Total Return %")]
    pub _1_month_total_return_pct: FloatOrString,
    #[serde(rename = "YTD Relative to S&P Total Return %")]
    pub ytd_relative_to_sp_total_return_pct: FloatOrString,
    #[serde(rename = "Day's Open")]
    pub days_open: FloatOrString,
    #[serde(rename = "14-Day RSI")]
    pub _14_day_rsi: FloatOrString,
    #[serde(rename = "12-Month Total Return %")]
    pub _12_month_total_return_pct: FloatOrString,
    #[serde(rename = "6-Month Price Index")]
    pub _6_month_price_index: FloatOrString,
    #[serde(rename = "Price (5y High)")]
    pub price_5y_high: FloatOrString,
    #[serde(rename = "Price (52w High)")]
    pub price_52w_high: FloatOrString,
    #[serde(rename = "Total 2-Month Average Trade Volume")]
    pub total_2_month_average_trade_volume: FloatOrString,
    #[serde(rename = "6-Month Relative to S&P Total Return %")]
    pub _6_month_relative_to_sp_total_return_pct: FloatOrString,
    #[serde(rename = "5-Year Annualized Total Return %")]
    pub _5_year_annualized_total_return_pct: FloatOrString,
    #[serde(rename = "10-Year Annualized Total Return %")]
    pub _10_year_annualized_total_return_pct: FloatOrString,
    #[serde(rename = "Price (52w Low)")]
    pub price_52w_low: FloatOrString,
    #[serde(rename = "1-Week Total Return %")]
    pub _1_week_total_return_pct: FloatOrString,
    #[serde(rename = "Avg Daily Trade Volume (2 Months)")]
    pub avg_daily_trade_volume_2_months: FloatOrString,
    #[serde(rename = "Price (10y Low)")]
    pub price_10y_low: FloatOrString,
    #[serde(rename = "3-Month Total Return %")]
    pub _3_month_total_return_pct: FloatOrString,
    #[serde(rename = "Price (5y Low)")]
    pub price_5y_low: FloatOrString,
    #[serde(rename = "200-Day SMA")]
    pub _200_day_sma: FloatOrString,
    #[serde(rename = "Day's Volume")]
    pub days_volume: FloatOrString,
    #[serde(rename = "Price (3y Low)")]
    pub price_3y_low: FloatOrString,
    #[serde(rename = "20-Day SMA")]
    pub _20_day_sma: FloatOrString,
    #[serde(rename = "Day's Low")]
    pub days_low: FloatOrString,
    #[serde(rename = "Price (10y High)")]
    pub price_10y_high: FloatOrString,
    #[serde(rename = "Volatility")]
    pub volatility: FloatOrString,
    #[serde(rename = "YTD Total Return %")]
    pub ytd_total_return_pct: FloatOrString,
    #[serde(rename = "Total Daily Trade Volume")]
    pub total_daily_trade_volume: FloatOrString,
    #[serde(rename = "Day's High")]
    pub days_high: FloatOrString,
    #[serde(rename = "1-Week Relative to S&P Total Return %")]
    pub _1_week_relative_to_sp_total_return_pct: FloatOrString,
}

#[derive(Deserialize, Debug)]
pub struct Quality {
    #[serde(rename = "Predictability Rank")]
    pub predictability_rank: FloatOrString,
}

#[derive(Deserialize, Debug)]
pub struct Fundamental {
    #[serde(rename = "Return-on-Tangible-Equity (10y Median)")]
    pub return_on_tangible_equity_10y_median: FloatOrString,
    #[serde(rename = "Piotroski F-Score (10y Low)")]
    pub piotroski_f_score_10y_low: FloatOrString,
    #[serde(rename = "Sales (M)")]
    pub sales_m: FloatOrString,
    #[serde(rename = "Interest Coverage (10y Median)")]
    pub interest_coverage_10y_median: FloatOrString,
    #[serde(rename = "Total Payout Yield %")]
    pub total_payout_yield_pct: FloatOrString,
    #[serde(rename = "Return-on-Tangible-Equity")]
    pub return_on_tangible_equity: FloatOrString,
    #[serde(rename = "Enterprise Value ($M)")]
    pub enterprise_value_million_usd: FloatOrString,
    #[serde(rename = "Probability of Financial Distress (%)")]
    pub probability_of_financial_distress_pct: FloatOrString,
    #[serde(rename = "Estimated Sales of Next FY (M)")]
    pub estimated_sales_of_next_fy_m: FloatOrString,
    #[serde(rename = "ROA %")]
    pub roa_pct: FloatOrString,
    #[serde(rename = "Trailing 12-Month Pretax Income")]
    pub trailing_12_month_pretax_income: FloatOrString,
    #[serde(rename = "3-Year Average Share Buyback Ratio (10y Median)")]
    pub _3_year_average_share_buyback_ratio_10y_median: FloatOrString,
    #[serde(rename = "Altman Z-Score (10y Low)")]
    pub altman_z_score_10y_low: FloatOrString,
    #[serde(rename = "Warning Signs (Medium)")]
    pub warning_signs_medium: FloatOrString,
    #[serde(rename = "Warning Signs (Severe)")]
    pub warning_signs_severe: FloatOrString,
    #[serde(rename = "Current Ratio (10y High)")]
    pub current_ratio_10y_high: FloatOrString,
    #[serde(rename = "Trailing 12-Month EBIT")]
    pub trailing_12_month_ebit: FloatOrString,
    #[serde(rename = "Gross-Profit-to-Tangible-Asset")]
    pub gross_profit_to_tangible_asset: FloatOrString,
    #[serde(rename = "Debt-to-Equity (10y Low)")]
    pub debt_to_equity_10y_low: FloatOrString,
    #[serde(rename = "Most Recent Financial Update")]
    pub most_recent_financial_update: FloatOrString,
    #[serde(rename = "Num of Analyst Following")]
    pub num_of_analyst_following: FloatOrString,
    #[serde(rename = "Institution Shares Held")]
    pub institution_shares_held: FloatOrString,
    #[serde(rename = "Headquarter Country")]
    pub headquarter_country: FloatOrString,
    #[serde(rename = "ROE %")]
    pub roe_pct: FloatOrString,
    #[serde(rename = "3-Year Average Share Buyback Ratio (10y High)")]
    pub _3_year_average_share_buyback_ratio_10y_high: FloatOrString,
    #[serde(rename = "Piotroski F-Score (10y Median)")]
    pub piotroski_f_score_10y_median: FloatOrString,
    #[serde(rename = "ROA % (10y High)")]
    pub roa_pct_10y_high: FloatOrString,
    #[serde(rename = "Debt-to-Equity (10y Median)")]
    pub debt_to_equity_10y_median: FloatOrString,
    #[serde(rename = "Trailing 12-Month EPS")]
    pub trailing_12_month_eps: FloatOrString,
    #[serde(rename = "Days Payable")]
    pub days_payable: FloatOrString,
    #[serde(rename = "Goodwill-to-Asset")]
    pub goodwill_to_asset: FloatOrString,
    #[serde(rename = "Debt-to-Equity (10y High)")]
    pub debt_to_equity_10y_high: FloatOrString,
    #[serde(rename = "ROE % (5y Median)")]
    pub roe_pct_5y_median: FloatOrString,
    #[serde(rename = "Days Payable (10y High)")]
    pub days_payable_10y_high: FloatOrString,
    #[serde(rename = "Debt-to-EBITDA (10y Median)")]
    pub debt_to_ebitda_10y_median: FloatOrString,
    #[serde(rename = "Days Inventory (10y High)")]
    pub days_inventory_10y_high: FloatOrString,
    #[serde(rename = "WACC %")]
    pub wacc_pct: FloatOrString,
    #[serde(rename = "Current Ratio (10y Low)")]
    pub current_ratio_10y_low: FloatOrString,
    #[serde(rename = "Debt-to-EBITDA (10y High)")]
    pub debt_to_ebitda_10y_high: FloatOrString,
    #[serde(rename = "Debt-to-Revenue")]
    pub debt_to_revenue: FloatOrString,
    #[serde(rename = "Financial Strength")]
    pub financial_strength: FloatOrString,
    #[serde(rename = "Market Cap ($M)")]
    pub market_cap_million_usd: FloatOrString,
    #[serde(rename = "Beneish M-Score (10y Low)")]
    pub beneish_m_score_10y_low: FloatOrString,
    #[serde(rename = "Days Sales Outstanding (10y Low)")]
    pub days_sales_outstanding_10y_low: FloatOrString,
    #[serde(rename = "Current Ratio")]
    pub current_ratio: FloatOrString,
    #[serde(rename = "Cash-to-Debt (10y High)")]
    pub cash_to_debt_10y_high: FloatOrString,
    #[serde(rename = "10-Year Share Buyback Rate")]
    pub _10_year_share_buyback_rate: FloatOrString,
    #[serde(rename = "Latest Quarter End")]
    pub latest_quarter_end: FloatOrString,
    #[serde(rename = "1-Year Share Buyback Rate")]
    pub _1_year_share_buyback_rate: FloatOrString,
    #[serde(rename = "ROC (Joel Greenblatt) % (10y Low)")]
    pub roc_joel_greenblatt_pct_10y_low: FloatOrString,
    #[serde(rename = "ROIC %")]
    pub roic_pct: FloatOrString,
    #[serde(rename = "Next Earnings Date")]
    pub next_earnings_date: FloatOrString,
    #[serde(rename = "Tax Rate % (5y Median)")]
    pub tax_rate_pct_5y_median: FloatOrString,
    #[serde(rename = "Beneish M-Score")]
    pub beneish_m_score: FloatOrString,
    #[serde(rename = "Equity-to-Asset (10y High)")]
    pub equity_to_asset_10y_high: FloatOrString,
    #[serde(rename = "Total Assets (Current)")]
    pub total_assets_current: FloatOrString,
    #[serde(rename = "ROE % (10y High)")]
    pub roe_pct_10y_high: FloatOrString,
    #[serde(rename = "Total Payout Ratio")]
    pub total_payout_ratio: FloatOrString,
    #[serde(rename = "Trailing 12-Month EPS without NRI")]
    pub trailing_12_month_eps_without_nri: FloatOrString,
    #[serde(rename = "Days Sales Outstanding (10y High)")]
    pub days_sales_outstanding_10y_high: FloatOrString,
    #[serde(rename = "Insider Ownership")]
    pub insider_ownership: FloatOrString,
    #[serde(rename = "EPS without NRI")]
    pub eps_without_nri: FloatOrString,
    #[serde(rename = "Trailing 12-Month EBITDA")]
    pub trailing_12_month_ebitda: FloatOrString,
    #[serde(rename = "Cash-to-Debt (10y Low)")]
    pub cash_to_debt_10y_low: FloatOrString,
    #[serde(rename = "Quick Ratio (10y High)")]
    pub quick_ratio_10y_high: FloatOrString,
    #[serde(rename = "Next Ex-Dividend Date")]
    pub next_ex_dividend_date: FloatOrString,
    #[serde(rename = "ROA % (5y Median)")]
    pub roa_pct_5y_median: FloatOrString,
    #[serde(rename = "ROE % (10y Low)")]
    pub roe_pct_10y_low: FloatOrString,
    #[serde(rename = "EPS")]
    pub eps: FloatOrString,
    #[serde(rename = "Return-on-Tangible-Asset (10y Low)")]
    pub return_on_tangible_asset_10y_low: FloatOrString,
    #[serde(rename = "Beneish M-Score (10y High)")]
    pub beneish_m_score_10y_high: FloatOrString,
    #[serde(rename = "Selling, General, & Admin. Expense")]
    pub selling_general_admin_expense: FloatOrString,
    #[serde(rename = "IPO Date")]
    pub ipo_date: FloatOrString,
    #[serde(rename = "Insider Shares Owned")]
    pub insider_shares_owned: FloatOrString,
    #[serde(rename = "Quick Ratio (10y Median)")]
    pub quick_ratio_10y_median: FloatOrString,
    #[serde(rename = "Scaled Net Operating Assets")]
    pub scaled_net_operating_assets: FloatOrString,
    #[serde(rename = "Altman Z-Score (10y Median)")]
    pub altman_z_score_10y_median: FloatOrString,
    #[serde(rename = "Days Sales Outstanding")]
    pub days_sales_outstanding: FloatOrString,
    #[serde(rename = "Sloan Ratio %")]
    pub sloan_ratio_pct: FloatOrString,
    #[serde(rename = "5-Year Share Buyback Rate")]
    pub _5_year_share_buyback_rate: FloatOrString,
    #[serde(rename = "Days Inventory (10y Median)")]
    pub days_inventory_10y_median: FloatOrString,
    #[serde(rename = "ROC (ROIC) (5y Median)")]
    pub roc_roic_5y_median: FloatOrString,
    #[serde(rename = "Return-on-Tangible-Equity (10y Low)")]
    pub return_on_tangible_equity_10y_low: FloatOrString,
    #[serde(rename = "Beneish M-Score (10y Median)")]
    pub beneish_m_score_10y_median: FloatOrString,
    #[serde(rename = "Cash-to-Debt (10y Median)")]
    pub cash_to_debt_10y_median: FloatOrString,
    #[serde(rename = "ROC (ROIC) (10y Median)")]
    pub roc_roic_10y_median: FloatOrString,
    #[serde(rename = "Piotroski F-Score")]
    pub piotroski_f_score: FloatOrString,
    #[serde(rename = "ROE % Adjusted to Book Value")]
    pub roe_pct_adjusted_to_book_value: FloatOrString,
    #[serde(rename = "Equity-to-Asset (10y Median)")]
    pub equity_to_asset_10y_median: FloatOrString,
    #[serde(rename = "Revenue predictibility")]
    pub revenue_predictibility: FloatOrString,
    #[serde(rename = "Enterprise Value (Current M)")]
    pub enterprise_value_current_m: FloatOrString,
    #[serde(rename = "Interest Coverage (10y Low)")]
    pub interest_coverage_10y_low: FloatOrString,
    #[serde(rename = "Currency")]
    pub currency: FloatOrString,
    #[serde(rename = "Equity-to-Asset (10y Low)")]
    pub equity_to_asset_10y_low: FloatOrString,
    #[serde(rename = "Return-on-Tangible-Asset")]
    pub return_on_tangible_asset: FloatOrString,
    #[serde(rename = "Return-on-Tangible-Equity (10y High)")]
    pub return_on_tangible_equity_10y_high: FloatOrString,
    #[serde(rename = "Quick Ratio (10y Low)")]
    pub quick_ratio_10y_low: FloatOrString,
    #[serde(rename = "Days Payable (10y Median)")]
    pub days_payable_10y_median: FloatOrString,
    #[serde(rename = "Net Cash per Share")]
    pub net_cash_per_share: FloatOrString,
    #[serde(rename = "ROC (Joel Greenblatt) % (10y High)")]
    pub roc_joel_greenblatt_pct_10y_high: FloatOrString,
    #[serde(rename = "Gross-Profit-to-Asset %")]
    pub gross_profit_to_asset_pct: FloatOrString,
    #[serde(rename = "Trailing 12-Month Revenue")]
    pub trailing_12_month_revenue: FloatOrString,
    #[serde(rename = "Primary Exchange")]
    pub primary_exchange: FloatOrString,
    #[serde(rename = "Debt-to-Asset")]
    pub debt_to_asset: FloatOrString,
    #[serde(rename = "ROIC % (10y Low)")]
    pub roic_pct_10y_low: FloatOrString,
    #[serde(rename = "Return-on-Tangible-Asset (10y High)")]
    pub return_on_tangible_asset_10y_high: FloatOrString,
    #[serde(rename = "ROA % (10y Low)")]
    pub roa_pct_10y_low: FloatOrString,
    #[serde(rename = "Predictability Rank")]
    pub predictability_rank: FloatOrString,
    #[serde(rename = "Cash Flow from Operations Direct Method")]
    pub cash_flow_from_operations_direct_method: FloatOrString,
    #[serde(rename = "Equity-to-Asset")]
    pub equity_to_asset: FloatOrString,
    #[serde(rename = "Primary Symbol")]
    pub primary_symbol: FloatOrString,
    #[serde(rename = "Piotroski F-Score (10y High)")]
    pub piotroski_f_score_10y_high: FloatOrString,
    #[serde(rename = "Days Inventory (10y Low)")]
    pub days_inventory_10y_low: FloatOrString,
    #[serde(rename = "Days Payable (10y Low)")]
    pub days_payable_10y_low: FloatOrString,
    #[serde(rename = "Inventory-to-Revenue")]
    pub inventory_to_revenue: FloatOrString,
    #[serde(rename = "Book Value per Share")]
    pub book_value_per_share: FloatOrString,
    #[serde(rename = "Cash-to-Debt")]
    pub cash_to_debt: FloatOrString,
    #[serde(rename = "Good Signs")]
    pub good_signs: FloatOrString,
    #[serde(rename = "Institutional Ownership")]
    pub institutional_ownership: FloatOrString,
    #[serde(rename = "Cash Conversion Cycle")]
    pub cash_conversion_cycle: FloatOrString,
    #[serde(rename = "ROC (Joel Greenblatt) % (5y Median)")]
    pub roc_joel_greenblatt_pct_5y_median: FloatOrString,
    #[serde(rename = "Related Company")]
    pub related_company: FloatOrString,
    #[serde(rename = "ROC (Joel Greenblatt) %")]
    pub roc_joel_greenblatt_pct: FloatOrString,
    #[serde(rename = "3-Year Average Share Buyback Ratio (10y Low)")]
    pub _3_year_average_share_buyback_ratio_10y_low: FloatOrString,
    #[serde(rename = "Effective Interest Rate on Debt %")]
    pub effective_interest_rate_on_debt_pct: FloatOrString,
    #[serde(rename = "Next Dividend Amount")]
    pub next_dividend_amount: FloatOrString,
    #[serde(rename = "ROE % (10y Median)")]
    pub roe_pct_10y_median: FloatOrString,
    #[serde(rename = "Optionable Stock")]
    pub optionable_stock: FloatOrString,
    #[serde(rename = "Profitability")]
    pub profitability: FloatOrString,
    #[serde(rename = "Share Class Description")]
    pub share_class_description: FloatOrString,
    #[serde(rename = "Trailing 12-Month Gross Profit")]
    pub trailing_12_month_gross_profit: FloatOrString,
    #[serde(rename = "ROC (ROIC) %")]
    pub roc_roic_pct: FloatOrString,
    #[serde(rename = "Altman Z-Score")]
    pub altman_z_score: FloatOrString,
    #[serde(rename = "Quick Ratio")]
    pub quick_ratio: FloatOrString,
    #[serde(rename = "Currency for Estimated Value")]
    pub currency_for_estimated_value: FloatOrString,
    #[serde(rename = "Current Ratio (10y Median)")]
    pub current_ratio_10y_median: FloatOrString,
    #[serde(rename = "Days Sales Outstanding (10y Median)")]
    pub days_sales_outstanding_10y_median: FloatOrString,
    #[serde(rename = "ROA % (10y Median)")]
    pub roa_pct_10y_median: FloatOrString,
    #[serde(rename = "ROC (Joel Greenblatt) % (10y Median)")]
    pub roc_joel_greenblatt_pct_10y_median: FloatOrString,
    #[serde(rename = "Interest Coverage")]
    pub interest_coverage: FloatOrString,
    #[serde(rename = "3-Year Share Buyback Rate")]
    pub _3_year_share_buyback_rate: FloatOrString,
    #[serde(rename = "Return-on-Tangible-Asset (10y Median)")]
    pub return_on_tangible_asset_10y_median: FloatOrString,
    #[serde(rename = "Debt-to-EBITDA")]
    pub debt_to_ebitda: FloatOrString,
    #[serde(rename = "Debt-to-Equity")]
    pub debt_to_equity: FloatOrString,
    #[serde(rename = "SIC")]
    pub sic: FloatOrString,
    #[serde(rename = "Altman Z-Score (10y High)")]
    pub altman_z_score_10y_high: FloatOrString,
    #[serde(rename = "Debt-to-EBITDA (10y Low)")]
    pub debt_to_ebitda_10y_low: FloatOrString,
    #[serde(rename = "Days Inventory")]
    pub days_inventory: FloatOrString,
    #[serde(rename = "Interest Coverage (10y High)")]
    pub interest_coverage_10y_high: FloatOrString,
    #[serde(rename = "NAICS")]
    pub naics: FloatOrString,
    #[serde(rename = "ROIC % (10y High)")]
    pub roic_pct_10y_high: FloatOrString,
}

/// Container for analyst estimates for all periods
#[derive(Deserialize, Debug)]
pub struct AnalystEstimates {
    pub annual: AnnualAnalystEstimate,
    pub quarter: QuarterlyAnalystEstimate,
}

/// Container for analyst estimates for annual periods
#[derive(Deserialize, Debug)]
pub struct AnnualAnalystEstimate {
    pub long_term_growth_rate_mean: FloatOrString,
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
