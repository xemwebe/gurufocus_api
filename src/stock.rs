use serde::Deserialize;
use std::collections::HashMap;
pub use crate::strnum::FloatOrString;
pub use crate::hexnum::HexNum;

/// Container for basic data for a single stock
#[derive(Deserialize, Debug)]
pub struct Stock {
    /// Name of the company
    pub company: String,
    /// Currency in which the stock is quoted
    pub currency: String,
    /// ID of the exchange the stock is quoted ad
    pub exchange: String,
    /// Main industry the company operates in
    pub industry: String,
    /// Industry sector the company belongs to
    pub sector: String,
    /// Main subindustry the company operates in
    pub subindustry: String,
    /// Stock ticker symbol
    pub symbol: String,
}

/// Container for single quote data
#[derive(Deserialize, Debug)]
pub struct Quote {
    /// Currency of quoted stock price
    #[serde(rename = "Currency")]
    pub currency: String,
    /// Current day's price change
    #[serde(rename = "Day\'s Change")]
    pub todays_change: FloatOrString,
    /// Current day's trading volume
    #[serde(rename = "Day\'s Volume")]
    pub todays_volume: FloatOrString,
    /// Exchange's symbol
    #[serde(rename = "Exchange")]
    pub exchange: String,
    /// Last quoted price
    #[serde(rename = "Price")]
    pub price: FloatOrString,
    /// Last price change
    #[serde(rename = "Price Change")]
    pub price_change: FloatOrString,
    /// Time of last price update
    #[serde(rename = "Price Updated Time")]
    pub update_time: String,
    /// Stock ticker symbol
    #[serde(rename = "Symbol")]
    pub symbol: String,
    /// Today's highest price
    pub high: FloatOrString,
    /// Today's lowest price
    pub low: FloatOrString,
    /// Today's opening price
    pub open: FloatOrString,
    /// Timestamp
    pub timestamp: i64,
}



/// Container for summary of basic stock data and valuation ratios
#[derive(Deserialize, Debug)]
pub struct StockSummary {
    pub summary: StockSummaryByCat,
}


/// Container for summary of basic stock data by category
#[derive(Deserialize, Debug)]
pub struct StockSummaryByCat {
    pub guru: HashMap<String, GuruTransaction>,
    pub insider: HashMap<String, InsiderTransaction>,
    pub ratio: RatioSummary,
    pub chart: Chart,
    pub company_data: CompanyData,
    pub estimate: Estimate,
    pub general: GeneralData,
}

/// Container for transaction done by Gurus
#[derive(Deserialize, Debug)]
pub struct GuruTransaction {
    pub buy: i64,
    pub hold: i64,
    pub new_buy: i64,
    pub sell: i64,
    pub sold_out: i64,
}

/// Container for transaction done by Insiders
#[derive(Deserialize, Debug)]
pub struct InsiderTransaction {
    pub buy: FloatOrString,
    pub sell: FloatOrString,
    pub price: FloatOrString,
}

/// Ratio summary
#[derive(Deserialize, Debug)]
pub struct RatioSummary {
    #[serde(rename = "ROE (%)")]
    pub roe_pct: RatioCmp,
    #[serde(rename = "EBITDA Growth (%)")]
    pub ebitda_growth_pct: RatioCmp,
    #[serde(rename = "PEG")]
    pub peg: RatioCmp,
    #[serde(rename = "Equity to Asset")]
    pub equity_to_asset: RatioCmp,
    #[serde(rename = "PFCF")]
    pub pfcf: RatioCmp,
    #[serde(rename = "Price/DCF (Projected)")]
    pub pricedcf_projected: RatioCmp,
    #[serde(rename = "Quick Ratio")]
    pub quick_ratio: RatioCmp,
    #[serde(rename = "mscore")]
    pub mscore: RatioCmp,
    #[serde(rename = "Price/DCF")]
    pub pricedcf: RatioCmp,
    #[serde(rename = "zscore")]
    pub zscore: RatioCmp,
    #[serde(rename = "P/S")]
    pub ps: RatioCmp,
    #[serde(rename = "F-Score")]
    pub f_score: RatioCmp,
    #[serde(rename = "Price/Tangible Book")]
    pub pricetangible_book: RatioCmp,
    #[serde(rename = "Interest Coverage")]
    pub interest_coverage: RatioCmp,
    #[serde(rename = "Price/Median PS Value")]
    pub pricemedian_ps_value: RatioCmp,
    #[serde(rename = "EV-to-EBIT")]
    pub ev_to_ebit: RatioCmp,
    #[serde(rename = "Earnings Yield (Greenblatt)")]
    pub earnings_yield_greenblatt: RatioCmp,
    #[serde(rename = "Net-margin (%)")]
    pub net_margin_pct: RatioCmp,
    #[serde(rename = "ROC (Joel Greenblatt) (%)")]
    pub roc_joel_greenblatt_pct: RatioCmp,
    #[serde(rename = "P/E(ttm)")]
    pub pettm: RatioCmp,
    #[serde(rename = "Share Buyback Rate")]
    pub share_buyback_rate: RatioCmp,
    #[serde(rename = "EPS Growth (%)")]
    pub eps_growth_pct: RatioCmp,
    #[serde(rename = "Price/Peter Lynch Fair Value")]
    pub pricepeter_lynch_fair_value: RatioCmp,
    #[serde(rename = "fscore")]
    pub fscore: RatioCmp,
    #[serde(rename = "Operating margin (%)")]
    pub operating_margin_pct: RatioCmp,
    #[serde(rename = "Yield on cost (5-Year)")]
    pub yield_on_cost_5_year: RatioCmp,
    #[serde(rename = "Dividend Payout")]
    pub dividend_payout: RatioCmp,
    #[serde(rename = "Forward P/E")]
    pub forward_pe: RatioCmp,
    #[serde(rename = "PE(NRI)")]
    pub penri: RatioCmp,
    #[serde(rename = "P/B")]
    pub pb: RatioCmp,
    #[serde(rename = "Dividend growth (3y)")]
    pub dividend_growth_3y: RatioCmp,
    #[serde(rename = "Price/Graham Number")]
    pub pricegraham_number: RatioCmp,
    #[serde(rename = "Price/Net Cash")]
    pub pricenet_cash: RatioCmp,
    #[serde(rename = "")]
    pub unknown: RatioCmp,
    #[serde(rename = "Revenue Growth (%)")]
    pub revenue_growth_pct: RatioCmp,
    #[serde(rename = "Shiller P/E")]
    pub shiller_pe: RatioCmp,
    #[serde(rename = "Current Ratio")]
    pub current_ratio: RatioCmp,
    #[serde(rename = "Forward Rate of Return (Yacktman)")]
    pub forward_rate_of_return_yacktman: RatioCmp,
    #[serde(rename = "Cash to Debt")]
    pub cash_to_debt: RatioCmp,
    #[serde(rename = "ROA (%)")]
    pub roa_pct: RatioCmp,
    #[serde(rename = "Price/Net Current Asset Value")]
    pub pricenet_current_asset_value: RatioCmp,
    #[serde(rename = "Dividend Yield")]
    pub dividend_yield: RatioCmp,
}

