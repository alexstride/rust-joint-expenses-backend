use super::users::User;

pub struct JointExpenseTransaction {
    pub payee: String,
    pub memo: String,
    pub amount_in_pence: u64,
    pub iso_date: String,
    pub category_code: String,
    pub user: User,
}

impl JointExpenseTransaction {
    pub fn new(
        payee: &str,
        memo: &str,
        amount_in_pence: u64,
        iso_date: &str,
        category_code: &str,
        user: User,
    ) -> Self {
        JointExpenseTransaction {
            payee: String::from(payee),
            memo: String::from(memo),
            amount_in_pence,
            iso_date: String::from(iso_date),
            category_code: String::from(category_code),
            user
        }
    }
}

pub struct SettlementTransaction {
    pub to_user: User,
    pub from_user: User,
    pub iso_date: String,
    pub category_code: String,
    pub amount_in_pence: u64,
    pub memo: String,
}

impl SettlementTransaction {
    pub fn new(
        from_user: User,
        to_user: User,
        iso_date: &str,
        category_code: &str,
        amount_in_pence: u64,
        memo: &str,
    ) -> Self {
        SettlementTransaction {
            from_user,
            to_user,
            iso_date: String::from(iso_date),
            category_code: String::from(category_code),
            amount_in_pence,
            memo: String::from(memo),
        }
    }
}

pub enum Transaction {
    Expense(JointExpenseTransaction),
    Settlement(SettlementTransaction),
}
