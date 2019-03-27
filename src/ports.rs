use actix_web::{Responder, HttpResponse, HttpRequest, Path};
use regex;
use crate::entities::{Weather, Exception};
use crate::domain_logic::IWeatherService;
use actix_web::dev::Handler;

pub trait IActixWebPort<T: IWeatherService> {
    fn get_weather(&self, country_code: &str, city: &str, period: &str) -> HttpResponse;
}

pub struct ActixWebPort<T: IWeatherService> {
    pub  service: T
}

impl<T> IActixWebPort<T> for ActixWebPort<T> where T: IWeatherService {
    fn get_weather(&self, country_code: &str, city: &str, period: &str) -> HttpResponse {
        from_result(self.service.get_forecast(city,country_code,period.into()))
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