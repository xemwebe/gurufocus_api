use serde::Deserialize;

/// Structure holding the history of financial data for a single stock.
#[derive(Deserialize, Debug)]
pub struct FinancialData {
    pub financials: DataPeriods,
}

/// Structure holding the history of financial for different periods.
#[derive(Deserialize, Debug)]
pub struct DataPeriods {
    pub annuals: PeriodData,
    pub quarterly: PeriodData,
}

/// Structure holding the history of financial for annual or quarterly period.
#[derive(Deserialize, Debug)]
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
    pub preliminary: Vec<String>,
}

/// Structure holding the balance sheet data.
#[derive(Deserialize, Debug)]
pub struct BalanceSheet {
    #[serde(rename = "Current Deferred Taxes Liabilities")]
    pub current_deferred_taxes_liabilities: Vec<String>,
    #[serde(rename = "Investments And Advances")]
    pub investments_and_advances: Vec<String>,
    #[serde(rename = "Total Stockholders Equity")]
    pub total_stockholders_equity: Vec<String>,
    #[serde(rename = "Long-Term Debt")]
    pub long_term_debt: Vec<String>,
    #[serde(rename = "Preferred Stock")]
    pub preferred_stock: Vec<String>,
    #[serde(rename = "Other Gross PPE")]
    pub other_gross_ppe: Vec<String>,
    #[serde(rename = "Accumulated other comprehensive income (loss)")]
    pub accumulated_other_comprehensive_income: Vec<String>,
    #[serde(rename = "Short-Term Debt & Capital Lease Obligation")]
    pub short_term_debt_and_capital_lease_obligation: Vec<String>,
    #[serde(rename = "Property, Plant and Equipment")]
    pub property_plant_and_equipment: Vec<String>,
    #[serde(rename = "Equity-to-Asset")]
    pub equity_to_asset: Vec<String>,
    #[serde(rename = "Inventories, Inventories Adjustments")]
    pub inventories_adjustments: Vec<String>,
    #[serde(rename = "Inventories, Finished Goods")]
    pub inventories_finished_goods: Vec<String>,
    #[serde(rename = "Goodwill")]
    pub goodwill: Vec<String>,
    #[serde(rename = "Other Current Liabilities")]
    pub other_current_liabilities: Vec<String>,
    #[serde(rename = "Total Equity")]
    pub total_equity: Vec<String>,
    #[serde(rename = "PensionAndRetirementBenefit")]
    pub pension_and_retirement_benefit: Vec<String>,
    #[serde(rename = "Retained Earnings")]
    pub retained_earnings: Vec<String>,
    #[serde(rename = "Land And Improvements")]
    pub land_and_improvements: Vec<String>,
    #[serde(rename = "Additional Paid-In Capital")]
    pub additional_paid_in_capital: Vec<String>,
    #[serde(rename = "Total Current Liabilities")]
    pub total_current_liabilities: Vec<String>,
    #[serde(rename = "Total Inventories")]
    pub total_inventories: Vec<String>,
    #[serde(rename = "Loans Receivable")]
    pub loans_receivable: Vec<String>,
    #[serde(rename = "Total Liabilities")]
    pub total_liabilities: Vec<String>,
    #[serde(rename = "Accumulated Depreciation")]
    pub accumulated_depreciation: Vec<String>,
    #[serde(rename = "Total Current Assets")]
    pub total_current_assets: Vec<String>,
    #[serde(rename = "Long-Term Debt & Capital Lease Obligation")]
    pub long_term_debt_and_capital_lease_obligation: Vec<String>,
    #[serde(rename = "Total Receivables")]
    pub total_receivables: Vec<String>,
    #[serde(rename = "Other Current Assets")]
    pub other_current_assets: Vec<String>,
    #[serde(rename = "Long-Term Capital Lease Obligation")]
    pub long_term_capital_lease_obligation: Vec<String>,
    #[serde(rename = "Short-Term Debt")]
    pub short_term_debt: Vec<String>,
    #[serde(rename = "Intangible Assets")]
    pub intangible_assets: Vec<String>,
    #[serde(rename = "Inventories, Work In Process")]
    pub inventories_work_in_process: Vec<String>,
    #[serde(rename = "Other Long-Term Liabilities")]
    pub other_long_term_liabilities: Vec<String>,
    #[serde(rename = "Machinery, Furniture, Equipment")]
    pub machinery_furniture_equipment: Vec<String>,
    #[serde(rename = "Cash And Cash Equivalents")]
    pub cash_and_cash_equivalents: Vec<String>,
    #[serde(rename = "Treasury Stock")]
    pub treasury_stock: Vec<String>,
    #[serde(rename = "Accounts Payable")]
    pub accounts_payable: Vec<String>,
    #[serde(rename = "Total Assets")]
    pub total_assets: Vec<String>,
    #[serde(rename = "Accounts Payable & Accrued Expense")]
    pub accounts_payable_and_accrued_expense: Vec<String>,
    #[serde(rename = "Marketable Securities")]
    pub marketable_securities: Vec<String>,
    #[serde(rename = "Other Current Receivables")]
    pub other_current_receivables: Vec<String>,
    #[serde(rename = "Other Long Term Assets")]
    pub other_long_term_assets: Vec<String>,
    #[serde(rename = "Inventories, Raw Materials & Components")]
    pub inventories_raw_materials_and_components: Vec<String>,
    #[serde(rename = "Gross Property, Plant and Equipment")]
    pub gross_property_plant_and_equipment: Vec<String>,
    #[serde(rename = "Debt-to-Equity")]
    pub debt_to_equity: Vec<String>,
    #[serde(rename = "Cash, Cash Equivalents, Marketable Securities")]
    pub cash_cash_equivalents_marketable_securities: Vec<String>,
    #[serde(rename = "Other Stockholders Equity")]
    pub other_stockholders_equity: Vec<String>,
    #[serde(rename = "Construction In Progress")]
    pub construction_in_progress: Vec<String>,
    #[serde(rename = "Other Current Payables")]
    pub other_current_payables: Vec<String>,
    #[serde(rename = "Buildings And Improvements")]
    pub buildings_and_improvements: Vec<String>,
    #[serde(rename = "Current Deferred Revenue")]
    pub current_deferred_revenue: Vec<String>,
    #[serde(rename = "Total Tax Payable")]
    pub total_tax_payable: Vec<String>,
    #[serde(rename = "Inventories, Other")]
    pub inventories_other: Vec<String>,
    #[serde(rename = "Short-Term Capital Lease Obligation")]
    pub short_term_capital_lease_obligation: Vec<String>,
    #[serde(rename = "Common Stock")]
    pub common_stock: Vec<String>,
    #[serde(rename = "NonCurrent Deferred Liabilities")]
    pub noncurrent_deferred_liabilities: Vec<String>,
    #[serde(rename = "Current Accrued Expense")]
    pub current_accrued_expense: Vec<String>,
    #[serde(rename = "Accounts Receivable")]
    pub accounts_receivable: Vec<String>,
    #[serde(rename = "DeferredTaxAndRevenue")]
    pub deferred_tax_and_revenue: Vec<String>,
    #[serde(rename = "Minority Interest")]
    pub minority_interest: Vec<String>,
    #[serde(rename = "Notes Receivable")]
    pub notes_receivable: Vec<String>,
}


