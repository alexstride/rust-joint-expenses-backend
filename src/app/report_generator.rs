use super::core_types::{AllCategoriesReport, LedgerCategory, Transaction};

pub struct ReportGenerator {}


impl ReportGenerator {
    pub fn new(categories: Vec<LedgerCategory>) -> Self {
        return ReportGenerator {};
    }

    pub fn generate<T>(&self, transactions: T) -> AllCategoriesReport
    where
        T: IntoIterator<Item = Transaction>,
    {
        return Vec::new();
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use crate::app::core_types::{LedgerCategory, SoloCategoryConfig};

    use super::ReportGenerator;

    #[test]
    fn generate_whenPassedNoCategoriesAndNoTransactions_returnsEmptyVec() {
        let generator = ReportGenerator::new(Vec::new());
        assert_eq!(generator.generate(Vec::new()).len(), 0)
    }
}
