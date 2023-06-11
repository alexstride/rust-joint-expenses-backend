use super::reports::{CategoryLedger, LedgerEntry};

use serde::Deserialize;

/// A catogory for inclusion in the final list of ledgers which make up the expenses report.
/// 
/// alex_category and connie_category are both optional. Presence of either indicates that user
/// 
/// 
#[derive(Debug)]
pub struct LedgerCategory {
    canonical_name: String,
    alex_category: Option<SoloCategoryConfig>,
    connie_category: Option<SoloCategoryConfig>,
}

 #[derive(Debug, Deserialize)]
 pub struct CategoryConfig {
    pub shared: Vec<SharedCategoryConfig>,
    pub alex: Vec<SoloCategoryConfig>,
    pub connie: Vec<SoloCategoryConfig>,
}

#[derive(Debug, Deserialize)]

pub struct SharedCategoryConfig {
    pub canonical_name: String,
    pub alex: SoloCategoryConfig,
    pub connie: SoloCategoryConfig,
}

#[derive(Debug, Deserialize)]
pub struct SoloCategoryConfig {
    pub id: String,
    pub name: String,
    pub code: String,
}

// TODO - implement this
pub fn create_ledger_categories(category_config: CategoryConfig) -> Vec<LedgerCategory> {
    return Vec::new();
}