/// Structure holding the per share data
#[derive(Deserialize, Debug)]
pub struct PerShareData {
    #[serde(rename = "Owner Earnings per Share (TTM)")]
    pub owner_earnings: Vec<String>,
    #[serde(rename = "EBITDA per Share")]
    pub ebitda: Vec<String>,
    #[serde(rename = "Dividends per Share")]
    pub dividends: Vec<String>,
    #[serde(rename = "Book Value per Share")]
    pub book_value: Vec<String>,
    #[serde(rename = "EBIT per Share")]
    pub ebit: Vec<String>,
    #[serde(rename = "Earnings per Share (Diluted)")]
    pub earnings_diluted: Vec<String>,
    #[serde(rename = "Tangible Book per Share")]
    pub tangible_book: Vec<String>,
    #[serde(rename = "Free Cash Flow per Share")]
    pub free_cash_flow: Vec<String>,
    #[serde(rename = "Total Debt per Share")]
    pub total_debt: Vec<String>,
    #[serde(rename = "Month End Stock Price")]
    pub month_end_stock_price: Vec<String>,
    #[serde(rename = "Operating Cash Flow per Share")]
    pub operating_cash_flow: Vec<String>,
    #[serde(rename = "Revenue per Share")]
    pub revenue: Vec<String>,
    #[serde(rename = "EPS without NRI")]
    pub eps_without_nri: Vec<String>,
}

