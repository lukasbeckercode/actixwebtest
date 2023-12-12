 use serde::Deserialize;
 use diesel::prelude::*;

 /// Represents a Part  for which all CRUD Operations can be performed
    #[derive(Queryable,Selectable,Insertable,Deserialize)]
    #[diesel(table_name=crate::schema::parts)]
    #[diesel(check_for_backend(diesel::pg::Pg))]
    pub struct Part{
        pub(crate) id:i32,
        pub(crate) description:String,
        pub(crate) num_actual:i32,
        pub(crate) num_expected:i32
    }

    /// creates example parts to use for offline testing
    pub fn get_example_parts() ->[Part;2]{
        let new_part_1: Part = Part{ id: 1, description: String::from("Test1"), num_actual: 12, num_expected: 12 };
        let new_part_2: Part = Part{ id: 2, description: String::from("Test2"), num_actual: 8, num_expected: 8 };
        return [new_part_1,new_part_2]
    }
