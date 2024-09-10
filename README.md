# sqlbuilder

Goal: Implement a type checked sql query builder.
The API should be similar to the following:
```rs
let query = query::select::select()
    .from(person::Entity)
    .columns(vec![person::Column::Id, person::Column::Firstname])
    .build(PostgresQueryBuilder)
    .unwrap();
```

The following should not be compiled as we are querying the person table but asking for the item columns.
```rs
let query = query::select::select()
    .from(person::Entity)
    .columns(vec![item::Column::Id, item::Column::Name])
    .build(PostgresQueryBuilder)
    .unwrap();
```


## Layering
The query building does not perform typechecking, just simple strings are generate.
The higher level API should verify that everything fits
