use actix_web::{Responder, HttpResponse, HttpRequest, Path, FromRequest};
use regex;
use crate::entities::{Weather, Exception};
use crate::domain_logic::IWeatherService;
use crate::domain_logic;
use actix_web::dev::Handler;
use std::time::Duration;
use std::sync::Arc;

pub trait IActixWebPort<T: IWeatherService> {
    fn get_weather(&self, country_code: &str, city: &str, period: &str) -> HttpResponse;
}

pub struct ActixWebPort<T: IWeatherService> {
    pub  service: T
}

impl<T> IActixWebPort<T> for ActixWebPort<T> where T: IWeatherService {
    fn get_weather(&self, country_code: &str, city: &str, period: &str) -> HttpResponse {
        from_result(self.service.get_forecast(city, country_code, period.into()))
    }
}

fn from_result(result: Result<Vec<Weather>, Exception>) -> HttpResponse {
    match result {
        Ok(weather) => HttpResponse::Ok().json(weather),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("Error: {}", e))
    }
}

impl<S, T: 'static> Handler<S> for ActixWebPort<T> where T: IWeatherService {
    type Result = HttpResponse;

    /// Handle request
    fn handle(&self, req: &HttpRequest<S>) -> Self::Result {
        let info = Path::<(String, String, String)>::extract(req);
        match info {
            Ok(path) => self.get_weather(&path.0, &path.1, &path.2),
            Err(e) => HttpResponse::BadRequest()
                .content_type("text/html")
                .body(format!("{}", e))
        }
    }
}