pub trait EntityName {
    fn table_name(&self) -> &str;
}

pub trait ColumnTrait {
    fn column_name(&self) -> &str;
}

pub trait EntityTrait: EntityName {
    type Model;
    type Column;
}

#[cfg(test)]
mod test {
    use super::*;

    mod person {
        pub struct Model {
            pub id: i32,
            pub firstname: String,
            pub lastname: String,
        }

        // Code below should be generated
        use super::*;
        pub struct Entity;
        impl EntityName for Entity {
            fn table_name(&self) -> &str {
                "person"
            }
        }

        impl EntityTrait for Entity {
            type Model = Model;
            type Column = Column;
        }

        pub enum Column {
            Id,
            Firstname,
            Lastname,
        }
    }

    mod item {
        pub struct Model {
            pub id: i32,
            pub name: String,
        }

        // Code below should be generated
        use super::*;
        pub struct Entity;
        impl EntityName for Entity {
            fn table_name(&self) -> &str {
                "item"
            }
        }

        impl EntityTrait for Entity {
            type Model = Model;
            type Column = Column;
        }

        pub enum Column {
            Id,
            Name,
        }
    }
}
