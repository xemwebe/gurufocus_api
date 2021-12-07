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
}

/// Structure holding the history of financial for annual or quarterly period.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PeriodData {
    #[serde(rename = "Fiscal Year")]
    pub fiscal_year: Vec<String>,
    pub balance_sheet: BalanceSheet,
    pub per_share_data_array: PerShareData,
    pub cashflow_statement: CashFlowStatement,
    pub income_statement: IncomeStatement,
    pub valuation_ratios: ValuationRatios,
    pub common_size_ratios: CommonSizeRatios,
    pub valuation_and_quality: ValuationAndQuality,
    #[serde(rename = "Preliminary")]
    pub preliminary: Vec<FloatOrString>,
}

/// Structure holding the balance sheet data.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct BalanceSheet {
    // for non-financials
    #[serde(rename = "Accounts Payable")]
    pub accounts_payable: Option<Vec<FloatOrString>>,
    #[serde(rename = "Accounts Payable & Accrued Expense")]
    #[serde(alias = "Accounts Payable & Accrued Expense for Financ")]
    pub accounts_payable_and_accrued_expense: Vec<FloatOrString>,
    #[serde(rename = "Accounts Receivable")]
    pub accounts_receivable: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "Accumulated Depreciation")]
    pub accumulated_depreciation: Option<Vec<FloatOrString>>,
    #[serde(rename = "Accumulated other comprehensive income (loss)")]
    pub accumulated_other_comprehensive_income: Vec<FloatOrString>,
    #[serde(rename = "Additional Paid-In Capital")]
    pub additional_paid_in_capital: Vec<FloatOrString>,
    // for banks
    #[serde(rename = "Allowance For Loans And Lease Losses")]
    pub allowance_for_loans_and_lease_losses: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Buildings And Improvements")]
    pub buildings_and_improvements: Option<Vec<FloatOrString>>,
    #[serde(rename = "Cash And Cash Equivalents")]
    #[serde(alias = "Balance Statement Cash and cash equivalents")]
    pub cash_and_cash_equivalents: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Cash, Cash Equivalents, Marketable Securities")]
    pub cash_cash_equivalents_marketable_securities: Option<Vec<FloatOrString>>,
    #[serde(rename = "Common Stock")]
    pub common_stock: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "Construction In Progress")]
    pub construction_in_progress: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Current Accrued Expense")]
    pub current_accrued_expense: Option<Vec<FloatOrString>>,
    // for non-banks
    #[serde(rename = "Current Deferred Revenue")]
    pub current_deferred_revenue: Option<Vec<FloatOrString>>,
    // for non-banks
    #[serde(rename = "Current Deferred Taxes Liabilities")]
    pub current_deferred_taxes_liabilities: Option<Vec<FloatOrString>>,
    #[serde(rename = "Debt-to-Equity")]
    pub debt_to_equity: Vec<FloatOrString>,
    // for insurancies
    #[serde(rename = "Deferred Policy Acquisition Costs")]
    pub deferred_policy_acquisition_costs: Option<Vec<FloatOrString>>,
    // for non-banks
    #[serde(rename = "Deferred Tax And Revenue")]
    pub deferred_tax_and_revenue: Option<Vec<FloatOrString>>,
    // for insurancies
    #[serde(rename = "Equity Investments")]
    pub equity_investments: Option<Vec<FloatOrString>>,
    #[serde(rename = "Equity-to-Asset")]
    pub equity_to_asset: Vec<FloatOrString>,
    // for insurancies
    #[serde(rename = "Fixed Maturity Investment")]
    pub fixed_maturity_investments: Option<Vec<FloatOrString>>,
    // for insurancies
    #[serde(rename = "Future Policy Benefits")]
    pub future_policy_benefits: Option<Vec<FloatOrString>>,
    #[serde(rename = "Goodwill")]
    pub goodwill: Vec<FloatOrString>,
    // for banks
    #[serde(rename = "Gross Loan")]
    pub gross_loan: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Gross Property, Plant and Equipment")]
    pub gross_property_plant_and_equipment: Option<Vec<FloatOrString>>,
    #[serde(rename = "Intangible Assets")]
    pub intangible_assets: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "Inventories, Finished Goods")]
    pub inventories_finished_goods: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Inventories, Inventories Adjustments")]
    pub inventories_adjustments: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Inventories, Other")]
    pub inventories_other: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Inventories, Raw Materials & Components")]
    pub inventories_raw_materials_and_components: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Inventories, Work In Process")]
    pub inventories_work_in_process: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Investments And Advances")]
    pub investments_and_advances: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Land And Improvements")]
    pub land_and_improvements: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Loans Receivable")]
    pub loans_receivable: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Long-Term Capital Lease Obligation")]
    pub long_term_capital_lease_obligation: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Long-Term Debt")]
    pub long_term_debt: Option<Vec<FloatOrString>>,
    #[serde(rename = "Long-Term Debt & Capital Lease Obligation")]
    pub long_term_debt_and_capital_lease_obligation: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "Machinery, Furniture, Equipment")]
    pub machinery_furniture_equipment: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Marketable Securities")]
    pub marketable_securities: Option<Vec<FloatOrString>>,
    #[serde(rename = "Minority Interest")]
    pub minority_interest: Vec<FloatOrString>,
    // for banks
    #[serde(rename = "Money Market Investments")]
    pub money_market_investments: Option<Vec<FloatOrString>>,
    // for financials
    #[serde(rename = "Net Loan")]
    pub net_loan: Option<Vec<FloatOrString>>,
    // for non-banks
    #[serde(rename = "NonCurrent Deferred Liabilities")]
    pub noncurrent_deferred_liabilities: Option<Vec<FloatOrString>>,
    #[serde(rename = "Notes Receivable")]
    pub notes_receivable: Vec<FloatOrString>,
    // for banks
    #[serde(rename = "Other Assets for Banks")]
    pub other_assets_for_banks: Option<Vec<FloatOrString>>,
    // for bank insurance
    #[serde(rename = "Other Assets for Insurance Companies")]
    pub other_assets_for_insurance_companies: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Other Current Assets")]
    pub other_current_assets: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Other Current Liabilities")]
    pub other_current_liabilities: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Other Current Payables")]
    pub other_current_payables: Option<Vec<FloatOrString>>,
    #[serde(rename = "Other Current Receivables")]
    pub other_current_receivables: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "Other Gross PPE")]
    pub other_gross_ppe: Option<Vec<FloatOrString>>,
    // for banks
    #[serde(rename = "Other Liabilities for Banks")]
    pub other_liabilities_for_banks: Option<Vec<FloatOrString>>,
    // for insurancies
    #[serde(rename = "Other Liabilities for Insurance Companies")]
    pub other_liabilities_for_insurance_companies: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Other Long Term Assets")]
    pub other_long_term_assets: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Other Long-Term Liabilities")]
    pub other_long_term_liabilities: Option<Vec<FloatOrString>>,
    #[serde(rename = "Other Stockholders Equity")]
    pub other_stockholders_equity: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "Pension And Retirement Benefit")]
    pub pension_and_retirement_benefit: Option<Vec<FloatOrString>>,
    // for insurancies
    #[serde(rename = "Policyholder Funds")]
    pub policyholder_funds: Option<Vec<FloatOrString>>,
    #[serde(rename = "Preferred Stock")]
    pub preferred_stock: Vec<FloatOrString>,
    #[serde(rename = "Property, Plant and Equipment")]
    pub property_plant_and_equipment: Vec<FloatOrString>,
    #[serde(rename = "Retained Earnings")]
    pub retained_earnings: Vec<FloatOrString>,
    // for banks
    #[serde(rename = "Securities & Investments")]
    pub securities_and_investments: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Short-Term Capital Lease Obligation")]
    pub short_term_capital_lease_obligation: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Short-Term Debt")]
    pub short_term_debt: Option<Vec<FloatOrString>>,
    #[serde(rename = "Short-Term Debt & Capital Lease Obligation")]
    pub short_term_debt_and_capital_lease_obligation: Vec<FloatOrString>,
    // for insurancies
    #[serde(rename = "Short-term investments")]
    pub short_term_investments: Option<Vec<FloatOrString>>,
    #[serde(rename = "Total Assets")]
    pub total_assets: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "Total Current Assets")]
    pub total_current_assets: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Total Current Liabilities")]
    pub total_current_liabilities: Option<Vec<FloatOrString>>,
    // for banks
    #[serde(rename = "Total Deposits")]
    pub total_deposits: Option<Vec<FloatOrString>>,
    #[serde(rename = "Total Equity")]
    pub total_equity: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "Total Inventories")]
    pub total_inventories: Option<Vec<FloatOrString>>,
    #[serde(rename = "Total Liabilities")]
    pub total_liabilities: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "Total Long-Term Assets")]
    pub total_long_term_assets: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Total Long-Term Liabilities")]
    pub total_long_term_liabilities: Option<Vec<FloatOrString>>,
    #[serde(rename = "Total Receivables")]
    pub total_receivables: Vec<FloatOrString>,
    #[serde(rename = "Total Stockholders Equity")]
    pub total_stockholders_equity: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "Total Tax Payable")]
    pub total_tax_payable: Option<Vec<FloatOrString>>,
    #[serde(rename = "Treasury Stock")]
    pub treasury_stock: Vec<FloatOrString>,
    // for banks
    #[serde(rename = "Unearned Income")]
    pub unearned_income: Option<Vec<FloatOrString>>,
    // for insurances
    #[serde(rename = "Unearned Premiums")]
    pub unearned_premiums: Option<Vec<FloatOrString>>,
    // for insurancies
    #[serde(rename = "Unpaid Loss & Loss Reserve")]
    pub unpaid_loss_and_loss_revenue: Option<Vec<FloatOrString>>,
}

