//! main.rs

use std::net::TcpListener;

use zero2prod::run;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let listen = TcpListener::bind("127.0.0.1:8080").expect("Failed to find binding.");
    run(listen)?.await
}