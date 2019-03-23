extern crate actix_web;
use actix_web::{http, server, App, Path, Responder, HttpRequest, HttpResponse};

fn weather(info: Path<(String, String)>) -> impl Responder {
    if info.0 == "Moscow" {
        HttpResponse::Ok()
            .content_type("text/html")
            .body(format!("Weather in City: {}  for Period {}", info.0, info.1))

    } else {
     HttpResponse::NotFound()
         .content_type("text/html")
         .body(format!("Weather in City: {} Not found!", info.0))
    }
}

fn main() {
    server::new(
        || App::new()
            .prefix("/api/v1")
            .route("/weather/{city}/{period}", http::Method::GET, weather))
        .bind("127.0.0.1:8080")
        .unwrap()
        .run()
}