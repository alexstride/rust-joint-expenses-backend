use super::transactions::Transaction;
use super::users::User;

#[derive(Debug, PartialEq, Clone)]
pub struct LedgerEntry {
    payer: User,
    payee: String,
    amount: u64,
    memo: String,
    owed_to_alex: u64,
    owed_to_connie: u64,
}

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
            Transaction::Settlement(settlement) => LedgerEntry {
                payee: settlement.to_user.to_string(),
                payer: settlement.from_user,
                amount: settlement.amount_in_pence,
                memo: settlement.memo,
                owed_to_alex: prev_owed_to_alex
                    - (if settlement.to_user == User::Alex {
                        settlement.amount_in_pence
                    } else { 0 }),
                owed_to_connie: prev_owed_to_connie
                    - (if settlement.to_user == User::Connie {
                        settlement.amount_in_pence
                    } else { 0 }),
            },
        };
        ledger.push(new_entry);
    }
    ledger
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test_create_ledger {
    use super::LedgerEntry;
    use crate::app::transactions::{JointExpenseTransaction, SettlementTransaction, Transaction};
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

    #[test]
    fn create_ledger_withOneExpenseTransactionAndASettlementForFullAmount_producesCorrectLedger() {
        let transactions = vec![
            Transaction::Expense(JointExpenseTransaction::new(
                "store1",
                "spent some money",
                1000,
                "2020-01-01",
                "FOO",
                User::Alex,
            )),
            Transaction::Settlement(SettlementTransaction::new(
                User::Connie,
                User::Alex,
                "2020-01-02",
                "FOO",
                500,
                "settled",
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
                payer: User::Connie,
                payee: String::from("Alex"),
                amount: 500,
                memo: String::from("settled"),
                owed_to_alex: 0,
                owed_to_connie: 0
            }
        )
    }

    // TODO: Add tests similar to the above but with the users changed
    // TODO: Add a test for what happened when a settlement is recorded before a transaction
    // TODO: Add a test for what happens if a settlement is made for more money than is required
    // TODO: Add a test for what happens if a transaction is negative
    // TODO: Add a test for what happens if no transactions are passed in
}
