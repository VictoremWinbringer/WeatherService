use actix_web::{HttpResponse, HttpRequest, Path, FromRequest};
use crate::entities::{Weather, Exception, Period};
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
        let period= match period {
            "1day" => Period::For1Day,
            "5day" => Period::For5Day,
            _ => {
                return HttpResponse::NotFound().content_type("text/html").body(format!("Not found period {}!", period))
            }
        };
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