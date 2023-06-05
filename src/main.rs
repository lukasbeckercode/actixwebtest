pub mod part;
pub mod schema;

use actix_web::{App, get, HttpResponse, HttpServer, main, post, Responder, web};
use actix_web::error::ErrorNotFound;
use diesel::prelude::*;
use diesel::pg::PgConnection;
use dotenvy::dotenv;
use std::env;
use diesel::associations::HasTable;

use crate::part::part::{get_example_parts, Part};
use crate::schema::parts::dsl::parts;


#[derive(Debug)]
struct PartNotFoundErr{
    name:&'static str
}

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Hello World!")
}

#[get("/test/{part_id}")]
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

async fn manual_hello() -> impl Responder{
    HttpResponse::Ok().body("Hey there!")
}


fn check_part_exists(part_id:i32) -> Result<Part,PartNotFoundErr> {
    let parts_fetched = get_example_parts();

    for p in parts_fetched{
        if p.id == part_id{
            return Ok(p)
        }
    }

  Err(PartNotFoundErr{name:"Part not found!"})

}

fn establish_db_connection() -> PgConnection{
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set!");
    PgConnection::establish(&db_url).unwrap_or_else(|_| panic!("Connection failed!"))
}

fn get_parts_from_db(connection: &mut PgConnection) -> Vec<Part> {
    let result = parts
        .limit(5)
        .select(Part::as_select())
        .load(connection)
        .expect("Error loading parts");
    return result
}

fn add_part_to_db(connection: &mut PgConnection, new_part:Part){
    diesel::insert_into(parts::table())
        .values(&new_part)
        .returning(Part::as_returning())
        .get_result(connection)
        .expect("Part could not be added");
}

#[main]
async fn main() -> std::io::Result<()>{
    let mut conn = establish_db_connection();
    let examples = get_example_parts();
    for p in examples{
        add_part_to_db(&mut conn, p);
    }
    let res = get_parts_from_db(&mut conn);
    for p in res{
        println!("{}",p.id);
        println!("{}",p.description);
        println!("----");
    }
    HttpServer::new(||{
        App::new()
            .service(hello)
            .service(echo)
            .service(get_part)
            .route("/hey",web::get().to(manual_hello))
    }).bind(("127.0.0.1",7878))?
        .run()
        .await
}