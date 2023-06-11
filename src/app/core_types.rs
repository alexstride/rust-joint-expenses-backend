use serde::{Deserialize, Serialize};


// Transaction types

pub struct JointExpenseTransaction {
    payee: String,
    memo: String,
    amount_in_pence: u64,
    iso_date: String,
    category_code: String,
    user: User,
}

pub struct SettlementTransaction {
    from_user: User,
    to_user: User,
    iso_date: String,
    category_code: String,
    amount_in_pence: u64,
    memo: String,
}

pub enum Transaction {
    Expense(JointExpenseTransaction),
    Settlement(SettlementTransaction),
}

// Categories

#[derive(Debug)]
pub struct LedgerCategory {
    canonical_name: String,
    alex_category: Option<SoloCategoryConfig>,
    connie_category: Option<SoloCategoryConfig>,
}

impl LedgerCategory {}

/**
 * Sample Config
 *
 * {
 *      shared:
 *          [{
 *              canonicalName: string
 *              alexId: string
 *              alexCode: string
 *              connieId: string
 *              connieCode: string
 *          }],
 *      alex: [
 *          {
 *              id: string,
 *              name: string,
 *              code: string
 *           }
 *      ],
 *      connie: (same as above)
 *
 * }
 */

 #[derive(Debug, Deserialize, Serialize)]
 pub struct CategoryConfig {
    pub shared: Vec<SharedCategoryConfig>,
    pub alex: Vec<SoloCategoryConfig>,
    pub connie: Vec<SoloCategoryConfig>,
}

#[derive(Debug, Deserialize, Serialize)]

pub struct SharedCategoryConfig {
    pub canonical_name: String,
    pub alex: SoloCategoryConfig,
    pub connie: SoloCategoryConfig,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct SoloCategoryConfig {
    pub id: String,
    pub name: String,
    pub code: String,
}

// Users

#[derive(Debug)]
pub enum User {
    Alex,
    Connie,
}

// Reports & Ledgers

pub type AllCategoriesReport = Vec<CategoryLedger>;

#[derive(Debug)]
pub struct CategoryLedger {
    category_info: LedgerCategory,
    ledger: Vec<LedgerEntry>,
}

#[derive(Debug)]
pub struct LedgerEntry {
    payer: User,
    payee: String,
    amount: u64,
    memo: String,
    owed_to_alex: u64,
}
