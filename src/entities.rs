use std::fmt::Formatter;

#[derive(Debug)]
pub enum Exception {
    AccuWeatherCityNotFound(String),
    AccuWeatherForecastNotFound(String),
    RequestError(reqwest::Error),
    ErrorMessage(String),
    RegexError(regex::Error),
    JsonError(String),
}

impl std::error::Error for Exception {

}

impl std::fmt::Display for Exception {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        match self {
            Exception::AccuWeatherCityNotFound( city ) => write!(f,"Wether forecast not found for city: {}", city),
            Exception::AccuWeatherForecastNotFound( city_id ) => write!(f,"Wether forecast not found for city with id: {}", city_id),
            Exception::ErrorMessage( message ) => write!(f,"Error: {}", message),
            Exception::RequestError(err) => err.fmt(f),
            Exception::JsonError(message) => write!(f,"Json Error: {}",message),
            Exception::RegexError(err) => err.fmt(f),
            _ => write!(f, "{:#?}", self),
        }
    }
}

impl std::convert::From<reqwest::Error> for Exception {
    fn from(err: reqwest::Error) -> Self {
        Exception::RequestError(err)
    }
}

impl std::convert::From<regex::Error> for Exception {
    fn from(err: regex::Error) -> Self {
        Exception::RegexError(err)
    }
}

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Weather {
  pub temperature: f32,
}

#[derive(Debug, Copy, Clone)]
pub enum Period {
    For1Day,
    For5Day,
}

impl From<Period> for &'static str {
    fn from(period: Period) -> Self {
        match period {
            Period::For1Day => "1day",
            Period::For5Day => "5day",
        }
    }
}