/// Structure holding the per share data
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct PerShareData {
    #[serde(rename = "Book Value per Share")]
    pub book_value: Vec<FloatOrString>,
    #[serde(rename = "Dividends per Share")]
    pub dividends: Vec<FloatOrString>,
    #[serde(rename = "Earnings per Share (Diluted)")]
    pub earnings_diluted: Vec<FloatOrString>,
    // for non-banks
    #[serde(rename = "EBIT per Share")]
    pub ebit: Option<Vec<FloatOrString>>,
    // for non-banks
    #[serde(rename = "EBITDA per Share")]
    pub ebitda: Option<Vec<FloatOrString>>,
    #[serde(rename = "EPS without NRI")]
    pub eps_without_nri: Vec<FloatOrString>,
    // for REITs
    #[serde(rename = "FFO per Share")]
    pub ffo: Option<Vec<FloatOrString>>,
    #[serde(rename = "Free Cash Flow per Share")]
    pub free_cash_flow: Vec<FloatOrString>,
    #[serde(rename = "Month End Stock Price")]
    pub month_end_stock_price: Vec<FloatOrString>,
    #[serde(rename = "Operating Cash Flow per Share")]
    pub operating_cash_flow: Vec<FloatOrString>,
    #[serde(rename = "Cash per Share")]
    pub cash: Vec<FloatOrString>,
    #[serde(rename = "Owner Earnings per Share (TTM)")]
    pub owner_earnings: Vec<FloatOrString>,
    #[serde(rename = "Revenue per Share")]
    pub revenue: Vec<FloatOrString>,
    #[serde(rename = "Tangible Book per Share")]
    pub tangible_book: Vec<FloatOrString>,
    #[serde(rename = "Total Debt per Share")]
    pub total_debt: Vec<FloatOrString>,
}

