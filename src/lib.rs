extern crate actix_web;
use actix_web::{http, server, App, Path, Responder, HttpRequest, HttpResponse};
use actix_web::dev::Handler;

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

pub fn run(addr: impl std::net::ToSocketAddrs){
    server::new(
        || App::new()
            .prefix("/api/v1")
            .route("/weather/{city}/{period}", http::Method::GET, weather))
        .bind(addr)
        .unwrap()
        .run()
}