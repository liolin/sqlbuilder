pub(crate) mod select;

use crate::{QueryBuilder, QueryBuilderError};

pub trait Query {
    fn build<B: QueryBuilder>(self, builder: B) -> Result<String, QueryBuilderError>
    where
        Self: Sized,
    {
        builder.build(self)
    }
}