/// Structure holding the cash flow statement data.
#[derive(Deserialize, Debug)]
pub struct CashFlowStatement {
    #[serde(rename = "Purchase Of Investment")]
    pub purchase_of_investment: Vec<String>,
    #[serde(rename = "Change In Receivables")]
    pub change_in_receivables: Vec<String>,
    #[serde(rename = "Change In Working Capital")]
    pub change_in_working_capital: Vec<String>,
    #[serde(rename = "Net Foreign Currency Exchange Gain")]
    pub net_foreign_currency_exchange_gain: Vec<String>,
    #[serde(rename = "Change In DeferredTax")]
    pub change_in_deferrred_tax: Vec<String>,
    #[serde(rename = "Net Intangibles Purchase And Sale")]
    pub net_intangibles_purchase_and_sale: Vec<String>,
    #[serde(rename = "Free Cash Flow")]
    pub free_cash_flow: Vec<String>,
    #[serde(rename = "Purchase Of Business")]
    pub purchase_of_business: Vec<String>,
    #[serde(rename = "Cash Flow from Discontinued Operations")]
    pub cash_flow_from_discontinued_operations: Vec<String>,
    #[serde(rename = "Change In Prepaid Assets")]
    pub change_in_prepaid_assets: Vec<String>,
    #[serde(rename = "Cash From Other Investing Activities")]
    pub cash_from_other_investing_activities: Vec<String>,
    #[serde(rename = "Net Income")]
    pub net_income: Vec<String>,
    #[serde(rename = "Cash From Discontinued Investing Activities")]
    pub cash_from_discontinued_investing_activities: Vec<String>,
    #[serde(rename = "Cash Flow from Others")]
    pub cash_flow_from_others: Vec<String>,
    #[serde(rename = "Cash Flow for Dividends")]
    pub cash_flow_for_dividends: Vec<String>,
    #[serde(rename = "Effect of Exchange Rate Changes")]
    pub effect_of_exchange_rate_changes: Vec<String>,
    #[serde(rename = "Purchase Of Property, Plant, Equipment")]
    pub purchase_of_property_plant_equipment: Vec<String>,
    #[serde(rename = "Sale Of Business")]
    pub sale_of_business: Vec<String>,
    #[serde(rename = "Stock Based Compensation")]
    pub stock_based_compensation: Vec<String>,
    #[serde(rename = "Cash Flow from Financing")]
    pub cash_flow_from_financing: Vec<String>,
    #[serde(rename = "Net Income From Continuing Operations")]
    pub net_income_from_continuing_operations: Vec<String>,
    #[serde(rename = "Net Issuance of Preferred Stock")]
    pub net_issuance_of_preferred_stock: Vec<String>,
    #[serde(rename = "Change In Inventory")]
    pub change_in_inventory: Vec<String>,
    #[serde(rename = "Cash Flow from Operations")]
    pub cash_flow_from_operations: Vec<String>,
    #[serde(rename = "Asset Impairment Charge")]
    pub asset_impairment_charge: Vec<String>,
    #[serde(rename = "Sale Of Property, Plant, Equipment")]
    pub sale_of_property_plant_equipment: Vec<String>,
    #[serde(rename = "Issuance of Stock")]
    pub issuance_of_stock: Vec<String>,
    #[serde(rename = "Depreciation, Depletion and Amortization")]
    pub depreciation_depletion_and_amortization: Vec<String>,
    #[serde(rename = "Change In Payables And Accrued Expense")]
    pub change_in_payables_and_accrued_expense: Vec<String>,
    #[serde(rename = "Net Change in Cash")]
    pub net_change_in_cash: Vec<String>,
    #[serde(rename = "Cash Flow from Investing")]
    pub cash_flow_from_investing: Vec<String>,
    #[serde(rename = "Capital Expenditure")]
    pub capital_expenditure: Vec<String>,
    #[serde(rename = "Net Issuance of Debt")]
    pub net_issuance_of_debt: Vec<String>,
    #[serde(rename = "Other Financing")]
    pub other_financing: Vec<String>,
    #[serde(rename = "Sale Of Investment")]
    pub sale_of_investment: Vec<String>,
    #[serde(rename = "Repurchase of Stock")]
    pub repurchase_of_stock: Vec<String>,
}



