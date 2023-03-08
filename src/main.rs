use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::Serialize;


#[derive(Serialize)]
struct Package {
    name: String,
}

#[derive(Serialize)]
struct SimpleIndex {
    packages: Vec<Package>,
}

#[get("/")]
async fn hello() -> Result<impl Responder> {

    let index = SimpleIndex {};

    Ok(web::Json(index))
}

#[get("/simple")]
async fn echo() -> impl Responder {

    HttpResponse::Ok().body(req_body)
}

async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .service(hello)
            .service(echo)
            .route("/hey", web::get().to(manual_hello))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
