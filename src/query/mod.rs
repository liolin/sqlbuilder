pub(crate) mod select;

use crate::{QueryBuilder, QueryBuilderError};

pub enum Query {
    Select(select::Select),
}

pub trait QueryTrait {
    fn build<B: QueryBuilder>(self, builder: B) -> Result<String, QueryBuilderError>;
}
