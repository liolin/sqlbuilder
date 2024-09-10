mod builder;
mod entity;
mod query;

pub use builder::{postgres::PostgresQueryBuilder, QueryBuilder, QueryBuilderError};
pub use entity::{ColumnTrait, EntityName, EntityTrait};
pub use query::{select::select, Query, QueryTrait};

#[cfg(test)]
mod tests {
    use super::*;
    use crate::entity::test::person;

    #[test]
    fn example_usage() {
        use query::QueryTrait;
        let query = query::select::select()
            .column(person::Column::Id)
            .column(person::Column::Firstname)
            .from(person::Entity)
            .build(PostgresQueryBuilder)
            .unwrap();

        assert_eq!("SELECT id, firstname FROM person", query);
    }
}
