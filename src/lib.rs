extern crate actix_web;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;

use actix_web::{http, server, App, Path, Responder, HttpResponse};

mod entities;
mod adapters;

fn weather(info: Path<(String, String, String)>) -> impl Responder {

    if info.1 == "Moscow" {
        HttpResponse::Ok()
            .content_type("text/html")
            .header("Cache-Control","no-cache")
            .body(format!("Weather in Country:{} City: {}  for Period {}", info.0, info.1, info.2))
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
            .route("/weather/{country_code}/{city}/{period}", http::Method::GET, weather))
        .bind(addr)
        .unwrap()
        .run()
}