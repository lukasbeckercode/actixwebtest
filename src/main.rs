pub mod part;
pub mod schema;
pub mod db_utils;

use std::fs;
use actix_web::{App, delete, get, HttpResponse, HttpServer, main, post, Responder, web};
use actix_web::error::ErrorNotFound;
use crate::db_utils::{add_part_to_db, delete_part_from_db};
use crate::part::Part;

///Error if requested Part was not found
#[derive(Debug)]
struct PartNotFoundErr{
    name:&'static str
}

///Shows the Info Page
#[get("/")]
async fn hello() -> impl Responder{
    let content = fs::read_to_string("html/index.html").expect("File could not be read!");
    HttpResponse::Ok().body(content)
}

///returns the part with the ID given in the url
#[get("/{part_id}")]
async fn get_part(path:web::Path<i32>) -> actix_web::Result<String>{
    let part_id = path.into_inner();

    let res = check_part_exists(part_id);
    match res {
        //TODO: change this to return JSON data!
        Ok(part) => Ok(format!(
            "Part No. {} found! \nDescription: {}\nCurrent Delta: {}",
            part.id,
            part.description,
            part.num_actual - part.num_expected)),
        Err(err) => Err(ErrorNotFound(err.name))
    }
}

///deletes a part from the database
#[delete("/{part_id}")]
async fn delete_part(path:web::Path<i32>) -> actix_web::Result<String>{
    let part_id = path.into_inner();
    let res = check_part_exists(part_id);

    match res {
        Ok(part) => {
            delete_part_from_db(&part);
            Ok(format!("Part with id {} deleted successfully",part.id))
        },
        Err(err) => Err(ErrorNotFound(err.name))
    }
}

///Adds a Part sent in JSON Format to the Database
#[post("/add_part")]
async fn add_part(part_data: web::Json<Part>)->actix_web::Result<String>{
    add_part_to_db(&part_data.0);
    Ok(format!("Part with id {} added successfully!",part_data.id))
}

/// fetches all parts and checks if a requested part exists
fn check_part_exists(part_id:i32) -> Result<Part,PartNotFoundErr> {
    let parts_fetched = db_utils::get_parts_from_db();
    check_part_exists_filter(part_id,parts_fetched)
}

/// trys to find the requested part in the list of existing parts 
fn check_part_exists_filter(part_id:i32,parts_fetched:Vec<Part>) -> Result<Part,PartNotFoundErr>{

    for p in parts_fetched{
        if p.id == part_id{
            return Ok(p)
        }
    }

    Err(PartNotFoundErr{name:"Part not found!"})
}

///Main function: handles http binding 
#[main]
async fn main() -> std::io::Result<()>{
    HttpServer::new(||{
        App::new()
            .service(hello)
            .service(get_part)
            .service(add_part)
            .service(delete_part)
    }).bind(("0.0.0.0",7878))?
        .run()
        .await
}

///UNIT TESTS
/// Make sure the postgres-container in running 
/// For local testing, use run_db.sh to setup the test environment and stop_db.sh for teardown 
#[cfg(test)]
mod tests{
    use diesel::{Connection, PgConnection};
    use dotenvy::dotenv;
    use crate::check_part_exists_filter;
    use crate::db_utils::{add_part_to_db_connection, delete_part_from_db_connection, get_parts_from_db_connection};
    use crate::part::Part;

    fn setup() -> PgConnection {
        dotenv().ok();
        let db_url = "postgres://postgres:1234@127.0.0.1:5432/inventory";
        PgConnection::establish(&db_url).unwrap_or_else(|err| panic!("Database Connection Error: {}",err.to_string()))
    }
    #[test]
    fn example_test(){
        let mut test_var = 0;

        assert_eq!(test_var,0);

        test_var = 4+4;
        assert_eq!(test_var,8);
    }

    #[test]
    fn test_get_part_by_id_gc(){
        let  connection = &mut setup();
        let parts_fetched = get_parts_from_db_connection(connection);
        let result = check_part_exists_filter(0,parts_fetched);

        assert_eq!(result.is_err(),false);

        let result_unwraped = result.unwrap();
        let description = result_unwraped.description;
        let num_exp = result_unwraped.num_expected;
        let num_actual = result_unwraped.num_actual;

        assert_eq!(description,"Test 1");
        assert_eq!(num_exp,12);
        assert_eq!(num_actual,12);
    }

    #[test]
    fn test_get_part_by_id_bc(){
        let connection = &mut setup();
        let parts_fetched = get_parts_from_db_connection(connection);
        let result = check_part_exists_filter(99,parts_fetched);
        assert_eq!(result.is_err(),true);
    }

    #[test]
    fn test_add_new_part_gc(){
        let connection = &mut setup();
        let new_part = Part { id: 5, description: "Part added by TC".parse().unwrap(), num_actual: 9, num_expected: 9 };
        add_part_to_db_connection(connection,&new_part);
    }

    #[test]
    fn test_delete_part_from_db_gc(){
        let connection = &mut setup();
        let parts_fetched = get_parts_from_db_connection(connection);
        let result = check_part_exists_filter(1,parts_fetched);

        assert_eq!(result.is_err(),false);

        let result_unwraped = result.unwrap();

        delete_part_from_db_connection(connection,&result_unwraped)
    }


}