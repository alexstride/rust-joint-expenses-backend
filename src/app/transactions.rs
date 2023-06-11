use super::users::User;

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
