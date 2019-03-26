use crate::entities::{Weather, Exception};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use crate::adapters::{IWeatherAdapter, AccumaWeatherAdapter, OpenWeatherMapAdapter};
use crate::weather;
use serde_json::error::ErrorCode::ExpectedColon;

pub trait IWeatherService {
    fn daily_1day(&self, city: &str, country_code: &str) -> Result<Weather, Exception>;
    fn daily_5day(&self, city: &str, country_code: &str) -> Result<[Weather; 5], Exception>;
}

pub struct WeatherService {
    sources: Vec<Box<dyn IWeatherAdapter>>,
}

impl WeatherService {
   pub fn new(
        sources: Vec<Box<dyn IWeatherAdapter>>,
    ) -> WeatherService {
        WeatherService { sources }
    }
}

impl IWeatherService for WeatherService {
    fn daily_1day(&self, city: &str, country_code: &str) -> Result<Weather, Exception> {
        validate_city(city)?;
        validate_country_code(country_code)?;
        let mut weathers: Vec<Result<Weather, Exception>> = self.sources
            .iter()
            .map(|wa| wa.daily_1day(city, country_code))
            .collect();
        let seed = weathers
            .pop()
            .ok_or(Exception::ErrorMessage("Empty weathers in 1day forecast".to_owned()))?;
        weathers
            .into_iter()
            .fold(seed, |seed, current| add_weathers(seed, current))
    }

    fn daily_5day(&self, city: &str, country_code: &str) -> Result<[Weather; 5], Exception> {
        validate_city(city)?;
        validate_country_code(country_code)?;
        let mut weathers: Vec<Result<[Weather; 5], Exception>> = self.sources
            .iter()
            .map(|wa| wa.daily_5day(city, country_code))
            .collect();
        let seed = weathers
            .pop()
            .ok_or(Exception::ErrorMessage("Empty weathers in 1day forecast".to_owned()))?;
        weathers
            .into_iter()
            .fold(seed, |seed, current| zip_weathers(seed, current))
    }
}

fn add_weathers(lhs: Result<Weather, Exception>, rhs: Result<Weather, Exception>) -> Result<Weather, Exception> {
    let x = lhs?;
    let y = rhs?;
    Ok(Weather { temperature: (x.temperature + y.temperature) / 2.0 })
}

fn zip_weathers(lhs: Result<[Weather; 5], Exception>, rhs: Result<[Weather; 5], Exception>) -> Result<[Weather; 5], Exception> {
    let x = lhs?;
    let y = rhs?;
    let weathers = x
        .iter()
        .zip(&y)
        .map(|(a, b)| Weather { temperature: (a.temperature + b.temperature) / 2.0 })
        .collect();
    Ok(crate::adapters::from_vec(weathers))
}

fn validate_city(city: &str) -> Result<(), Exception> {
    if city.trim().is_empty() {
        return Err(Exception::ErrorMessage(format!("City name is empty {}!", city)));
    }
    Ok(())
}

fn validate_country_code(country_code: &str) -> Result<(), Exception> {
    let rgx = regex::Regex::new("[a-zA-Z]{2}")?;

    if !rgx.is_match(country_code) {
        return Err(Exception::ErrorMessage(format!("Country code must by ISO 2 alphabet code like us or ru. Current code is {}!", country_code)));
    }
    Ok(())
}

#[cfg(test)]
pub mod domain_tests {
    use super::WeatherService;
    use std::sync::{Arc, Mutex};
    use std::collections::HashMap;
    use crate::domain_logic::IWeatherService;
    use crate::adapters::{AccumaWeatherAdapter, OpenWeatherMapAdapter};
    use crate::entities::Exception;

    #[test]
    pub fn datly_1day() -> Result<(), Exception> {
        let service = WeatherService::new(
            vec![
                Box::new(AccumaWeatherAdapter),
                Box::new(OpenWeatherMapAdapter),
            ],
        );
        let forecast = service.daily_1day("test_city", "ru")?;
        assert!(forecast.temperature == 3.0, "{:?}", forecast);
        Ok(())
    }

    #[test]
    pub fn datly_5day() -> Result<(), Exception> {
        let service = WeatherService::new(
            vec![
                Box::new(AccumaWeatherAdapter),
                Box::new(OpenWeatherMapAdapter),
            ],
        );
        let forecast = service.daily_5day("test_city", "ru")?;
        assert_eq!(5, forecast.len());
        let mut i = 1.5;
        let step = 1.5;
        for f in forecast.iter() {
            assert_eq!(i, f.temperature);
            i += step;
        }
        Ok(())
    }
}