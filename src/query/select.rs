use crate::Query;

pub fn select() -> Select {
    Select {}
}

#[derive(Debug)]
pub struct Select {
    // TODO: Implement AST for SELECT
}

impl Query for Select {}