/// Structure holding the income statement data.
#[derive(Deserialize, Debug)]
pub struct IncomeStatement {
    #[serde(rename = "Interest Income")]
    pub interest_income: Vec<String>,
    #[serde(rename = "Net Income (Discontinued Operations)")]
    pub net_income_discontinued_operations: Vec<String>,
    #[serde(rename = "Operating Margin %")]
    pub operating_margin: Vec<String>,
    #[serde(rename = "Net Interest Income")]
    pub net_interest_income: Vec<String>,
    #[serde(rename = "Net Income")]
    pub net_income: Vec<String>,
    #[serde(rename = "Operating Income")]
    pub operating_income: Vec<String>,
    #[serde(rename = "Write Off")]
    pub write_off: Vec<String>,
    #[serde(rename = "Gross Margin %")]
    pub gross_margin_pct: Vec<String>,
    #[serde(rename = "Gross Profit")]
    pub gross_profit: Vec<String>,
    #[serde(rename = "Impairment Of Capital Assets")]
    pub impairment_of_capital_assets: Vec<String>,
    #[serde(rename = "Other Income (Expense)")]
    pub other_income: Vec<String>,
    #[serde(rename = "Interest Expense")]
    pub interest_expense: Vec<String>,
    #[serde(rename = "Pre-Tax Income")]
    pub pre_tax_income: Vec<String>,
    #[serde(rename = "Other Operating Expense")]
    pub other_operating_expense: Vec<String>,
    #[serde(rename = "Preferred Dividends")]
    pub prefferred_dividends: Vec<String>,
    #[serde(rename = "Depreciation, Depletion and Amortization")]
    pub depreciation_depletion_and_amortization: Vec<String>,
    #[serde(rename = "Research & Development")]
    pub research_and_development: Vec<String>,
    #[serde(rename = "Tax Rate %")]
    pub tax_rate_pct: Vec<String>,
    #[serde(rename = "Restructuring And Mergern Acquisition")]
    pub restructuring_and_merger_and_acquisition: Vec<String>,
    #[serde(rename = "Net Margin %")]
    pub net_margin: Vec<String>,
    #[serde(rename = "EPS (Diluted)")]
    pub eps_diluted: Vec<String>,
    #[serde(rename = "Revenue")]
    pub revenue: Vec<String>,
    #[serde(rename = "Non Operating Income")]
    pub non_operating_income: Vec<String>,
    #[serde(rename = "EBITDA")]
    pub ebitda: Vec<String>,
    #[serde(rename = "Shares Outstanding (Diluted Average)")]
    pub shares_outstanding_diluted_average: Vec<String>,
    #[serde(rename = "EPS (Basic)")]
    pub eps_basic: Vec<String>,
    #[serde(rename = "Tax Provision")]
    pub tax_provision: Vec<String>,
    #[serde(rename = "Net Income (Continuing Operations)")]
    pub net_income_continuing_operations: Vec<String>,
    #[serde(rename = "Other Income (Minority Interest)")]
    pub other_income_minority_interest: Vec<String>,
    #[serde(rename = "Selling, General, & Admin. Expense")]
    pub selling_general_and_admin_expense: Vec<String>,
    #[serde(rename = "Cost of Goods Sold")]
    pub cost_of_goods_sold: Vec<String>,
    #[serde(rename = "Gain on Sale of Security")]
    pub gain_on_sale_of_security: Vec<String>,
}


