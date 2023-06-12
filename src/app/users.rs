#[derive(Debug, PartialEq, Clone)]
pub enum User {
    Alex,
    Connie,
}

impl std::fmt::Display for User {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            User::Alex => write!(f, "Alex"),
            User::Connie => write!(f, "Connie"),
        }
    }
}