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
        match period {
            "1day" => from_single_result(self.service.daily_1day(city, country_code)),
            "5day" => from_5day_result(self.service.daily_5day(city, country_code)),
            _ => HttpResponse::NotFound()
                .content_type("text/html")
                .body(format!("Not found period {}!", period))
        }
    }
}

fn from_single_result(result: Result<Weather, Exception>) -> HttpResponse {
    match result {
        Ok(weather) => HttpResponse::Ok().json(weather),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("Error: {}", e))
    }
}

fn from_5day_result(result: Result<[Weather; 5], Exception>) -> HttpResponse {
    match result {
        Ok(weathers) =>
            HttpResponse::Ok().json(weathers),
        Err(e) => HttpResponse::InternalServerError()
            .content_type("text/html")
            .body(format!("Error: {}", e))
    }
}