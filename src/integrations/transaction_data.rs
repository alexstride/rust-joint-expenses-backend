pub struct TransactionJsonDataLoader {}

impl TransactionJsonDataLoader {
    pub fn new() -> Self { TransactionJsonDataLoader{} }

    pub fn load(&self) -> String {
        return String::from("{}")
    }
}