/// Container for holding valuation ratios
#[derive(Deserialize, Debug)]
pub struct ValuationRatios {
    #[serde(rename = "Dividend Yield %")]
    pub dividend_yield: Vec<String>,
    #[serde(rename = "Price-to-Free-Cash-Flow")]
    pub price_to_free_cash_flow: Vec<String>,
    #[serde(rename = "Price-to-Operating-Cash-Flow")]
    pub price_to_operation_cash_flow: Vec<String>,
    #[serde(rename = "EV-to-EBIT")]
    pub ev_to_ebit: Vec<String>,
    #[serde(rename = "Forward Rate of Return (Yacktman) %")]
    pub forward_rate_of_return: Vec<String>,
    #[serde(rename = "PE Ratio")]
    pub pe_ratio: Vec<String>,
    #[serde(rename = "PEG Ratio")]
    pub peg_ratio: Vec<String>,
    #[serde(rename = "PB Ratio")]
    pub pb_ratio: Vec<String>,
    #[serde(rename = "EV-to-EBITDA")]
    pub ev_to_ebitda: Vec<String>,
    #[serde(rename = "Price-to-Owner-Earnings")]
    pub price_to_owner_earnings: Vec<String>,
    #[serde(rename = "Price-to-Tangible-Book")]
    pub price_to_tangible_book: Vec<String>,
    #[serde(rename = "Earnings Yield (Joel Greenblatt) %")]
    pub earnings_yield: Vec<String>,
    #[serde(rename = "Shiller PE Ratio")]
    pub shiller_pe_ratio: Vec<String>,
    #[serde(rename = "EV-to-Revenue")]
    pub ev_to_revenue: Vec<String>,
    #[serde(rename = "PS Ratio")]
    pub ps_ratio: Vec<String>,
}

/// Container for holding common size ratios
#[derive(Deserialize, Debug)]
pub struct CommonSizeRatios {
    #[serde(rename = "Net Margin %")]
    pub net_margin_pct: Vec<String>,
    #[serde(rename = "Gross-Profit-to-Asset %")]
    pub gross_profit_to_asset_pct: Vec<String>,
    #[serde(rename = "Return-on-Tangible-Asset")]
    pub return_on_tangible_asset: Vec<String>,
    #[serde(rename = "ROIC %")]
    pub roic_pct: Vec<String>,
    #[serde(rename = "Return-on-Tangible-Equity")]
    pub return_on_tangible_equity: Vec<String>,
    #[serde(rename = "Asset Turnover")]
    pub asset_turnover: Vec<String>,
    #[serde(rename = "COGS-to-Revenue")]
    pub cogs_to_revenue: Vec<String>,
    #[serde(rename = "Dividend Payout Ratio")]
    pub dividend_payout_ratio: Vec<String>,
    #[serde(rename = "Debt-to-Equity")]
    pub debt_to_equity: Vec<String>,
    #[serde(rename = "Operating Margin %")]
    pub operating_margin_pct: Vec<String>,
    #[serde(rename = "Days Inventory")]
    pub days_inventory: Vec<String>,
    #[serde(rename = "FCF Margin %")]
    pub fcf_margin_pct: Vec<String>,
    #[serde(rename = "WACC %")]
    pub wacc_pct: Vec<String>,
    #[serde(rename = "Debt-to-Asset")]
    pub debt_to_asset: Vec<String>,
    #[serde(rename = "Inventory-to-Revenue")]
    pub inventory_to_revenue: Vec<String>,
    #[serde(rename = "Days Sales Outstanding")]
    pub days_sales_outstanding: Vec<String>,
    #[serde(rename = "Cash Conversion Cycle")]
    pub cash_conversion_cycle: Vec<String>,
    #[serde(rename = "Gross Margin %")]
    pub gross_margin_pct: Vec<String>,
    #[serde(rename = "ROA %")]
    pub roa_pct: Vec<String>,
    #[serde(rename = "Inventory Turnover")]
    pub inventory_turnover: Vec<String>,
    #[serde(rename = "ROE % Adjusted to Book Value")]
    pub roe_pct_adjusted_to_book_value: Vec<String>,
    #[serde(rename = "ROC (Joel Greenblatt) %")]
    pub roc_pct: Vec<String>,
    #[serde(rename = "Days Payable")]
    pub days_payable: Vec<String>,
    #[serde(rename = "ROE %")]
    pub roe_pct: Vec<String>,
    #[serde(rename = "Effective Interest Rate on Debt %")]
    pub effective_interest_rate_on_debt_pct: Vec<String>,
    #[serde(rename = "Equity-to-Asset")]
    pub equity_to_asset: Vec<String>,
}

