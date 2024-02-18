//! lib.rs

use actix_web::{dev::Server, get, post, web, App, HttpResponse, HttpServer, Responder};
use std::net::TcpListener;

#[get("/health-check")]
async fn health_check() -> impl Responder {
    HttpResponse::Ok().finish()
}

#[derive(serde::Deserialize)]
pub struct FormData {
    email: String,
    name: String
}

#[post("/subscription")]
async fn subscription(_form: web::Form<FormData>) -> HttpResponse {
    HttpResponse::Ok().finish()
}


pub fn run(listener: TcpListener) -> Result<Server, std::io::Error> {
    let server = HttpServer::new(
        || App::new()
    .service(health_check)
    .service(subscription)
).listen(listener)?
        .run();
    Ok(server)
}