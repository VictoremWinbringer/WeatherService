extern crate actix_web;
#[macro_use]
extern crate serde_derive;
extern crate serde_json;
extern crate reqwest;
extern crate regex;

use actix_web::{http, server, App};
use crate::ports::{ActixWebPort};
use crate::domain_logic::{WeatherService};
use crate::adapters::{AccumaWeatherAdapter, OpenWeatherMapAdapter, CacheAdapter};
use std::sync::{RwLock, Arc};
use crate::entities::Weather;
use std::time::Duration;

mod entities;
mod adapters;
mod domain_logic;
mod ports;

fn weather_handler(cache: Arc<RwLock<CacheAdapter<Vec<Weather>>>>) -> ActixWebPort<WeatherService> {
    ActixWebPort {
        service: WeatherService::new(
            vec![
                Box::new(AccumaWeatherAdapter),
                Box::new(OpenWeatherMapAdapter),
            ],
            cache,
        )
    }
}

pub fn run(addr: impl std::net::ToSocketAddrs) {
    let duration = Duration::new(60 * 60 * 24, 0);
    let cache: Arc<RwLock<CacheAdapter<Vec<Weather>>>> = Arc::new(RwLock::new(CacheAdapter::new(duration.clone())));

    domain_logic::run_clear_cache_thread(duration.clone(),cache.clone());

    server::new(
        move || {
            let cloned = cache.clone();
            App::new()
                .prefix("/api/v1")
                .resource("/weather/{country_code}/{city}/{period}", move |x| x.method(http::Method::GET).h(weather_handler(cloned)))
        })
        .bind(addr)
        .unwrap()
        .run()
}