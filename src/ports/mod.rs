use actix_web::{Responder, HttpResponse};
use regex;
use crate::entities::{Weather, Exception};
use crate::domain_logic::IWeatherService;

pub trait IActixWebPort<T: IWeatherService> {
    fn get_weather(&self, country_code: &str, city: &str, period: &str) -> HttpResponse;
}

pub struct ActixWebPort<T: IWeatherService> {
    service: T
}

impl<T> IActixWebPort<T> for ActixWebPort<T> where T: IWeatherService {
    fn get_weather(&self, country_code: &str, city: &str, period: &str) -> HttpResponse {
        let rgx = regex::Regex::new("[a-zA-Z]{2}");
        if let Err(e) = rgx {
            return HttpResponse::InternalServerError()
                .content_type("text/html")
                .body(format!("Regex error {}", e));
        }

        if rgx.unwrap().is_match(country_code) {
            return HttpResponse::BadRequest()
                .content_type("text/html")
                .body(format!("Country code must by ISO 2 alphabet code like us or ru. Current code is {}!", country_code));
        }

        if city.trim().is_empty() {
            return HttpResponse::BadRequest()
                .content_type("text/html")
                .body(format!("City name is empty {}!", city));
        }

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
    unimplemented!()
}

fn from_5day_result(result: Result<[Weather; 5], Exception>) -> HttpResponse {
    unimplemented!()
}
