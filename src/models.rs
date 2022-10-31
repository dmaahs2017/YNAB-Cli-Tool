use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub enum Cleared {
    #[serde(rename = "cleared")]
    Cleared,
    #[serde(rename = "uncleared")]
    Uncleared,
    #[serde(rename = "reconciled")]
    Reconciled,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum FlagColor {
    #[serde(rename = "red")]
    Red,
    #[serde(rename = "orange")]
    Orange,
    #[serde(rename = "yellow")]
    Yellow,
    #[serde(rename = "green")]
    Green,
    #[serde(rename = "blue")]
    Blue,
    #[serde(rename = "purple")]
    Purple,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum TransactionType {
    #[serde(rename = "transaction")]
    Transaction,
    #[serde(rename = "subtransaction")]
    Subtransaction,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HybridTransaction {
    pub id: String,
    pub date: String,
    pub amount: i64,
    pub memo: Option<String>,
    pub cleared: Cleared,
    pub approved: bool,
    pub flag_color: Option<FlagColor>,
    pub account_id: String,
    pub payee_id: Option<String>,
    pub category_id: String,
    pub transfer_account_id: Option<String>,
    pub transfer_transaction_id: Option<String>,
    pub matched_transaction_id: Option<String>,
    pub import_id: Option<String>,
    pub deleted: bool,
    #[serde(rename = "type")]
    pub transaction_type: TransactionType,
    pub parent_transaction_id: Option<String>,
    pub account_name: Option<String>,
    pub payee_name: Option<String>,
    pub category_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HybridTransactionsResponse {
    pub transactions: Vec<HybridTransaction>,
}

/// TB=’Target Category Balance’, TBD=’Target Category Balance by Date’, MF=’Monthly Funding’, NEED=’Plan Your Spending’
#[derive(Debug, Serialize, Deserialize)]
pub enum GoalType {
    TB,
    TBD,
    MF,
    NEED,
    DEBT,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Category {
    pub id: String,
    pub category_group_id: String,
    pub name: String,
    pub hidden: bool,
    pub original_category_group_id: Option<String>,
    pub note: Option<String>,
    pub budgeted: i64,
    pub activity: i64,
    pub balance: i64,
    pub goal_type: Option<GoalType>,
    pub goal_creation_month: Option<String>,
    pub goal_target: Option<i64>,
    pub goal_target_month: Option<String>,
    pub goal_percentage_complete: Option<i32>,
    pub goal_months_to_budget: Option<i32>,
    pub goal_under_funded: Option<i64>,
    pub goal_overall_funded: Option<i64>,
    pub goal_overall_left: Option<i64>,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoryGroupWithCategories {
    pub id: String,
    pub name: String,
    pub hidden: bool,
    pub deleted: bool,
    pub categories: Vec<Category>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CategoriesResponse {
    pub category_groups: Vec<CategoryGroupWithCategories>,
    pub server_knowledge: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DateFormat {
    pub format: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CurrencyFormat {
    pub iso_code: String,
    pub example_format: String,
    pub decimal_digits: i32,
    pub decimal_separator: String,
    pub symbol_first: bool,
    pub group_separator: String,
    pub currency_symbol: String,
    pub display_symbol: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub enum AccountTypeString {
    Checking,
    Savings,
    Cash,
    CreditCard,
    LineOfCredit,
    OtherAsset,
    OtherLiability,
    Mortgage,
    AutoLoan,
    StudentLoan,
    PersonalLoan,
    MedicalDebt,
    OtherDebt,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Account {
    pub id: String,
    pub name: String,
    #[serde(rename = "type")]
    pub account_type: AccountTypeString,
    pub on_budget: bool,
    pub closed: bool,
    pub note: String,
    pub balance: i64,
    pub cleared_balance: i64,
    pub uncleared_balance: i64,
    pub transfer_payee_id: String,
    pub direct_import_linked: bool,
    pub direct_import_in_error: bool,
    pub deleted: bool,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetSummary {
    pub id: String,
    pub name: String,
    pub last_modified_on: String,
    pub first_month: String,
    pub last_month: String,
    pub date_format: DateFormat,
    pub currency_format: CurrencyFormat,
    pub accounts: Option<Vec<Account>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BudgetSummaryResponse {
    pub budgets: Vec<BudgetSummary>,
    pub default_budget: Option<BudgetSummary>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Data<T> {
    pub data: T,
}
