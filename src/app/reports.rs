use std::thread::current;

use super::categories::{create_ledger_categories, CategoryConfig, LedgerCategory};
use super::transactions::Transaction;
use super::users::User;

#[derive(Debug)]
pub struct CategoryLedger {
    category_info: LedgerCategory,
    ledger: Vec<LedgerEntry>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct LedgerEntry {
    payer: User,
    payee: String,
    amount: u64,
    memo: String,
    owed_to_alex: u64,
    owed_to_connie: u64,
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

const LEDGER_ZERO_ENTRY: LedgerEntry = LedgerEntry {
    payer: User::Alex,
    payee: String::new(),
    amount: 0,
    memo: String::new(),
    owed_to_alex: 0,
    owed_to_connie: 0,
};

/// Creates a ledger from an iterator of transactions, which must be sorted from
/// oldest to newest.

pub fn create_ledger(sorted_transactions: Vec<Transaction>) -> Vec<LedgerEntry> {
    let mut ledger: Vec<LedgerEntry> = Vec::new();

    for transaction in sorted_transactions {
        let current_head = ledger.last();
        let (prev_owed_to_alex, prev_owed_to_connie) = match current_head {
            Some(entry) => (entry.owed_to_alex, entry.owed_to_connie),
            None => (0, 0),
        };

        let new_entry = match transaction {
            Transaction::Expense(expense) => LedgerEntry {
                payee: expense.payee,
                amount: expense.amount_in_pence,
                memo: expense.memo,
                owed_to_alex: prev_owed_to_alex
                    + (if expense.user == User::Alex {
                        expense.amount_in_pence / 2
                    } else {
                        0
                    }),
                owed_to_connie: prev_owed_to_connie
                    + (if expense.user == User::Connie {
                        expense.amount_in_pence / 2
                    } else {
                        0
                    }),
                payer: expense.user,
            },
            Transaction::Settlement(settlement) => LEDGER_ZERO_ENTRY,
        };
        ledger.push(new_entry);
    }
    ledger
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

#[cfg(test)]
#[allow(non_snake_case)]
mod test_create_ledger {
    use crate::app::reports::LedgerEntry;
    use crate::app::transactions::{JointExpenseTransaction, Transaction};
    use crate::app::users::User;

    use super::create_ledger;

    #[test]
    fn create_ledger_withSingleExpenseTransaction_producesCorrectLedger() {
        let transactions = vec![Transaction::Expense(JointExpenseTransaction::new(
            "store1",
            "spent some money",
            1000,
            "2020-01-01",
            "FOO",
            User::Alex,
        ))];

        let ledger = create_ledger(transactions);

        assert_eq!(ledger.len(), 1);

        let ledger_entry = ledger.get(0).expect("Could not get ledger entry!");

        assert_eq!(
            *ledger_entry,
            LedgerEntry {
                payer: User::Alex,
                payee: String::from("store1"),
                amount: 1000,
                memo: String::from("spent some money"),
                owed_to_alex: 500,
                owed_to_connie: 0
            }
        )
    }

    #[test]
    fn create_ledger_withTwoExpenseTransactions_producesCorrectLedger() {
        let transactions = vec![
            Transaction::Expense(JointExpenseTransaction::new(
                "store1",
                "spent some money",
                1000,
                "2020-01-01",
                "FOO",
                User::Alex,
            )),
            Transaction::Expense(JointExpenseTransaction::new(
                "store2",
                "spent more money",
                2000,
                "2020-01-01",
                "FOO",
                User::Alex,
            )),
        ];

        let ledger = create_ledger(transactions);

        assert_eq!(ledger.len(), 2);

        let ledger_entry_1 = ledger.get(0).expect("Could not get ledger entry!");

        assert_eq!(
            *ledger_entry_1,
            LedgerEntry {
                payer: User::Alex,
                payee: String::from("store1"),
                amount: 1000,
                memo: String::from("spent some money"),
                owed_to_alex: 500,
                owed_to_connie: 0
            }
        );

        let ledger_entry_2 = ledger.get(1).expect("Could not get ledger entry!");

        assert_eq!(
            *ledger_entry_2,
            LedgerEntry {
                payer: User::Alex,
                payee: String::from("store2"),
                amount: 2000,
                memo: String::from("spent more money"),
                owed_to_alex: 1500,
                owed_to_connie: 0
            }
        )
    }
}
