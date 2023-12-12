use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::associations::HasTable;
use dotenvy::dotenv;
use crate::part::Part;
use crate::schema::parts::dsl::parts;
use crate::schema::parts::id;

pub fn create_connection() -> PgConnection{
    dotenv().ok();
    let db_url = "postgres://postgres:1234@db:5432/inventory";
    PgConnection::establish(&db_url).unwrap_or_else(|err| panic!("Database Connection Error: {}",err.to_string()))

}


pub fn get_parts_from_db() -> Vec<Part> {
    let connection= &mut create_connection();
    get_parts_from_db_connection(connection)
}

pub fn get_parts_from_db_connection(connection: &mut PgConnection )->Vec<Part>{

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


pub fn delete_part_from_db(remove_part:&Part){
    let connection = &mut create_connection();
    diesel::delete(parts::table())
        .filter(id.eq(remove_part.id))
        .execute(connection)
        .expect("Part could not be deleted!");
}