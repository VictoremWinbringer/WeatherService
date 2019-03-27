use std::fmt::Formatter;

#[derive(Debug)]
pub enum Exception {
    AccuWeatherCityNotFound(String),
    AccuWeatherForecastNotFound(String),
    RequestError(reqwest::Error),
    ErrorMessage(String),
    RegexError(regex::Error),
    UnknownPeriod,
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
            Exception::UnknownPeriod => write!(f,"Unknown period!",),
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
    Unknown
}

impl From<Period> for &'static str {
    fn from(period: Period) -> Self {
        match period {
            Period::For1Day => "1day",
            Period::For5Day => "5day",
            Period::Unknown => "Unknown"
        }
    }
}

impl From<&str> for Period {
    fn from(period: &str) -> Self {
        match period {
           "1day" => Period::For1Day,
           "5day" => Period::For5Day,
                _ => Period::Unknown,
        }
    }
}