/// Chart
#[derive(Deserialize, Debug)]
pub struct Chart {
    #[serde(rename = "Graham Number")]
    pub graham_number: FloatOrString,
    #[serde(rename = "Median P/S Value")]
    pub median_ps_value: FloatOrString,
    #[serde(rename = "Peter Lynch Value")]
    pub peter_lynch_value: FloatOrString,
    #[serde(rename = "Net Current Asset Value")]
    pub net_current_asset_value: FloatOrString,
    #[serde(rename = "Net-Net Working Capital")]
    pub net_net_working_capital: FloatOrString,
    #[serde(rename = "Projected FCF")]
    pub projected_fcf: FloatOrString,
    #[serde(rename = "Tangible Book")]
    pub tangible_book: FloatOrString,
}

/// CompanyData
#[derive(Deserialize, Debug)]
pub struct CompanyData {
    pub dayssalesoutstand_low: FloatOrString,
    pub pfcflow: FloatOrString,
    pub ev2rev: FloatOrString,
    #[serde(rename = "Owner_Earnings")]
    pub owner_earnings: FloatOrString,
    pub ebit_growth_5y: FloatOrString,
    #[serde(rename = "ROTE")]
    pub rote: FloatOrString,
    pub daysinventory_med: FloatOrString,
    pub psrange: RatioRange,
    pub exch: String,
    pub pchange_ytd: FloatOrString,
    pub ebitda_growth_3y: FloatOrString,
    pub price52whigh: FloatOrString,
    pub rank_value: FloatOrString,
    pub optionable_stock: String,
    pub cash_ebit: FloatOrString,
    pub cashflow_growth_3y: FloatOrString,
    #[serde(rename = "p2iv_dcEarning_med")]
    pub p2iv_dcearning_med: FloatOrString,
    pub book_growth_5y: FloatOrString,
    pub ev2ebitdalow: FloatOrString,
    pub fscore_med: FloatOrString,
    #[serde(rename = "5yoffhigh")]
    pub _5yoffhigh: FloatOrString,
    pub equity2asset: FloatOrString,
    pub display_price: String,
    pub ebitda_growth_3y_low: FloatOrString,
    pub delist_reason: String,
    pub volumn_day: FloatOrString,
    pub ebit_q4_growth: FloatOrString,
    pub pshigh: FloatOrString,
    #[serde(rename = "ROTE_low")]
    pub rote_low: FloatOrString,
    pub book_growth_3y_high: FloatOrString,
    pub p2ncav_high: FloatOrString,
    pub p_change: FloatOrString,
    pub dividend_growth_1y: FloatOrString,
    pub industry_detail: IndustryDetails,
    pub currency_secr: String,
    pub similar_cap: Vec<String>,
    pub high: FloatOrString,
    pub similar_yield: Vec<String>,
    pub ev_morn_norm: FloatOrString,
    pub discountdcf: FloatOrString,
    pub p2medpsvalue_low: FloatOrString,
    pub penrihigh: FloatOrString,
    pub price52wlow: FloatOrString,
    pub total_ebitda_growth_3y_med: FloatOrString,
    pub zscore_detail: String,
    pub sales: FloatOrString,
    pub grossmargin_low: FloatOrString,
    #[serde(rename = "pchangeSP_12w")]
    pub pchangesp_12w: FloatOrString,
    pub roa_med: FloatOrString,
    pub ev2revlow: FloatOrString,
    pub short_ratio: FloatOrString,
    pub sma_200: FloatOrString,
    pub book_growth_1y: FloatOrString,
    pub total_buyback_3y: FloatOrString,
    pub yield_on_cost_low: FloatOrString,
    pub e10: FloatOrString,
    pub p2tangible_book_med: FloatOrString,
    pub stock_dynamic_data: StockDynamics,
    pub pchange_12w: FloatOrString,
    pub roa: FloatOrString,
    pub book_growth_3y: FloatOrString,
    #[serde(rename = "pchangeSP_24w")]
    pub pchangesp_24w: FloatOrString,
    pub debt2rev_growth_1y: FloatOrString,
    pub dayspayable_low: FloatOrString,
    pub asset: FloatOrString,
    pub p2medpbvalue: FloatOrString,
    pub ebitda_growth_5y: FloatOrString,
    pub sector_code: FloatOrString,
    #[serde(rename = "SGA")]
    pub sga: FloatOrString,
    pub rank_history: FloatOrString,
    pub grossprofit: FloatOrString,
    pub cash: FloatOrString,
    #[serde(rename = "FCFmargin_med_5y")]
    pub fcfmargin_med_5y: FloatOrString,
    pub display_symbol: String,
    pub p2iv_dcf: FloatOrString,
    pub p2val_avg: FloatOrString,
    pub debt2rev: FloatOrString,
    pub ev2ebitdamed: FloatOrString,
    pub pb: FloatOrString,
    #[serde(rename = "pchangeSP_4w")]
    pub pchangesp_4w: FloatOrString,
    #[serde(rename = "1")]
    pub _1: FloatOrString,
    pub price52wrange: RatioRange,
    pub rsquare_pb: FloatOrString,
    pub low: FloatOrString,
    pub country: String,
    #[serde(rename = "EPV")]
    pub epv: FloatOrString,
    #[serde(rename = "ROTE_med")]
    pub rote_med: FloatOrString,
    pub cusip: String,
    pub ebitda_growth: FloatOrString,
    pub beta: FloatOrString,
    pub rank: FloatOrString,
    pub current_ratio_med: FloatOrString,
    #[serde(rename = "ROC_JOEL_growth")]
    pub roc_joel_growth: FloatOrString,
    pub pettm: FloatOrString,
    pub sp: FloatOrString,
    #[serde(rename = "ROTA_high")]
    pub rota_high: FloatOrString,
    pub total_buyback_3y_high: FloatOrString,
    pub fscore_detail: String,
    pub payout_low: FloatOrString,
    pub asset_growth_3y: FloatOrString,
    pub p2iv_dcf_low: FloatOrString,
    pub pfcf: FloatOrString,
    #[serde(rename = "IsDirect")]
    pub isdirect: String,
    pub yield_on_cost_med: FloatOrString,
    pub ebit_growth_3y_low: FloatOrString,
    pub use_in_screen: FloatOrString,
    pub funda_time: i64,
    #[serde(rename = "NAICS")]
    pub naics: FloatOrString,
    #[serde(rename = "pchangeSP_1w")]
    pub pchangesp_1w: FloatOrString,
    pub pebit_med: FloatOrString,
    pub shareso: FloatOrString,
    pub total_ebitda_growth_3y: FloatOrString,
    pub dividend_growth_10y: FloatOrString,
    pub projected_pb: FloatOrString,
    pub currency_comp: String,
    pub debt2equity: FloatOrString,
    pub p2net_cash_high: FloatOrString,
    pub quick_ratio_high: FloatOrString,
    pub p2medpbvalue_med: FloatOrString,
    pub quick_ratio_med: FloatOrString,
    pub offhigh: FloatOrString,
    pub asset_growth_5y: FloatOrString,
    pub pettmlow: FloatOrString,
    pub total_rvn_growth_1y: FloatOrString,
    pub p2net_cash_low: FloatOrString,
    pub yield_on_cost_high: FloatOrString,
    pub asset_growth_1y: FloatOrString,
    pub price3yhigh: FloatOrString,
    pub operatingmargin_growth: FloatOrString,
    pub total_netincome_growth_10y: FloatOrString,
    #[serde(rename = "52woffhigh")]
    pub _52woffhigh: FloatOrString,
    pub ev2ebitmed: FloatOrString,
    pub total_buyback_3y_low: FloatOrString,
    pub dayssalesoutstand_high: FloatOrString,
    pub p2lynchvalue_high: FloatOrString,
    #[serde(rename = "ShortInterest")]
    pub shortinterest: FloatOrString,
    pub ev2ebitda: FloatOrString,
    pub total_rvn_growth_3y_high: FloatOrString,
    pub medpbvalue: FloatOrString,
    pub revenue: FloatOrString,
    pub total_rvn_growth_3y_med: FloatOrString,
    pub ebit_growth_10y: FloatOrString,
    pub total_netincome_growth_3y_low: FloatOrString,
    pub book_growth_3y_low: FloatOrString,
    pub fscore: FloatOrString,
    pub projected_ps: FloatOrString,
    pub ev2ebithigh: FloatOrString,
    pub asset_growth_3y_high: FloatOrString,
    pub company: String,
    pub ev2ebitlow: FloatOrString,
    pub cash2debt_high: FloatOrString,
    pub eps_nri: FloatOrString,
    pub p2tangible_book: FloatOrString,
    pub yield_low: FloatOrString,
    pub p2iv_dcf_share_med: FloatOrString,
    pub debt2asset: FloatOrString,
    pub roic_high: FloatOrString,
    pub buyback: FloatOrString,
    pub medpsvalue: FloatOrString,
    #[serde(rename = "ROC_JOEL")]
    pub roc_joel: FloatOrString,
    pub ncav_real: FloatOrString,
    pub pocfmed: FloatOrString,
    pub price5yhigh: FloatOrString,
    pub roa_low: FloatOrString,
    pub tangible_book: FloatOrString,
    pub ttm_eps: FloatOrString,
    pub competitor: Vec<String>,
    pub predictability: FloatOrString,
    pub ncav: FloatOrString,
    pub roa_high: FloatOrString,
    pub valuation_avg: FloatOrString,
    pub total_netincome_growth_3y: FloatOrString,
    pub rvn_growth_1y: FloatOrString,
    pub total_buyback_3y_med: FloatOrString,
    pub pchange_4w: FloatOrString,
    pub currency_est: String,
    pub next_dividend_payment_date: String,
    #[serde(rename = "ttm_EBIT")]
    pub ttm_ebit: FloatOrString,
    pub similar_sales: Vec<String>,
    pub report_frequency: String,
    #[serde(rename = "ROTA_med")]
    pub rota_med: FloatOrString,
    pub rank_quality: FloatOrString,
    #[serde(rename = "ttm_EBITDA")]
    pub ttm_ebitda: FloatOrString,
    pub ev2pretaxincome: FloatOrString,
    pub float: FloatOrString,
    pub book: FloatOrString,
    pub penrimed: FloatOrString,
    pub pchange_52w: FloatOrString,
    pub ev2revhigh: FloatOrString,
    pub debt2equity_med: FloatOrString,
    pub pbelow52whigh: FloatOrString,
    pub dayspayable: FloatOrString,
    pub fscore_low: FloatOrString,
    #[serde(rename = "p2Owner_Earnings_high")]
    pub p2owner_earnings_high: FloatOrString,
    pub next_dividend_amount: String,
    pub oprt_margain_med: FloatOrString,
    #[serde(rename = "p2Owner_Earnings")]
    pub p2owner_earnings: FloatOrString,
    pub price5ylow: FloatOrString,
    pub exchange_symbol: String,
    #[serde(rename = "ROTA")]
    pub rota: FloatOrString,
    pub ebit_q3_growth: FloatOrString,
    pub p2lynchvalue_med: FloatOrString,
    pub net_margain_high: FloatOrString,
    pub p2tangible_book_low: FloatOrString,
    pub pabove52wlow: FloatOrString,
    pub rvn_predc_10y: FloatOrString,
    pub total_ebitda_growth_1y: FloatOrString,
    pub dayspayable_med: FloatOrString,
    pub shares: FloatOrString,
    pub ttm_pretaxincome: FloatOrString,
    #[serde(rename = "p2iv_dcEarning_low")]
    pub p2iv_dcearning_low: FloatOrString,
    pub cap: FloatOrString,
    pub mktcap_chg_1y: FloatOrString,
    pub past_earnings_date: String,
    pub interest_coverage_med: FloatOrString,
    pub interest_coverage_high: FloatOrString,
    pub pettmmed: FloatOrString,
    pub pe: FloatOrString,
    pub quotetime: FloatOrString,
    pub volumn_day_total: FloatOrString,
    pub cik: String,
    pub daysinventory_high: FloatOrString,
    pub asset_growth_10y: FloatOrString,
    #[serde(rename = "p2iv_dcEarning")]
    pub p2iv_dcearning: FloatOrString,
    pub warning_details: HashMap<String, WarningDetails>,
    #[serde(rename = "ShillerPE_low")]
    pub shillerpe_low: FloatOrString,
    pub price10yhigh: FloatOrString,
    pub cash2debt_low: FloatOrString,
    pub sloanratio: FloatOrString,
    pub rsquare_pebitda: FloatOrString,
    pub currency: String,
    #[serde(rename = "IPO_date")]
    pub ipo_date: String,
    pub cashflow_growth_3y_med: FloatOrString,
    pub rvn_growth_3y_low: FloatOrString,
    #[serde(rename = "NetInterestMargin")]
    pub netinterestmargin: FloatOrString,
    pub ebitda_growth_3y_high: FloatOrString,
    #[serde(rename = "p2Owner_Earnings_low")]
    pub p2owner_earnings_low: FloatOrString,
    pub pebitda_med: FloatOrString,
    #[serde(rename = "Logit")]
    pub logit: FloatOrString,
    pub zscore_low: FloatOrString,
    pub payout_high: FloatOrString,
    pub latest_quarter: String,
    pub fscore_chg_1y: FloatOrString,
    pub p2iv_dcf_med: FloatOrString,
    pub rdcf_growth: FloatOrString,
    pub roic: FloatOrString,
    pub peglow: FloatOrString,
    pub pretax_margain: FloatOrString,
    pub isin: String,
    pub dividend_freq: FloatOrString,
    pub roe_high: FloatOrString,
    pub ccc: FloatOrString,
    pub roe_low: FloatOrString,
    pub country_name: String,
    pub rvn_growth_5y: FloatOrString,
    pub pslow: FloatOrString,
    pub dividend: FloatOrString,
    pub roa_med_5y: FloatOrString,
    pub p2lynchvalue_low: FloatOrString,
    pub next_earnings_date: String,
    pub current_ratio_low: FloatOrString,
    #[serde(rename = "TotalPayoutRatio")]
    pub totalpayoutratio: FloatOrString,
    pub p2iv_dcf_share: FloatOrString,
    pub projected_correlation: FloatOrString,
    pub pbrange: RatioRange,
    pub total_buyback_5y: FloatOrString,
    pub ev: FloatOrString,
    pub ttm_dividend: FloatOrString,
    pub net_margain_low: FloatOrString,
    #[serde(rename = "InsiderSharesOwned")]
    pub insidersharesowned: FloatOrString,
    pub earning_growth_3y_low: FloatOrString,
    pub payout_med: FloatOrString,
    pub total_netincome_growth_1y: FloatOrString,
    pub cashflow_growth_5y: FloatOrString,
    pub similar_pe: Vec<String>,
    pub mscore_detail: String,
    pub dayspayable_high: FloatOrString,
    pub p2grahamnumber_med: FloatOrString,
    #[serde(rename = "type")]
    pub company_type: String,
    pub earning_yield_greenblatt_high: FloatOrString,
    pub inventory2sales: FloatOrString,
    pub quick_ratio: FloatOrString,
    pub pchange_10y: FloatOrString,
    pub grossmargin_growth: FloatOrString,
    pub payout_range: RatioRange,
    pub rank_value_global: FloatOrString,
    pub roic_med: FloatOrString,
    pub rank_industry: FloatOrString,
    pub debt2ebitda_low: FloatOrString,
    pub ebit_q1_growth: FloatOrString,
    pub fiscal_year_end: FloatOrString,
    pub net_cash: FloatOrString,
    pub p2medpsvalue_high: FloatOrString,
    pub rsi_14: FloatOrString,
    pub iv_dcf_normalized_proj_growth: FloatOrString,
    pub sma_50: FloatOrString,
    pub total_rvn_growth_5y: FloatOrString,
    pub dividend_date: String,
    #[serde(rename = "ROC_JOEL_med")]
    pub roc_joel_med: FloatOrString,
    pub sma_20: FloatOrString,
    pub earning_growth_10y: FloatOrString,
    pub id: i64,
    pub delist_date: String,
    #[serde(rename = "ShillerPE_high")]
    pub shillerpe_high: FloatOrString,
    pub mscore_med: FloatOrString,
    pub total_netincome_growth_3y_med: FloatOrString,
    pub dayssalesoutstand: FloatOrString,
    pub valuee: FloatOrString,
    pub good_details: FloatOrString,
    pub forward_sales: FloatOrString,
    pub daysinventory: FloatOrString,
    pub last_price_date: String,
    pub dividend_startyear: FloatOrString,
    pub iv_dcf_share: FloatOrString,
    pub rsquare_penri: FloatOrString,
    pub volume: FloatOrString,
    pub total_netincome_growth_5y: FloatOrString,
    pub volume_total: FloatOrString,
    #[serde(rename = "ROC_JOEL_high")]
    pub roc_joel_high: FloatOrString,
    pub earning_yield_greenblatt_med: FloatOrString,
    pub insider_update: FloatOrString,
    pub peghigh: FloatOrString,
    pub display_symbol_region: String,
    #[serde(rename = "InstitutionSharesHeld")]
    pub institutionsharesheld: FloatOrString,
    pub zscore_med: FloatOrString,
    pub open: FloatOrString,
    pub y_yield: FloatOrString,
    #[serde(rename = "SIC")]
    pub sic: i64,
    pub earning_growth_3y: FloatOrString,
    pub ltd_chg_1y: FloatOrString,
    pub earning_growth_1y: FloatOrString,
    pub earning_growth_5y: FloatOrString,
    pub class_descpt: String,
    pub ttm_eps_nri: FloatOrString,
    pub roe: FloatOrString,
    #[serde(rename = "ShillerPE_med")]
    pub shillerpe_med: FloatOrString,
    pub price: FloatOrString,
    pub grossprofit2asset: FloatOrString,
    pub oprt_margain_high: FloatOrString,
    pub daysinventory_low: FloatOrString,
    #[serde(rename = "Dividend2FFO")]
    pub dividend2ffo: FloatOrString,
    pub yield_on_cost: FloatOrString,
    pub mktcap: FloatOrString,
    #[serde(rename = "yield")]
    pub yield_: FloatOrString,
    pub group: String,
    pub rank_predictability: FloatOrString,
    pub dividend_firstyear: FloatOrString,
    pub grossmargin_med: FloatOrString,
    pub pchange_24w: FloatOrString,
    pub p2ncav_low: FloatOrString,
    pub pfcfhigh: FloatOrString,
    pub debt2rev_growth_5y: FloatOrString,
    pub dividend_growth_3y_high: FloatOrString,
    pub oprt_margain_med_5y: FloatOrString,
    #[serde(rename = "pchangeSP_52w")]
    pub pchangesp_52w: FloatOrString,
    pub above52wlow: String,
    pub yield_range: RatioRange,
    pub pocflow: FloatOrString,
    pub p2tangible_book_high: FloatOrString,
    #[serde(rename = "FCFyield")]
    pub fcfyield: FloatOrString,
    pub p2net_cash_med: FloatOrString,
    pub book_growth_10y: FloatOrString,
    pub roe_adj: FloatOrString,
    pub cash2debt_med: FloatOrString,
    pub cashflow: FloatOrString,
    pub currency_symbol: String,
    pub earning_yield: FloatOrString,
    pub profit_year_num: FloatOrString,
    pub projected_penri: FloatOrString,
    pub wacc: FloatOrString,
    pub tax_rate_med_5y: FloatOrString,
    pub gava: FloatOrString,
    #[serde(rename = "ForwardDividendYield")]
    pub forwarddividendyield: FloatOrString,
    pub p2grahamnumber_high: FloatOrString,
    pub rvn_growth_3y_high: FloatOrString,
    pub cashflow_growth_1y: FloatOrString,
    pub hard_asset: FloatOrString,
    pub is_otc: FloatOrString,
    pub earning_growth_3y_high: FloatOrString,
    pub display_timestamp: String,
    #[serde(rename = "forwardPE")]
    pub forwardpe: FloatOrString,
    pub p2iv_dcf_share_high: FloatOrString,
    pub psmed: FloatOrString,
    pub traded_countries: Vec<Country>,
    pub country_iso: String,
    pub mktcap_chg_3y: FloatOrString,
    #[serde(rename = "p2EPV")]
    pub p2epv: FloatOrString,
    #[serde(rename = "iv_dcEarning")]
    pub iv_dcearning: FloatOrString,
    pub total_ebitda_growth_10y: FloatOrString,
    pub total_netincome_growth_3y_high: FloatOrString,
    pub yield_med: FloatOrString,
    pub roe_med: FloatOrString,
    pub debt2equity_high: FloatOrString,
    pub zscore_high: FloatOrString,
    pub perange: RatioRange,
    pub dayssalesoutstand_med: FloatOrString,
    #[serde(rename = "ROC_JOEL_med_5y")]
    pub roc_joel_med_5y: FloatOrString,
    pub related_comp: String,
    pub meta: String,
    pub grossmargin: FloatOrString,
    pub p2nnwc: FloatOrString,
    pub description: CompanyDescription,
    pub primary_symbol: String,
    pub book_predict_10y: FloatOrString,
    pub total_buyback_10y: FloatOrString,
    pub book_growth_3y_med: FloatOrString,
    pub share_growth_10y: FloatOrString,
    pub insider_buy: FloatOrString,
    pub projected_return: FloatOrString,
    pub ps: FloatOrString,
    #[serde(rename = "EffectiveInterestRate")]
    pub effectiveinterestrate: FloatOrString,
    pub pretax_margain_low: FloatOrString,
    pub penri: FloatOrString,
    pub earning_yield_greenblatt: FloatOrString,
    #[serde(rename = "RateOfReturn_high")]
    pub rateofreturn_high: FloatOrString,
    pub dividend_growth_3y: FloatOrString,
    pub goodwill2asset: FloatOrString,
    pub roic_low: FloatOrString,
    #[serde(rename = "RateOfReturn_low")]
    pub rateofreturn_low: FloatOrString,
    pub ttm_sales: FloatOrString,
    #[serde(rename = "2")]
    pub _2: FloatOrString,
    pub inst_ownership: FloatOrString,
    #[serde(rename = "pbmed")]
    pub pbmed: FloatOrString,
    #[serde(rename = "ROTA_low")]
    pub rota_low: FloatOrString,
    #[serde(rename = "price3ylow")]
    pub price3ylow: FloatOrString,
    #[serde(rename = "region")]
    pub region: String,
    #[serde(rename = "mscore_high")]
    pub mscore_high: FloatOrString,
    #[serde(rename = "eps")]
    pub eps: FloatOrString,
    #[serde(rename = "ROTE_high")]
    pub rote_high: FloatOrString,
    #[serde(rename = "num_analyst")]
    pub num_analyst: FloatOrString,
    #[serde(rename = "pchange_3y")]
    pub pchange_3y: FloatOrString,
    #[serde(rename = "return5y")]
    pub return5y: FloatOrString,
    #[serde(rename = "discounte")]
    pub discounte: FloatOrString,
    #[serde(rename = "ShillerPE")]
    pub shillerpe: FloatOrString,
    #[serde(rename = "payout")]
    pub payout: FloatOrString,
    #[serde(rename = "earning_yield_greenblatt_low")]
    pub earning_yield_greenblatt_low: FloatOrString,
    #[serde(rename = "dividend_months")]
    pub dividend_months: String,
    #[serde(rename = "earning_growth_3y_med")]
    pub earning_growth_3y_med: FloatOrString,
    #[serde(rename = "p2iv_dcf_high")]
    pub p2iv_dcf_high: FloatOrString,
    #[serde(rename = "total_ebitda_growth_3y_high")]
    pub total_ebitda_growth_3y_high: FloatOrString,
    #[serde(rename = "valuedcf")]
    pub valuedcf: FloatOrString,
    #[serde(rename = "first5_egrowth")]
    pub first5_egrowth: FloatOrString,
    #[serde(rename = "oprt_margain")]
    pub oprt_margain: FloatOrString,
    #[serde(rename = "pchange_5y")]
    pub pchange_5y: FloatOrString,
    #[serde(rename = "similar_ps")]
    pub similar_ps: Vec<String>,
    #[serde(rename = "equity2asset_low")]
    pub equity2asset_low: FloatOrString,
    #[serde(rename = "equity2liab_tangbile")]
    pub equity2liab_tangbile: FloatOrString,
    #[serde(rename = "pocf")]
    pub pocf: FloatOrString,
    #[serde(rename = "cash2debt")]
    pub cash2debt: FloatOrString,
    #[serde(rename = "grossmargin_high")]
    pub grossmargin_high: FloatOrString,
    #[serde(rename = "dividend_growth_3y_med")]
    pub dividend_growth_3y_med: FloatOrString,
    #[serde(rename = "price10ylow")]
    pub price10ylow: FloatOrString,
    #[serde(rename = "roi")]
    pub roi: FloatOrString,
    #[serde(rename = "cashflow_growth_3y_low")]
    pub cashflow_growth_3y_low: FloatOrString,
    #[serde(rename = "penrilow")]
    pub penrilow: FloatOrString,
    #[serde(rename = "primary_ShareClass")]
    pub primary_share_class: String,
    pub roe_med_5y: FloatOrString,
    pub volatility: FloatOrString,
    pub num_good_signs: FloatOrString,
    pub pblow: FloatOrString,
    pub primary_exch: String,
    pub mktcap_norm: FloatOrString,
    pub current_ratio: FloatOrString,
    pub rank_financial_strength: FloatOrString,
    #[serde(rename = "PFD")]
    pub pfd: FloatOrString,
    pub roc: FloatOrString,
    pub rsquare_ps: FloatOrString,
    pub p2ncav_med: FloatOrString,
    pub pchange_1w: FloatOrString,
    pub sales_growth: FloatOrString,
    pub ebit_growth_1y: FloatOrString,
    pub industry_code: i64,
    pub zscore: FloatOrString,
    pub p2medpbvalue_low: FloatOrString,
    pub total_rvn_growth_3y_low: FloatOrString,
    #[serde(rename = "RateOfReturn_med")]
    pub rateofreturn_med: FloatOrString,
    pub sector: String,
    pub pbhigh: FloatOrString,
    pub roic_med_5y: FloatOrString,
    pub peg: FloatOrString,
    pub interest_coverage: FloatOrString,
    pub net_margain: FloatOrString,
    pub dividend_growth_5y: FloatOrString,
    pub quick_ratio_low: FloatOrString,
    pub pegmed: FloatOrString,
    pub insider_ownership: FloatOrString,
    pub ltd_chg_3y: FloatOrString,
    pub ptb: FloatOrString,
    pub p2iv_dcf_share_low: FloatOrString,
    pub priceindex_6m: FloatOrString,
    pub rvn_growth_10y: FloatOrString,
    pub cashflow_growth_3y_high: FloatOrString,
    pub pretax_margain_med: FloatOrString,
    pub rank_absolute: FloatOrString,
    pub data_complete: FloatOrString,
    pub exchange: FloatOrString,
    pub projected_pebitda: FloatOrString,
    pub last_10kq: FloatOrString,
    pub rank_balancesheet: FloatOrString,
    #[serde(rename = "ForwardDividend")]
    pub forwarddividend: FloatOrString,
    pub interest_coverage_low: FloatOrString,
    pub next_dividend_date: String,
    pub lynchvalue: FloatOrString,
    pub ind_template: String,
    pub num_warning_signs_severe: FloatOrString,
    pub predictability_rank: FloatOrString,
    pub ebitda_predc_10y: FloatOrString,
    pub iv_dcf: FloatOrString,
    pub oprt_margain_low: FloatOrString,
    pub ebitda_growth_1y: FloatOrString,
    pub cashflow_growth_10y: FloatOrString,
    pub ebit_q2_growth: FloatOrString,
    #[serde(rename = "p2Owner_Earnings_med")]
    pub p2owner_earnings_med: FloatOrString,
    pub p2medpsvalue: FloatOrString,
    pub ebitda_growth_10y: FloatOrString,
    #[serde(rename = "ShortPercentageOfSharesOutstanding")]
    pub shortpercentageofsharesoutstanding: FloatOrString,
    pub pcashflow: FloatOrString,
    pub pretax_margain_high: FloatOrString,
    pub roc_growth_5y: FloatOrString,
    pub total_ebitda_growth_3y_low: FloatOrString,
    pub yield_high: FloatOrString,
    pub total_rvn_growth_10y: FloatOrString,
    pub total_ebitda_growth_5y: FloatOrString,
    pub earning_growth_5y_est: FloatOrString,
    pub mscore: FloatOrString,
    pub symbol: String,
    #[serde(rename = "FCFmargin")]
    pub fcfmargin: FloatOrString,
    #[serde(rename = "ShortPercentageOfFloat")]
    pub shortpercentageoffloat: FloatOrString,
    pub second5_egrowth: FloatOrString,
    #[serde(rename = "p2iv_dcEarning_high")]
    pub p2iv_dcearning_high: FloatOrString,
    pub predictability_5y: FloatOrString,
    pub rvn_growth_3y_med: FloatOrString,
    pub fscore_high: FloatOrString,
    pub timestamp: i64,
    pub num_warning_signs_meidum: FloatOrString,
    pub pettmhigh: FloatOrString,
    pub p2grahamnumber_low: FloatOrString,
    pub display_country: String,
    #[serde(rename = "margin_dcEarning")]
    pub margin_dcearning: FloatOrString,
    pub total_rvn_growth_3y: FloatOrString,
    pub dividend_growth_3y_low: FloatOrString,
    pub asset_growth_3y_low: FloatOrString,
    pub grossprofit2tangibleasset: FloatOrString,
    pub p2grahamnumber: FloatOrString,
    pub ebitda_growth_3y_med: FloatOrString,
    pub delisted: FloatOrString,
    pub ebit_growth_3y_high: FloatOrString,
    pub p2ncav: FloatOrString,
    pub rank_quality_global: FloatOrString,
    pub group_code: i64,
    pub total_buyback_1y: FloatOrString,
    pub offlow: FloatOrString,
    #[serde(rename = "TotalPayoutYield")]
    pub totalpayoutyield: FloatOrString,
    pub p_pct_change: FloatOrString,
    pub equity2asset_med: FloatOrString,
    #[serde(rename = "ROC_JOEL_low")]
    pub roc_joel_low: FloatOrString,
    pub industry: String,
    pub mktcap_temp: FloatOrString,
    pub sedol: FloatOrString,
    pub debt2ebitda_high: FloatOrString,
    pub debt2ebitda_med: FloatOrString,
    pub analyst_opinion: String,
    pub p2medpbvalue_high: FloatOrString,
    pub debt2rev_growth_10y: FloatOrString,
    pub p2medpsvalue_med: FloatOrString,
    pub grahamnumber: FloatOrString,
    #[serde(rename = "FCFyield_med_5y")]
    pub fcfyield_med_5y: FloatOrString,
    pub rvn_growth_3y: FloatOrString,
    pub asset_growth_3y_med: FloatOrString,
    pub ev_morn: FloatOrString,
    pub iv_dcf_normalized: FloatOrString,
    pub ltd_chg_5y: FloatOrString,
    pub rank_profitability: FloatOrString,
    pub ebit_growth_3y: FloatOrString,
    #[serde(rename = "SNOA")]
    pub snoa: FloatOrString,
    pub ev2ebit: FloatOrString,
    pub pfcfmed: FloatOrString,
    pub is_spin_off: bool,
    #[serde(rename = "RateOfReturn")]
    pub rateofreturn: FloatOrString,
    pub mktcap_chg_5y: FloatOrString,
    pub debt2equity_low: FloatOrString,
    pub ev_temp: FloatOrString,
    pub mktwindustry: i64,
    pub mscore_low: FloatOrString,
    #[serde(rename = "pchangeSP_ytd")]
    pub pchangesp_ytd: FloatOrString,
    #[serde(rename = "3yoffhigh")]
    pub _3yoffhigh: FloatOrString,
    pub ebit_growth_3y_med: FloatOrString,
    pub current_ratio_high: FloatOrString,
    pub oprt_cashflow_12m: FloatOrString,
    pub equity2asset_high: FloatOrString,
    pub net_margain_med: FloatOrString,
    pub p2lynchvalue: FloatOrString,
    pub p2net_cash: FloatOrString,
    pub pocfhigh: FloatOrString,
    #[serde(rename = "0")]
    pub _0: String,
    pub debt2ebitda: FloatOrString,
    pub similar_pb: Vec<String>,
    pub ev2ebitdahigh: FloatOrString,
    pub ev2revmed: FloatOrString,
}

