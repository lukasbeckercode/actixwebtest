use std::env;
use diesel::{Connection, ExpressionMethods, PgConnection, QueryDsl, RunQueryDsl, SelectableHelper};
use diesel::associations::HasTable;
use dotenvy::dotenv;
use crate::part::Part;
use crate::schema::parts::dsl::parts;
use crate::schema::parts::id;

/// Creates a connection to the database within docker compose 
pub fn create_connection() -> PgConnection{
    dotenv().ok();
    let secret_user = env::var("SECRET_USER").expect("SECRET_USER not found in .env");
    let secret_pass = env::var("SECRET_PASS").expect("SECRET_PASS not found in .env");
    let db_url= format!("postgres://{}:{}@db:5432/inventory", secret_user,secret_pass);

    PgConnection::establish(&db_url).unwrap_or_else(|err| panic!("Database Connection Error: {}",err.to_string()))

}

/// retrieves all parts from the docker compose database
pub fn get_parts_from_db() -> Vec<Part> {
    let connection= &mut create_connection();
    get_parts_from_db_connection(connection)
}

/// retrieves all parts from a database found under the provided connection 
pub fn get_parts_from_db_connection(connection: &mut PgConnection )->Vec<Part>{

    let result = parts
        .limit(5)
        .select(Part::as_select())
        .load(connection)
        .expect("Error loading parts");
    return result
}

/// adds a part to the docker compose database 
pub fn add_part_to_db(new_part:&Part){
    let connection = &mut create_connection();
    add_part_to_db_connection(connection,new_part); 
}

/// adds a part to the database given in the provided connection 
pub fn add_part_to_db_connection(connection: &mut PgConnection, new_part: &Part){
    diesel::insert_into(parts::table())
        .values(new_part)
        .returning(Part::as_returning())
        .get_result(connection)
        .expect("Part could not be added");
}

/// deletes a part from the docker compose database 
pub fn delete_part_from_db(remove_part:&Part){
    let connection = &mut create_connection();
    delete_part_from_db_connection(connection, remove_part); 
}

/// deletes a part from the database given in the provided connection 
pub fn delete_part_from_db_connection(connection: &mut PgConnection, remove_part: &Part){
    diesel::delete(parts::table())
        .filter(id.eq(remove_part.id))
        .execute(connection)
        .expect("Part could not be deleted!");
}