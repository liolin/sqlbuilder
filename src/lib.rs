mod builder;
mod entity;
mod query;

pub use builder::{postgres::PostgresQueryBuilder, QueryBuilder, QueryBuilderError};
pub use entity::{ColumnTrait, EntityName, EntityTrait};
pub use query::{select::select, Query};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_usage() {
        use query::Query;
        let x = query::select::select().build(PostgresQueryBuilder);
    }
}