/// Estimate summary
#[derive(Deserialize, Debug)]
pub struct Estimate {
    #[serde(rename = "LongTermGrowthRateMean")]
    pub longtermgrowthratemean: FloatOrString,
    #[serde(rename = "eps_nri")]
    pub eps_nri: Vec<FloatOrString>,
    #[serde(rename = "count")]
    pub count: FloatOrString,
    #[serde(rename = "per share eps")]
    pub per_share_eps: Vec<FloatOrString>,
    #[serde(rename = "percentage")]
    pub percentage: FloatOrString,
    #[serde(rename = "Dividends Per Share")]
    pub dividends_per_share: Vec<FloatOrString>,
    #[serde(rename = "Revenue")]
    pub revenue: Vec<FloatOrString>,
    #[serde(rename = "quarter")]
    pub quarter: Vec<FloatOrString>,
}

/// General data for summary
#[derive(Deserialize, Debug)]
pub struct GeneralData {
    #[serde(rename = "industry")]
    pub industry: String,
    #[serde(rename = "company")]
    pub company: String,
    #[serde(rename = "desc")]
    pub desc: String,
    #[serde(rename = "rank_financial_strength")]
    pub rank_financial_strength: FloatOrString,
    #[serde(rename = "sector")]
    pub sector: String,
    #[serde(rename = "currency")]
    pub currency: String,
    #[serde(rename = "price")]
    pub price: FloatOrString,
    #[serde(rename = "short_desc")]
    pub short_desc: String,
    #[serde(rename = "rank_profitability")]
    pub rank_profitability: FloatOrString,
    #[serde(rename = "rating")]
    pub rating: FloatOrString,
    #[serde(rename = "country")]
    pub country: String,
    #[serde(rename = "group")]
    pub group: String,
    #[serde(rename = "timestamp")]
    pub timestamp: String,
}

