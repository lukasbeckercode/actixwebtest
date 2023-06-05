pub mod part;
pub mod schema;
pub mod db_utils;

use std::fs;
use actix_web::{App, get, HttpResponse, HttpServer, main, post, Responder, web};
use actix_web::error::ErrorNotFound;
use crate::db_utils::add_part_to_db;
use crate::part::Part;

#[derive(Debug)]
struct PartNotFoundErr{
    name:&'static str
}

#[get("/")]
async fn hello() -> impl Responder{
    let content = fs::read_to_string("html/index.html").expect("File could not be read!");
    HttpResponse::Ok().body(content)
}

#[get("/{part_id}")]
async fn get_part(path:web::Path<i32>) -> actix_web::Result<String>{
    let part_id = path.into_inner();

    let res = check_part_exists(part_id);
    match res {
        Ok(part) => Ok(format!(
            "Part No. {} found! \nDescription: {}\nCurrent Delta: {}",
            part.id,
            part.description,
            part.num_expected - part.num_actual)),
        Err(err) => Err(ErrorNotFound(err.name))
    }


}

#[post("/echo")]
async fn echo(req_body:String) -> impl Responder{
    HttpResponse::Ok().body(req_body)
}

#[post("/add_part")]
async fn add_part(part_data: web::Json<Part>)->actix_web::Result<String>{
    add_part_to_db(&part_data.0);
    Ok(format!("Part with id {} added successfully!",part_data.id))
}



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
            .service(echo)
            .service(get_part)
            .service(add_part)
    }).bind(("127.0.0.1",7878))?
        .run()
        .await
}