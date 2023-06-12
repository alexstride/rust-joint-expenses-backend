use super::categories::{create_ledger_categories, CategoryConfig, LedgerCategory};
use super::transactions::Transaction;
use super::ledgers::LedgerEntry;

#[derive(Debug)]
pub struct CategoryLedger {
    category_info: LedgerCategory,
    ledger: Vec<LedgerEntry>,
}

pub struct ReportGenerator {
    categories: Vec<LedgerCategory>,
}

///
///  A struct to store the categories loaded on startup and fetch data and serve
///  reports on demand
impl ReportGenerator {
    pub fn new(category_config: CategoryConfig) -> Self {
        return ReportGenerator {
            categories: create_ledger_categories(category_config),
        };
    }

    // TODO: implement
    pub fn generate<T>(&self, transactions: T) -> Vec<CategoryLedger>
    where
        T: IntoIterator<Item = Transaction>,
    {
        Vec::new()
    }
}


#[cfg(test)]
#[allow(non_snake_case)]
mod test_ReportGenerator {
    use crate::app::categories::CategoryConfig;

    use super::ReportGenerator;

    fn get_empty_category_config() -> CategoryConfig {
        CategoryConfig {
            shared: Vec::new(),
            alex: Vec::new(),
            connie: Vec::new(),
        }
    }

    #[test]
    fn generate_whenPassedNoCategoriesAndNoTransactions_returnsEmptyVec() {
        let generator = ReportGenerator::new(get_empty_category_config());
        assert_eq!(generator.generate(Vec::new()).len(), 0)
    }
}

