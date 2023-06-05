use std::env;
use diesel::{Connection, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::associations::HasTable;
use dotenvy::dotenv;
use crate::part::Part;
use crate::schema::parts::dsl::parts;

pub fn create_connection() -> PgConnection{
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Connection failed!"))

}


pub fn get_parts_from_db() -> Vec<Part> {
    let connection= &mut create_connection();

    let result = parts
        .limit(5)
        .select(Part::as_select())
        .load(connection)
        .expect("Error loading parts");
    return result
}


pub fn add_part_to_db(new_part:&Part){
    let connection = &mut create_connection();
    diesel::insert_into(parts::table())
        .values(new_part)
        .returning(Part::as_returning())
        .get_result(connection)
        .expect("Part could not be added");
}


