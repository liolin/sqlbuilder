use crate::query::{
    select::{self, Select},
    Query,
};

use super::{QueryBuilder, QueryBuilderError};

#[derive(Debug)]
pub struct PostgresQueryBuilder;

impl QueryBuilder for PostgresQueryBuilder {
    fn build(&self, query: Query) -> Result<String, QueryBuilderError> {
        match query {
            Query::Select(s) => build_select(s),
        }
    }
}

fn build_select(select: Select) -> Result<String, QueryBuilderError> {
    let columns = select
        .expression_list
        .into_iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join(", ");
    let tables = select
        .from_list
        .into_iter()
        .map(|e| e.to_string())
        .collect::<Vec<_>>()
        .join(", ");

    Ok(format!("SELECT {} FROM {}", columns, tables))
}
