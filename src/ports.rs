use actix_web::{HttpResponse, HttpRequest, Path, FromRequest};
use crate::entities::{Weather, Exception, Period};
use crate::domain_logic::IWeatherService;
use actix_web::dev::{Handler, HttpResponseBuilder};

pub trait IActixWebPort<T: IWeatherService> {
    fn get_weather(&self, country_code: &str, city: &str, period: &str) -> HttpResponse;
}

pub struct ActixWebPort<T: IWeatherService> {
    pub  service: T
}

impl<T> IActixWebPort<T> for ActixWebPort<T> where T: IWeatherService {
    fn get_weather(&self, country_code: &str, city: &str, period: &str) -> HttpResponse {
        from_result(self.service.get_forecast(city, country_code, period))
    }
}

fn from_result(result: Result<Vec<Weather>, Exception>) -> HttpResponse {
    match result {
        Ok(weather) => HttpResponse::Ok().json(weather),
        Err(e) => {
            error!("{}", e);
            match e {
                Exception::NotValidCountryCode(message) => html_response(HttpResponse::BadRequest(), message),
                Exception::NotValidCityName(message) => html_response(HttpResponse::BadRequest(), message),
                Exception::AccuWeatherCityNotFound(message) => html_response(HttpResponse::BadRequest(), message),
                Exception::PeriodNotFound(message) => html_response(HttpResponse::BadRequest(), message),
                _ => html_response(HttpResponse::InternalServerError(), "Please try later or write ticket to support".to_owned())
            }
        }
    }
}

fn html_response(mut builder: HttpResponseBuilder, text: String) -> HttpResponse {
    builder
        .content_type("text/html")
        .body(text)
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