use crate::query::Query;

use super::{QueryBuilder, QueryBuilderError};

#[derive(Debug)]
pub struct PostgresQueryBuilder;

impl QueryBuilder for PostgresQueryBuilder {
    fn build<Q: Query>(&self, _query: Q) -> Result<String, QueryBuilderError> {
        todo!()
    }
}