/// Container for holding valuation and quality figures
#[derive(Deserialize, Debug)]
pub struct ValuationAndQuality {
    #[serde(rename = "Scaled Net Operating Assets")]
    pub scaled_net_operating_assets: Vec<String>,
    #[serde(rename = "Beneish M-Score")]
    pub beneish_m_score: Vec<String>,
    #[serde(rename = "Current Ratio")]
    pub current_ratio: Vec<String>,
    #[serde(rename = "Market Cap")]
    pub market_cap: Vec<String>,
    #[serde(rename = "Median PS Value")]
    pub median_ps_value: Vec<String>,
    #[serde(rename = "Number of Shareholders")]
    pub number_of_shareholders: Vec<String>,
    #[serde(rename = "Graham Number")]
    pub graham_number: Vec<String>,
    #[serde(rename = "Restated Filing Date")]
    pub restated_filing_date: Vec<String>,
    #[serde(rename = "Buyback Yield %")]
    pub buyback_yield_pct: Vec<String>,
    #[serde(rename = "Intrinsic Value: Projected FCF")]
    pub intrinsic_value_projected_fcf: Vec<String>,
    #[serde(rename = "Month End Stock Price")]
    pub month_end_stock_price: Vec<String>,
    #[serde(rename = "Net Cash per Share")]
    pub net_cash_per_share: Vec<String>,
    #[serde(rename = "Earnings Power Value (EPV)")]
    pub earnings_power_value: Vec<String>,
    #[serde(rename = "Net Current Asset Value")]
    pub net_current_asset_value: Vec<String>,
    #[serde(rename = "Median PB Value")]
    pub median_pb_value: Vec<String>,
    #[serde(rename = "Peter Lynch Fair Value")]
    pub peter_lynch_fair_value: Vec<String>,
    #[serde(rename = "Shares Outstanding (EOP)")]
    pub shares_outstanding: Vec<String>,
    #[serde(rename = "YoY Rev. per Sh. Growth")]
    pub yoy_rev_per_sh_growth: Vec<String>,
    #[serde(rename = "Interest Coverage")]
    pub interest_coverage: Vec<String>,
    #[serde(rename = "Number of Employees")]
    pub number_of_employees: Vec<String>,
    #[serde(rename = "Lowest Stock Price")]
    pub lowest_stock_price: Vec<String>,
    #[serde(rename = "Shares Buyback Ratio %")]
    pub shares_buyback_ratio_pct: Vec<String>,
    #[serde(rename = "Sloan Ratio %")]
    pub sloan_ratio_pct: Vec<String>,
    #[serde(rename = "Piotroski F-Score")]
    pub pitroski_f_score: Vec<String>,
    #[serde(rename = "Altman Z-Score")]
    pub altman_z_score: Vec<String>,
    #[serde(rename = "Filing Date")]
    pub filing_date: Vec<String>,
    #[serde(rename = "Beta")]
    pub beta: Vec<String>,
    #[serde(rename = "Net-Net Working Capital")]
    pub net_net_working_capital: Vec<String>,
    #[serde(rename = "YoY EPS Growth")]
    pub yoy_eps_growth: Vec<String>,
    #[serde(rename = "5-Year EBITDA Growth Rate")]
    pub five_year_ebitda_growth_rate: Vec<String>,
    #[serde(rename = "Shares Outstanding (Basic Average)")]
    pub shares_outstanding_basic_average: Vec<String>,
    #[serde(rename = "Enterprise Value")]
    pub enterprice_value: Vec<String>,
    #[serde(rename = "Quick Ratio")]
    pub quick_ratio: Vec<String>,
    #[serde(rename = "YoY EBITDA Growth")]
    pub yoy_ebitda_growth: Vec<String>,
    #[serde(rename = "Highest Stock Price")]
    pub highest_stock_price: Vec<String>,
}