/// Structure holding the cash flow statement data.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CashFlowStatement {
    // financials, direct method
    #[serde(rename = "All Taxes Paid")]
    pub all_taxes_paid: Option<Vec<FloatOrString>>,
    // insurancies, direct method
    #[serde(rename = "Cash Paid for Insurance Activities")]
    pub cash_paid_for_insurance_activities: Option<Vec<FloatOrString>>,
    // direct method
    #[serde(rename = "Cash Payments")]
    pub cash_payments: Option<Vec<FloatOrString>>,
    // banks, direct method
    #[serde(rename = "Cash Payments for Deposits by Banks and Customers")]
    pub cash_payments_for_deposits_by_banks_and_customers: Option<Vec<FloatOrString>>,
    // banks, direct method
    #[serde(rename = "Cash Payments for Loans")]
    pub cash_payments_for_loans: Option<Vec<FloatOrString>>,
    // banks, direct method
    #[serde(rename = "Cash Receipts from Deposits by Banks and Customers")]
    pub cash_receipts_from_deposits_by_banks_and_customers: Option<Vec<FloatOrString>>,
    // direct method
    #[serde(rename = "Cash Receipts from Operating Activities")]
    pub cash_receipts_from_operating_activities: Option<Vec<FloatOrString>>,
    // banks, direct method
    #[serde(rename = "Cash Receipts from Securities Related Activities")]
    pub cash_receipts_from_securities_related_activities: Option<Vec<FloatOrString>>,
    // financials, direct method
    #[serde(rename = "Cash Receipts from Tax Refunds")]
    pub cash_receipts_from_tax_refunds: Option<Vec<FloatOrString>>,
    // insurancies, direct method
    #[serde(rename = "Cash Received from Insurance Activities")]
    pub cash_received_from_insurance_activities: Option<Vec<FloatOrString>>,
    // non-financials, direct method
    #[serde(rename = "Dividends Paid")]
    pub dividends_paid: Option<Vec<FloatOrString>>,
    // non-financials, direct method
    #[serde(rename = "Dividends Received")]
    pub dividends_received: Option<Vec<FloatOrString>>,
    // banks, direct method
    #[serde(rename = "Interest and Commission Paid")]
    pub interest_and_commission_paid: Option<Vec<FloatOrString>>,
    // non-financials, direct method
    #[serde(rename = "Interest Paid")]
    pub interest_paid: Option<Vec<FloatOrString>>,
    // non-financials, direct method
    #[serde(rename = "Interest Received")]
    pub interest_received: Option<Vec<FloatOrString>>,
    // banks, direct method
    #[serde(rename = "Other Cash Payments from Operating Activities")]
    pub other_cash_payments_from_operating_activities: Option<Vec<FloatOrString>>,
    // direct method
    #[serde(rename = "Other Cash Receipts from Operating Activities")]
    pub other_cash_receipts_from_operating_activities: Option<Vec<FloatOrString>>,
    // all
    #[serde(rename = "Issuance of Debt")]
    pub issuance_of_debt: Option<Vec<FloatOrString>>,
    // all
    #[serde(rename = "Payments of Debt")]
    pub payments_of_debt: Option<Vec<FloatOrString>>,
    // direct method
    #[serde(rename = "Payments on Behalf of Employees")]
    pub payments_on_behalf_of_employees: Option<Vec<FloatOrString>>,
    // non-financials, direct method
    #[serde(rename = "Payments to Suppliers for Goods and Services")]
    pub payments_to_suppliers_for_goods_and_services: Option<Vec<FloatOrString>>,
    // non-financials, direct method
    #[serde(rename = "Receipts from Customers")]
    pub receipts_from_customers: Option<Vec<FloatOrString>>,
    // non-financials, direct method
    #[serde(rename = "Receipts from Government Grants")]
    pub receipts_from_government_grants: Option<Vec<FloatOrString>>,
    // non-financials, direct method
    #[serde(rename = "Taxes Refund Paid")]
    pub taxes_refund_paid: Option<Vec<FloatOrString>>,
    // indirect method
    #[serde(rename = "Asset Impairment Charge")]
    pub asset_impairment_charge: Option<Vec<FloatOrString>>,
    // non-insurance, indirect method
    #[serde(rename = "Cash from Discontinued Operating Activities")]
    pub cash_from_discontinued_operating_activities: Option<Vec<FloatOrString>>,
    // indirect method
    #[serde(rename = "Change In Inventory")]
    pub change_in_inventory: Option<Vec<FloatOrString>>,
    // indirect method
    #[serde(rename = "Change In Other Working Capital")]
    pub change_in_other_working_capital: Option<Vec<FloatOrString>>,
    // indirect method
    #[serde(rename = "Change In Payables And Accrued Expense")]
    pub change_in_payables_and_accrued_expense: Option<Vec<FloatOrString>>,
    // indirect method
    #[serde(rename = "Change In Prepaid Assets")]
    pub change_in_prepaid_assets: Option<Vec<FloatOrString>>,
    // indirect method
    #[serde(rename = "Change In Receivables")]
    pub change_in_receivables: Option<Vec<FloatOrString>>,
    // indirect method
    #[serde(rename = "Change In Working Capital")]
    pub change_in_working_capital: Option<Vec<FloatOrString>>,
    // indirect method
    #[serde(rename = "Deferred Tax")]
    pub deferred_tax: Option<Vec<FloatOrString>>,
    // indirect method
    #[serde(rename = "Cash Flow Depreciation, Depletion and Amortization")]
    pub depreciation_depletion_and_amortization: Option<Vec<FloatOrString>>,
    // indirect method
    #[serde(rename = "Net Income From Continuing Operations")]
    pub net_income_from_continuing_operations: Option<Vec<FloatOrString>>,
    // indirect method
    #[serde(rename = "Stock Based Compensation")]
    pub stock_based_compensation: Option<Vec<FloatOrString>>,
    #[serde(rename = "Capital Expenditure")]
    pub capital_expenditure: Vec<FloatOrString>,
    #[serde(rename = "Cash Flow for Dividends")]
    pub cash_flow_for_dividends: Vec<FloatOrString>,
    #[serde(rename = "Cash Flow from Financing")]
    pub cash_flow_from_financing: Vec<FloatOrString>,
    #[serde(rename = "Cash Flow from Investing")]
    pub cash_flow_from_investing: Vec<FloatOrString>,
    #[serde(rename = "Cash Flow from Operations")]
    pub cash_flow_from_operations: Vec<FloatOrString>,
    #[serde(rename = "Cash Flow from Others")]
    pub cash_flow_from_others: Vec<FloatOrString>,
    #[serde(rename = "Cash From Discontinued Investing Activities")]
    pub cash_from_discontinued_investing_activities: Vec<FloatOrString>,
    #[serde(rename = "Cash From Other Investing Activities")]
    pub cash_from_other_investing_activities: Vec<FloatOrString>,
    #[serde(rename = "Effect of Exchange Rate Changes")]
    pub effect_of_exchange_rate_changes: Vec<FloatOrString>,
    // for REITs
    #[serde(rename = "FFO")]
    pub ffo: Option<Vec<FloatOrString>>,
    #[serde(rename = "Free Cash Flow")]
    pub free_cash_flow: Vec<FloatOrString>,
    #[serde(rename = "Issuance of Stock")]
    pub issuance_of_stock: Vec<FloatOrString>,
    #[serde(rename = "Net Change in Cash")]
    pub net_change_in_cash: Vec<FloatOrString>,
    #[serde(rename = "Net Intangibles Purchase And Sale")]
    pub net_intangibles_purchase_and_sale: Vec<FloatOrString>,
    #[serde(rename = "Net Issuance of Debt")]
    pub net_issuance_of_debt: Vec<FloatOrString>,
    #[serde(rename = "Net Issuance of Preferred Stock")]
    pub net_issuance_of_preferred_stock: Vec<FloatOrString>,
    #[serde(rename = "Other Financing")]
    pub other_financing: Vec<FloatOrString>,
    #[serde(rename = "Purchase Of Business")]
    pub purchase_of_business: Vec<FloatOrString>,
    #[serde(rename = "Purchase Of Investment")]
    pub purchase_of_investment: Vec<FloatOrString>,
    #[serde(rename = "Purchase Of Property, Plant, Equipment")]
    pub purchase_of_property_plant_equipment: Vec<FloatOrString>,
    #[serde(rename = "Repurchase of Stock")]
    pub repurchase_of_stock: Vec<FloatOrString>,
    #[serde(rename = "Sale Of Business")]
    pub sale_of_business: Vec<FloatOrString>,
    #[serde(rename = "Sale Of Investment")]
    pub sale_of_investment: Vec<FloatOrString>,
    #[serde(rename = "Sale Of Property, Plant, Equipment")]
    pub sale_of_property_plant_equipment: Vec<FloatOrString>,
}

