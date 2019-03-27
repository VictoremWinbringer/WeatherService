extern crate actix_web;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate regex;

use actix_web::{http, server, App, Path, Responder, HttpResponse, HttpRequest, FromRequest};
use crate::ports::{IActixWebPort, ActixWebPort};
use std::rc::Rc;
use crate::domain_logic::{WeatherService, IWeatherService};
use crate::adapters::{AccumaWeatherAdapter, OpenWeatherMapAdapter, CacheAdapter};
use std::sync::{RwLock, Arc};
use crate::entities::Weather;
use std::collections::HashMap;
use std::time::Duration;
use actix_web::dev::{Handler, PathConfig};
use std::thread;

mod entities;
mod adapters;
mod domain_logic;
mod ports;

struct WeatherHandler(ActixWebPort<WeatherService>);

impl WeatherHandler {
    fn new(cache: Arc<RwLock<CacheAdapter<Vec<Weather>>>>) -> Self {
        WeatherHandler(ActixWebPort {
            service: WeatherService::new(
                vec![
                    Box::new(AccumaWeatherAdapter),
                    Box::new(OpenWeatherMapAdapter),
                ],
                cache,
            )
        })
    }
}

impl<S> Handler<S> for WeatherHandler {
    type Result = HttpResponse;

    /// Handle request
    fn handle(&self, req: &HttpRequest<S>) -> Self::Result {
        let info = Path::<(String, String, String)>::extract(req);
        match info {
            Ok(path) => self.0.get_weather(&path.0, &path.1, &path.2),
            Err(e) => HttpResponse::BadRequest()
                .content_type("text/html")
                .body(format!("{}", e))
        }
    }
}

pub fn run(addr: impl std::net::ToSocketAddrs) {
    let duration = Duration::new(60 * 60 * 24, 0);
    let cache: Arc<RwLock<CacheAdapter<Vec<Weather>>>> = Arc::new(RwLock::new(CacheAdapter::new(duration.clone())));

   let ch = cache.clone();
   let dr = duration.clone();
    thread::spawn(move ||{
        thread::sleep(dr);
        ch.write().unwrap().clear_expired();
    });


    server::new(
        move || {
            let cloned = cache.clone();
            App::new()
                .prefix("/api/v1")
                .resource("/weather/{country_code}/{city}/{period}", move |x| x.method(http::Method::GET).h(WeatherHandler::new(cloned)))
        })
        .bind(addr)
        .unwrap()
        .run()
}