/// Ratio comparison
#[derive(Deserialize, Debug)]
pub struct RatioCmp {
    pub his: HistoryCmp,
    pub indu: IndustryCmp,
    pub status: FloatOrString,
    pub value: FloatOrString,
}

/// Comparison to historic data
#[derive(Deserialize, Debug)]
pub struct HistoryCmp {
    pub high: FloatOrString,
    pub low: FloatOrString,
    #[serde(default)]
    pub med: FloatOrString,
}

/// Comparison to industry
#[derive(Deserialize, Debug)]
pub struct IndustryCmp {
    pub global_rank: FloatOrString,
    pub indu_med: FloatOrString,
    pub indu_tot: FloatOrString,
}


/// Description of range of some ratio
#[derive(Deserialize, Debug)]
pub struct RatioRange {
    pub color: HexNum,
    pub current: FloatOrString,
    pub high: FloatOrString,
    pub low: FloatOrString,
}


/// Details to a specific warning
#[derive(Deserialize, Debug)]
pub struct WarningDetails {
    pub category: String,
    pub degree: String,
    pub details: String,
    pub display: String,
    pub name: String,
}


/// Address and related company information
#[derive(Deserialize, Debug)]
pub struct CompanyDescription {
    pub address: String,
    pub descrpt: String,
    pub morn_comp_id: String,
    pub short_descript: String,
    pub symbol: String,
    pub website: String,
}