/// Structure holding the income statement data.
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct IncomeStatement {
    // for non-financials
    #[serde(rename = "Cost of Goods Sold")]
    pub cost_of_goods_sold: Option<Vec<FloatOrString>>,
    // for banks
    #[serde(rename = "Credit Losses Provision")]
    pub credit_losses_provision: Option<Vec<FloatOrString>>,
    #[serde(rename = "Depreciation, Depletion and Amortization")]
    pub depreciation_depletion_and_amortization: Vec<FloatOrString>,
    // for non-banks
    #[serde(rename = "EBIT")]
    pub ebit: Option<Vec<FloatOrString>>,
    // for non-banks
    #[serde(rename = "EBITDA")]
    pub ebitda: Option<Vec<FloatOrString>>,
    #[serde(rename = "EPS (Basic)")]
    pub eps_basic: Vec<FloatOrString>,
    #[serde(rename = "EPS (Diluted)")]
    pub eps_diluted: Vec<FloatOrString>,
    // for insurancies
    #[serde(rename = "Fees and Other Income")]
    pub fees_and_other_income: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Gross Margin %")]
    pub gross_margin_pct: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Gross Profit")]
    pub gross_profit: Option<Vec<FloatOrString>>,
    #[serde(rename = "Interest Expense")]
    #[serde(alias = "Interest Expense (Positive)")]
    pub interest_expense: Vec<FloatOrString>,
    #[serde(rename = "Interest Income")]
    pub interest_income: Vec<FloatOrString>,
    #[serde(rename = "Net Income")]
    pub net_income: Vec<FloatOrString>,
    #[serde(rename = "Net Income (Continuing Operations)")]
    pub net_income_continuing_operations: Vec<FloatOrString>,
    #[serde(rename = "Net Income (Discontinued Operations)")]
    pub net_income_discontinued_operations: Vec<FloatOrString>,
    // for non-insurancies
    #[serde(rename = "Net Interest Income")]
    #[serde(alias = "Net Interest Income (for Banks)")]
    pub net_interest_income: Option<Vec<FloatOrString>>,
    // for insurancies
    #[serde(rename = "Net Investment Income")]
    pub net_investment_income: Option<Vec<FloatOrString>>,
    #[serde(rename = "Net Margin %")]
    pub net_margin: Vec<FloatOrString>,
    // for banks
    #[serde(rename = "Non Interest Income")]
    pub non_interest_income: Option<Vec<FloatOrString>>,
    // for insurancies
    #[serde(rename = "Net Policyholder Benefits/Claims")]
    pub net_policyholder_benefits_claims: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Operating Income")]
    pub operating_income: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Operating Margin %")]
    pub operating_margin: Option<Vec<FloatOrString>>,
    #[serde(rename = "Other Income (Expense)")]
    #[serde(alias = "Other Expense")]
    pub other_income: Vec<FloatOrString>,
    #[serde(rename = "Other Income (Minority Interest)")]
    pub other_income_minority_interest: Vec<FloatOrString>,
    // for banks
    #[serde(rename = "Other Noninterest Expense")]
    pub other_noninterest_expense: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Other Operating Expense")]
    pub other_operating_expense: Option<Vec<FloatOrString>>,
    // for insurancies
    #[serde(rename = "Policy Acquisition Expense")]
    pub policy_acquisition_expense: Option<Vec<FloatOrString>>,
    #[serde(rename = "Pretax Income")]
    pub pre_tax_income: Vec<FloatOrString>,
    #[serde(rename = "Preferred Dividends")]
    pub prefferred_dividends: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "Research & Development")]
    pub research_and_development: Option<Vec<FloatOrString>>,
    #[serde(rename = "Revenue")]
    pub revenue: Vec<FloatOrString>,
    #[serde(rename = "Selling, General, & Admin. Expense")]
    pub selling_general_and_admin_expense: Vec<FloatOrString>,
    #[serde(rename = "Shares Outstanding (Diluted Average)")]
    pub shares_outstanding_diluted_average: Vec<FloatOrString>,
    // for banks
    #[serde(rename = "Special Charges")]
    pub special_charges: Option<Vec<FloatOrString>>,
    #[serde(rename = "Tax Provision")]
    pub tax_provision: Vec<FloatOrString>,
    #[serde(rename = "Tax Rate %")]
    pub tax_rate_pct: Vec<FloatOrString>,
    // for insurancies
    #[serde(rename = "Total Expenses")]
    pub total_expenses: Option<Vec<FloatOrString>>,
    // for banks
    #[serde(rename = "Total Noninterest Expense")]
    pub total_noninterest_expense: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Total Operating Expense")]
    pub total_operating_expense: Option<Vec<FloatOrString>>,
    // for insurancies
    #[serde(rename = "Total Premiums Earned")]
    pub total_premiums_earned: Option<Vec<FloatOrString>>,
}

