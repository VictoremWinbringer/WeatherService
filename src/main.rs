extern crate actix_web;
use actix_web::{http, server, App, Path, Responder};

fn weather(info: Path<(String, String)>) -> impl Responder {
    format!("Weather in City: {}  for Period {}", info.0, info.1)
}

fn main() {
    server::new(
        || App::new()
            .route("/api/v1/weather/{city}/{period}", http::Method::GET, weather))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
    ()
}