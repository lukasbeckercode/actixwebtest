mod part;

use actix_web::{App, get, HttpResponse, HttpServer, main, post, Responder, web};
use actix_web::error::ErrorNotFound;
use crate::part::part::{get_example_parts, Part};


#[derive(Debug)]
struct PartNotFoundErr{
    name:&'static str
}

#[get("/")]
async fn hello() -> impl Responder{
    HttpResponse::Ok().body("Hello World!")
}

#[get("/test/{part_id}")]
async fn get_part(path:web::Path<u16>) -> actix_web::Result<String>{
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


fn check_part_exists(part_id:u16) -> Result<Part,PartNotFoundErr> {
    let parts = get_example_parts();

    for p in parts{
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
            .route("/hey",web::get().to(manual_hello))
    }).bind(("127.0.0.1",7878))?
        .run()
        .await
}