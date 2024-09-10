pub mod postgres;
use crate::query::Query;
use std::{error::Error, fmt::Display};

pub trait QueryBuilder {
    fn build(&self, query: Query) -> Result<String, QueryBuilderError>;
}

#[derive(Debug)]
pub enum QueryBuilderError {
    // TODO: Replace `MissingStuff` with an proper error
    // TODO: Best would be to use the builder pattern
    MissingStuff,
}

impl Display for QueryBuilderError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let message = match self {
            QueryBuilderError::MissingStuff => "some information are missing to build the query",
        };

        write!(f, "{}", message)
    }
}

impl Error for QueryBuilderError {}
