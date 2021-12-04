#[macro_use]
extern crate actix_web;

use actix_web::{App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    println!("GET: /");
    HttpResponse::Ok().body("Hello world!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("Starting actix-web server");

    HttpServer::new(|| App::new().service(index))
        .bind("0.0.0.0:8080")?
        .run()
        .await
}