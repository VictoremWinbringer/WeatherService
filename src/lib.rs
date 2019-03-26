extern crate actix_web;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate arrayvec;
extern crate regex;

use actix_web::{http, server, App, Path, Responder, HttpResponse};
use crate::ports::{IActixWebPort, ActixWebPort};
use std::rc::Rc;
use crate::domain_logic::{WeatherService, IWeatherService};
use crate::adapters::{AccumaWeatherAdapter, OpenWeatherMapAdapter};

mod entities;
mod adapters;
mod domain_logic;
mod ports;

fn weather(info: Path<(String, String, String)>) -> impl Responder {
    let port = ActixWebPort {
        service: WeatherService::new(
            vec![
                Box::new(AccumaWeatherAdapter),
                Box::new(OpenWeatherMapAdapter),
            ]
        )
    };
    port.get_weather(&info.0, &info.1, &info.2)
}

pub fn run(addr: impl std::net::ToSocketAddrs) {

    server::new(
        || App::new()
            .prefix("/api/v1")
            .route("/weather/{country_code}/{city}/{period}", http::Method::GET, weather))
        .bind(addr)
        .unwrap()
        .run()
}