/// Container for holding valuation ratios
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ValuationRatios {
    #[serde(rename = "Dividend Yield %")]
    pub dividend_yield: Vec<FloatOrString>,
    #[serde(rename = "Earnings Yield (Joel Greenblatt) %")]
    pub earnings_yield: Vec<FloatOrString>,
    // for non-banks
    #[serde(rename = "EV-to-EBIT")]
    pub ev_to_ebit: Option<Vec<FloatOrString>>,
    // for non-banks
    #[serde(rename = "EV-to-EBITDA")]
    pub ev_to_ebitda: Option<Vec<FloatOrString>>,
    #[serde(rename = "EV-to-Revenue")]
    pub ev_to_revenue: Vec<FloatOrString>,
    #[serde(rename = "Forward Rate of Return (Yacktman) %")]
    pub forward_rate_of_return: Vec<FloatOrString>,
    #[serde(rename = "PB Ratio")]
    pub pb_ratio: Vec<FloatOrString>,
    #[serde(rename = "PE Ratio")]
    pub pe_ratio: Vec<FloatOrString>,
    #[serde(rename = "PE Ratio without NRI")]
    pub pe_ratio_without_nri: Vec<FloatOrString>,
    #[serde(rename = "PEG Ratio")]
    pub peg_ratio: Vec<FloatOrString>,
    // for REITs
    #[serde(rename = "Price-to-FFO")]
    pub price_to_ffo: Option<Vec<FloatOrString>>,
    #[serde(rename = "Price-to-Free-Cash-Flow")]
    pub price_to_free_cash_flow: Vec<FloatOrString>,
    #[serde(rename = "Price-to-Operating-Cash-Flow")]
    pub price_to_operation_cash_flow: Vec<FloatOrString>,
    #[serde(rename = "Price-to-Owner-Earnings")]
    pub price_to_owner_earnings: Vec<FloatOrString>,
    #[serde(rename = "Price-to-Tangible-Book")]
    pub price_to_tangible_book: Vec<FloatOrString>,
    #[serde(rename = "PS Ratio")]
    pub ps_ratio: Vec<FloatOrString>,
    #[serde(rename = "Shiller PE Ratio")]
    pub shiller_pe_ratio: Vec<FloatOrString>,
}

