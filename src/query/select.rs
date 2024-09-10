use std::fmt::Debug;

use crate::{ColumnTrait, Query};

pub fn select() -> Select {
    Default::default()
}

#[derive(Debug, Default)]
pub struct Select {
    // TODO: Implement AST for SELECT
    pub(crate) expression_list: Vec<Expression>,
    pub(crate) from_list: Vec<FromItem>,
}

impl Select {
    pub fn column<C: ColumnTrait>(mut self, column: C) -> Self {
        self.expression_list
            .push(Expression::Column(ColumnReference::Column(
                column.column_name().into(),
            )));
        self
    }
}

impl Query for Select {}

#[derive(Debug)]
enum Expression {
    Column(ColumnReference),
    Constant(Value),
    // TODO: Function call, ...
}

#[derive(Debug)]
enum ColumnReference {
    Column(Identifier),                  // name
    TableColumn(Identifier, Identifier), // person.name
    Asterisk,                            // *
    TableAsterisk(Identifier),           // person.*
}

type Identifier = String;

#[derive(Debug)]
enum Value {
    Bool(bool),
    String(String),
    TinyInt(i8),
    SmallInt(i16),
    Int(i32),
    BigInt(i64),
    TinyUnsignedInt(i8),
    SmallUnsignedInt(i16),
    UnsignedInt(i32),
    BigUnsignedInt(i64),
    Float(f32),
    Double(f64),
}

#[derive(Debug)]
enum FromItem {}

#[cfg(test)]
mod test {
    use crate::entity::test::person;
    use crate::select;

    #[test]
    fn test() {
        select()
            .column(person::Column::Id)
            .column(person::Column::Firstname);
    }
}

/* SELECT query according to postgres
[ WITH [ RECURSIVE ] with_query [, ...] ]
SELECT [ ALL | DISTINCT [ ON ( expression [, ...] ) ] ]
    [ { * | expression [ [ AS ] output_name ] } [, ...] ]
    [ FROM from_item [, ...] ]
    [ WHERE condition ]
    [ GROUP BY [ ALL | DISTINCT ] grouping_element [, ...] ]
    [ HAVING condition ]
    [ WINDOW window_name AS ( window_definition ) [, ...] ]
    [ { UNION | INTERSECT | EXCEPT } [ ALL | DISTINCT ] select ]
    [ ORDER BY expression [ ASC | DESC | USING operator ] [ NULLS { FIRST | LAST } ] [, ...] ]
    [ LIMIT { count | ALL } ]
    [ OFFSET start [ ROW | ROWS ] ]
    [ FETCH { FIRST | NEXT } [ count ] { ROW | ROWS } { ONLY | WITH TIES } ]
    [ FOR { UPDATE | NO KEY UPDATE | SHARE | KEY SHARE } [ OF table_name [, ...] ] [ NOWAIT | SKIP LOCKED ] [...] ]

where from_item can be one of:

    [ ONLY ] table_name [ * ] [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
                [ TABLESAMPLE sampling_method ( argument [, ...] ) [ REPEATABLE ( seed ) ] ]
    [ LATERAL ] ( select ) [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
    with_query_name [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
    [ LATERAL ] function_name ( [ argument [, ...] ] )
                [ WITH ORDINALITY ] [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
    [ LATERAL ] function_name ( [ argument [, ...] ] ) [ AS ] alias ( column_definition [, ...] )
    [ LATERAL ] function_name ( [ argument [, ...] ] ) AS ( column_definition [, ...] )
    [ LATERAL ] ROWS FROM( function_name ( [ argument [, ...] ] ) [ AS ( column_definition [, ...] ) ] [, ...] )
                [ WITH ORDINALITY ] [ [ AS ] alias [ ( column_alias [, ...] ) ] ]
    from_item join_type from_item { ON join_condition | USING ( join_column [, ...] ) [ AS join_using_alias ] }
    from_item NATURAL join_type from_item
    from_item CROSS JOIN from_item

and grouping_element can be one of:

    ( )
    expression
    ( expression [, ...] )
    ROLLUP ( { expression | ( expression [, ...] ) } [, ...] )
    CUBE ( { expression | ( expression [, ...] ) } [, ...] )
    GROUPING SETS ( grouping_element [, ...] )

and with_query is:

    with_query_name [ ( column_name [, ...] ) ] AS [ [ NOT ] MATERIALIZED ] ( select | values | insert | update | delete )
        [ SEARCH { BREADTH | DEPTH } FIRST BY column_name [, ...] SET search_seq_col_name ]
        [ CYCLE column_name [, ...] SET cycle_mark_col_name [ TO cycle_mark_value DEFAULT cycle_mark_default ] USING cycle_path_col_name ]

TABLE [ ONLY ] table_name [ * ]
*/
