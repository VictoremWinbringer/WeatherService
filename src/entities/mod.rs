use std::fmt::Formatter;
use core::fmt::Display;

#[derive(Debug)]
pub enum Exception {
    AccuWeatherCityNotFound(String),
    AccuWeatherForecastNotFound(String),
    RequestError(reqwest::Error)
}

impl std::error::Error for Exception {

}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Exception::AccuWeatherCityNotFound( city ) => write!(f,"Wether forecast not found for city: {}", city),
            Exception::AccuWeatherForecastNotFound( city_id ) => write!(f,"Wether forecast not found for city with id: {}", city_id),
            Exception::RequestError(err) => err.fmt(f),
            _ => write!(f, "{:#?}", self),
        }
    }
}

impl std::convert::From<reqwest::Error> for Exception {
    fn from(err: reqwest::Error) -> Self {
        Exception::RequestError(err)
    }
}