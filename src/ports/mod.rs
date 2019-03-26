use actix_web::{Responder, HttpResponse};
use regex;
use nom::InputIter;

pub trait  IActixWebPort{
    fn get_weather(&self,country_code:&str,city:&str,period:&str) -> impl Responder;
}

pub struct ActixWebPort;

impl IActixWebPort for ActixWebPort{
    fn get_weather(&self,country_code:&str,city:&str,period:&str) -> impl Responder {
       let rgx =  regex::Regex::new("[a-zA-Z]{2}");
        if !rgx.is_match(country_code){
            return HttpResponse::BadRequest()
                .content_type("text/html")
                .body(format!("Country code must by ISO 2 alphabet code like us or ru. Current code is {}!", country_code))
        }
        if city.trim().is_empty() {
            return HttpResponse::BadRequest()
                .content_type("text/html")
                .body(format!("City name is empty {}!", city))
        }

        match period {
            "1day" => HttpResponse::Ok()
                .content_type("text/html")
                .header("Cache-Control","no-cache")
                .body(format!("Weather in Country:{} City: {}  for Period {}", country_code, city, period)),
            "5day" => HttpResponse::Ok()
                .content_type("text/html")
                .header("Cache-Control","no-cache")
                .body(format!("Weather in Country:{} City: {}  for Period {}", country_code, city, period)),
            _ =>  HttpResponse::NotFound()
                .content_type("text/html")
                .body(format!("Not found period {}!", period))
        }
    }
}