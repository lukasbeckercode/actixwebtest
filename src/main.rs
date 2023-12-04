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

///Checks if a requested part exists
fn check_part_exists(part_id:i32) -> Result<Part,PartNotFoundErr> {
    let parts_fetched = db_utils::get_parts_from_db();

    for p in parts_fetched{
        if p.id == part_id{
            return Ok(p)
        }
    }

  Err(PartNotFoundErr{name:"Part not found!"})

}

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

//UNIT TESTS
//TODO: Define tests
#[cfg(test)]
mod tests{

    #[test]
    fn example_test(){
        let mut test_var = 0;

        assert_eq!(test_var,0);

        test_var = 4+4;
        assert_eq!(test_var,8);
    }

}