/// Container for holding common size ratios
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct CommonSizeRatios {
    #[serde(rename = "Asset Turnover")]
    pub asset_turnover: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "Cash Conversion Cycle")]
    pub cash_conversion_cycle: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "COGS-to-Revenue")]
    pub cogs_to_revenue: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Days Inventory")]
    pub days_inventory: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Days Payable")]
    pub days_payable: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Days Sales Outstanding")]
    pub days_sales_outstanding: Option<Vec<FloatOrString>>,
    #[serde(rename = "Debt-to-Asset")]
    pub debt_to_asset: Vec<FloatOrString>,
    #[serde(rename = "Liabilities-to-Assets")]
    pub liabilities_to_assets: Vec<FloatOrString>,
    #[serde(rename = "Debt-to-Equity")]
    pub debt_to_equity: Vec<FloatOrString>,
    #[serde(rename = "Dividend Payout Ratio")]
    pub dividend_payout_ratio: Vec<FloatOrString>,
    // for REITs
    #[serde(rename = "Dividend-Payout-to-FFO")]
    pub dividend_payout_to_ffo: Option<Vec<FloatOrString>>,
    #[serde(rename = "Effective Interest Rate on Debt %")]
    pub effective_interest_rate_on_debt_pct: Vec<FloatOrString>,
    #[serde(rename = "Equity-to-Asset")]
    pub equity_to_asset: Vec<FloatOrString>,
    #[serde(rename = "FCF Margin %")]
    pub fcf_margin_pct: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "Gross Margin %")]
    pub gross_margin_pct: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Gross-Profit-to-Asset %")]
    pub gross_profit_to_asset_pct: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Inventory Turnover")]
    pub inventory_turnover: Option<Vec<FloatOrString>>,
    // for non-financials
    #[serde(rename = "Inventory-to-Revenue")]
    pub inventory_to_revenue: Option<Vec<FloatOrString>>,
    // for banks
    #[serde(rename = "Net Interest Margin (Bank Only) %")]
    pub net_interest_margin_bank_pct: Option<Vec<FloatOrString>>,
    #[serde(rename = "Net Margin %")]
    pub net_margin_pct: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "Operating Margin %")]
    pub operating_margin_pct: Option<Vec<FloatOrString>>,
    #[serde(rename = "Return-on-Tangible-Asset")]
    pub return_on_tangible_asset: Vec<FloatOrString>,
    #[serde(rename = "Return-on-Tangible-Equity")]
    pub return_on_tangible_equity: Vec<FloatOrString>,
    #[serde(rename = "ROA %")]
    pub roa_pct: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "ROC (Joel Greenblatt) %")]
    pub roc_pct: Option<Vec<FloatOrString>>,
    #[serde(rename = "ROE %")]
    pub roe_pct: Vec<FloatOrString>,
    // for non-financials
    #[serde(rename = "ROCE %")]
    pub roce_pct: Option<Vec<FloatOrString>>,
    #[serde(rename = "ROIC %")]
    pub roic_pct: Vec<FloatOrString>,
    #[serde(rename = "ROE % Adjusted to Book Value")]
    pub roe_pct_adjusted_to_book_value: Vec<FloatOrString>,
    #[serde(rename = "WACC %")]
    pub wacc_pct: Vec<FloatOrString>,
    #[serde(rename = "Capex-to-Revenue")]
    pub capex_to_revenue: Vec<FloatOrString>,
}