/// Country and exchange information
#[derive(Deserialize, Debug)]
pub struct Country {
    pub country: String,
    pub exchange: String,
    pub symbol: String,
}


/// Stock price change information
#[derive(Deserialize, Debug)]
pub struct StockDynamics {
    pub high: FloatOrString,
    pub low: FloatOrString,
    pub open: FloatOrString,
    pub p_change: FloatOrString,
    pub p_pct_change: FloatOrString,
    pub price: FloatOrString,
    pub stockid: String,
    pub volumn_day: FloatOrString,
}

/// Industry detail information
#[derive(Deserialize, Debug)]
pub struct IndustryDetails {
    pub etf: String,
    pub group: String,
    pub groupcode: i64,
    pub industry: String,
    pub industrycode: i64,
    #[serde(rename = "netMar10thPctile")]
    pub net_mar10th_pctile: FloatOrString,
    #[serde(rename = "netMar20thPctile")]
    pub net_mar20th_pctile: FloatOrString,
    #[serde(rename = "netMar30thPctile")]
    pub net_mar30th_pctile: FloatOrString,
    #[serde(rename = "netMar40thPctile")]
    pub net_mar40th_pctile: FloatOrString,
    #[serde(rename = "netMar50thPctile")]
    pub net_mar50th_pctile: FloatOrString,
    #[serde(rename = "netMar60thPctile")]
    pub net_mar60th_pctile: FloatOrString,
    #[serde(rename = "netMar70thPctile")]
    pub net_mar70th_pctile: FloatOrString,
    #[serde(rename = "netMar80thPctile")]
    pub net_mar80th_pctile: FloatOrString,
    #[serde(rename = "netMar90thPctile")]
    pub net_mar90th_pctile: FloatOrString,
    #[serde(rename = "oprtMar10thPctile")]
    pub oprt_mar10th_pctile: FloatOrString,
    #[serde(rename = "oprtMar20thPctile")]
    pub oprt_mar20th_pctile: FloatOrString,
    #[serde(rename = "oprtMar30thPctile")]
    pub oprt_mar30th_pctile: FloatOrString,
    #[serde(rename = "oprtMar40thPctile")]
    pub oprt_mar40th_pctile: FloatOrString,
    #[serde(rename = "oprtMar50thPctile")]
    pub oprt_mar50th_pctile: FloatOrString,
    #[serde(rename = "oprtMar60thPctile")]
    pub oprt_mar60th_pctile: FloatOrString,
    #[serde(rename = "oprtMar70thPctile")]
    pub oprt_mar70th_pctile: FloatOrString,
    #[serde(rename = "oprtMar80thPctile")]
    pub oprt_mar80th_pctile: FloatOrString,
    #[serde(rename = "oprtMar90thPctile")]
    pub oprt_mar90th_pctile: FloatOrString,
    #[serde(rename = "pb10thPctile")]
    pub pb10th_pctile: FloatOrString,
    #[serde(rename = "pb20thPctile")]
    pub pb20th_pctile: FloatOrString,
    #[serde(rename = "pb30thPctile")]
    pub pb30th_pctile: FloatOrString,
    #[serde(rename = "pb40thPctile")]
    pub pb40th_pctile: FloatOrString,
    #[serde(rename = "pb50thPctile")]
    pub pb50th_pctile: FloatOrString,
    #[serde(rename = "pb60thPctile")]
    pub pb60th_pctile: FloatOrString,
    #[serde(rename = "pb70thPctile")]
    pub pb70th_pctile: FloatOrString,
    #[serde(rename = "pb80thPctile")]
    pub pb80th_pctile: FloatOrString,
    #[serde(rename = "pb90thPctile")]
    pub pb90th_pctile: FloatOrString,
    #[serde(rename = "pe10thPctile")]
    pub pe10th_pctile: FloatOrString,
    #[serde(rename = "pe20thPctile")]
    pub pe20th_pctile: FloatOrString,
    #[serde(rename = "pe30thPctile")]
    pub pe30th_pctile: FloatOrString,
    #[serde(rename = "pe40thPctile")]
    pub pe40th_pctile: FloatOrString,
    #[serde(rename = "pe50thPctile")]
    pub pe50th_pctile: FloatOrString,
    #[serde(rename = "pe60thPctile")]
    pub pe60th_pctile: FloatOrString,
    #[serde(rename = "pe70thPctile")]
    pub pe70th_pctile: FloatOrString,
    #[serde(rename = "pe80thPctile")]
    pub pe80th_pctile: FloatOrString,
    #[serde(rename = "pe90thPctile")]
    pub pe90th_pctile: FloatOrString,
    #[serde(rename = "pretaxMar10thPctile")]
    pub pretax_mar10th_pctile: FloatOrString,
    #[serde(rename = "pretaxMar20thPctile")]
    pub pretax_mar20th_pctile: FloatOrString,
    #[serde(rename = "pretaxMar30thPctile")]
    pub pretax_mar30th_pctile: FloatOrString,
    #[serde(rename = "pretaxMar40thPctile")]
    pub pretax_mar40th_pctile: FloatOrString,
    #[serde(rename = "pretaxMar50thPctile")]
    pub pretax_mar50th_pctile: FloatOrString,
    #[serde(rename = "pretaxMar60thPctile")]
    pub pretax_mar60th_pctile: FloatOrString,
    #[serde(rename = "pretaxMar70thPctile")]
    pub pretax_mar70th_pctile: FloatOrString,
    #[serde(rename = "pretaxMar80thPctile")]
    pub pretax_mar80th_pctile: FloatOrString,
    #[serde(rename = "pretaxMar90thPctile")]
    pub pretax_mar90th_pctile: FloatOrString,
    #[serde(rename = "ps10thPctile")]
    pub ps10th_pctile: FloatOrString,
    #[serde(rename = "ps20thPctile")]
    pub ps20th_pctile: FloatOrString,
    #[serde(rename = "ps30thPctile")]
    pub ps30th_pctile: FloatOrString,
    #[serde(rename = "ps40thPctile")]
    pub ps40th_pctile: FloatOrString,
    #[serde(rename = "ps50thPctile")]
    pub ps50th_pctile: FloatOrString,
    #[serde(rename = "ps60thPctile")]
    pub ps60th_pctile: FloatOrString,
    #[serde(rename = "ps70thPctile")]
    pub ps70th_pctile: FloatOrString,
    #[serde(rename = "ps80thPctile")]
    pub ps80th_pctile: FloatOrString,
    #[serde(rename = "ps90thPctile")]
    pub ps90th_pctile: FloatOrString,
    pub sector: String,
    pub sectorcode: i64,
    pub supersector: String,
    pub supersectorcode: i64,
    pub symbol: String,
}

/// Historic dividend data
#[derive(Deserialize, Debug)]
pub struct Dividend {
    pub ex_date: String,
    pub record_date: String,
    pub amount: FloatOrString,
    pub pay_date: String,
    pub currency: String,
    #[serde(rename = "type")]
    pub div_type: String,
}