/// Container for holding valuation and quality figures
#[derive(Deserialize, Debug)]
#[serde(deny_unknown_fields)]
pub struct ValuationAndQuality {
    // non-banks
    #[serde(rename = "5-Year EBITDA Growth Rate (Per Share)")]
    pub five_year_ebitda_growth_rate: Option<Vec<FloatOrString>>,
    // non-financials
    #[serde(rename = "Altman Z-Score")]
    pub altman_z_score: Option<Vec<FloatOrString>>,
    #[serde(rename = "Beneish M-Score")]
    pub beneish_m_score: Vec<FloatOrString>,
    #[serde(rename = "Beta")]
    pub beta: Vec<FloatOrString>,
    #[serde(rename = "Buyback Yield %")]
    pub buyback_yield: Vec<FloatOrString>,
    // non-financials
    #[serde(rename = "Current Ratio")]
    pub current_ratio: Option<Vec<FloatOrString>>,
    #[serde(rename = "Earnings Power Value (EPV)")]
    pub earnings_power_value: Vec<FloatOrString>,
    #[serde(rename = "Enterprise Value ($M)")]
    pub enterprice_value: Vec<FloatOrString>,
    #[serde(rename = "Filing Date")]
    pub filing_date: Vec<FloatOrString>,
    #[serde(rename = "Forex Rate")]
    pub forex_rate: Vec<FloatOrString>,
    #[serde(rename = "Graham Number")]
    pub graham_number: Vec<FloatOrString>,
    #[serde(rename = "Highest Stock Price")]
    pub highest_stock_price: Vec<FloatOrString>,
    // non-banks
    #[serde(rename = "Interest Coverage")]
    pub interest_coverage: Option<Vec<FloatOrString>>,
    #[serde(rename = "Intrinsic Value: Projected FCF")]
    pub intrinsic_value_projected_fcf: Vec<FloatOrString>,
    #[serde(rename = "Lowest Stock Price")]
    pub lowest_stock_price: Vec<FloatOrString>,
    #[serde(rename = "Market Cap")]
    pub market_cap: Vec<FloatOrString>,
    #[serde(rename = "Median PB Value")]
    pub median_pb_value: Option<Vec<FloatOrString>>,
    #[serde(rename = "Median PS Value")]
    pub median_ps_value: Vec<FloatOrString>,
    #[serde(rename = "Month End Stock Price")]
    pub month_end_stock_price: Vec<FloatOrString>,
    #[serde(rename = "Net Cash per Share")]
    pub net_cash_per_share: Vec<FloatOrString>,
    #[serde(rename = "Net Current Asset Value")]
    pub net_current_asset_value: Vec<FloatOrString>,
    #[serde(rename = "Net-Net Working Capital")]
    pub net_net_working_capital: Vec<FloatOrString>,
    #[serde(rename = "Number of Employees")]
    pub number_of_employees: Vec<FloatOrString>,
    #[serde(rename = "Number of Shareholders")]
    pub number_of_shareholders: Vec<FloatOrString>,
    #[serde(rename = "Peter Lynch Fair Value")]
    pub peter_lynch_fair_value: Vec<FloatOrString>,
    // non-financials
    #[serde(rename = "Piotroski F-Score")]
    pub pitroski_f_score: Option<Vec<FloatOrString>>,
    // non-financials
    #[serde(rename = "Quick Ratio")]
    pub quick_ratio: Option<Vec<FloatOrString>>,
    #[serde(rename = "Cash Ratio")]
    pub cash_ratio: Option<Vec<FloatOrString>>,
    #[serde(rename = "Restated Filing Date")]
    pub restated_filing_date: Vec<String>,
    #[serde(rename = "Earnings Release Date")]
    pub earnings_release_date: Vec<String>,
    #[serde(rename = "Scaled Net Operating Assets")]
    pub scaled_net_operating_assets: Vec<FloatOrString>,
    #[serde(rename = "Shares Buyback Ratio %")]
    pub shares_buyback_ratio_pct: Vec<FloatOrString>,
    #[serde(rename = "Shares Outstanding (Basic Average)")]
    pub shares_outstanding_basic_average: Vec<FloatOrString>,
    #[serde(rename = "Shares Outstanding (EOP)")]
    pub shares_outstanding: Vec<FloatOrString>,
    #[serde(rename = "Sloan Ratio %")]
    pub sloan_ratio_pct: Vec<FloatOrString>,
    // for non-banks
    #[serde(rename = "YoY EBITDA Growth (%)")]
    pub yoy_ebitda_growth: Option<Vec<FloatOrString>>,
    #[serde(rename = "YoY EPS Growth")]
    pub yoy_eps_growth: Vec<FloatOrString>,
    #[serde(rename = "YoY Rev. per Sh. Growth")]
    pub yoy_rev_per_sh_growth: Vec<FloatOrString>,
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
