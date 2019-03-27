use crate::entities::{Weather, Exception};
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use crate::adapters::{IWeatherAdapter, AccumaWeatherAdapter, OpenWeatherMapAdapter, CacheAdapter};
use serde_json::error::ErrorCode::ExpectedColon;

pub trait IWeatherService {
    fn daily_1day(&self, city: &str, country_code: &str) -> Result<Weather, Exception>;
    fn daily_5day(&self, city: &str, country_code: &str) -> Result<[Weather; 5], Exception>;
}

pub struct WeatherService {
    sources: Vec<Box<dyn IWeatherAdapter>>,
    cache_1day: Arc<RwLock<CacheAdapter<Weather>>>,
    cache_5day: Arc<RwLock<CacheAdapter<[Weather; 5]>>>,
}

impl WeatherService {
    pub fn new(
        sources: Vec<Box<dyn IWeatherAdapter>>,
        cache_1day: Arc<RwLock<CacheAdapter<Weather>>>,
        cache_5day: Arc<RwLock<CacheAdapter<[Weather; 5]>>>,
    ) -> WeatherService {
        WeatherService { sources, cache_1day, cache_5day }
    }
}

enum Period {
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

impl IWeatherService for WeatherService {
    fn daily_1day(&self, city: &str, country_code: &str) -> Result<Weather, Exception> {
        validate_city(city)?;
        validate_country_code(country_code)?;
        self.cache_1day.write().unwrap().clear_expired();
        if let Some(weather) = self.cache_1day.read().unwrap().get(country_code, city, Period::For1Day.into()) {
            return Ok(weather);
        }
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
            .map(|w| {
                self.cache_1day.write().unwrap().add(country_code, city, Period::For1Day.into(), w.clone());
                w
            })
    }

    fn daily_5day(&self, city: &str, country_code: &str) -> Result<[Weather; 5], Exception> {
        validate_city(city)?;
        validate_country_code(country_code)?;
        self.cache_5day.write().unwrap().clear_expired();
        if let Some(weather) = self.cache_5day.read().unwrap().get(country_code, city, Period::For5Day.into()) {
            return Ok(weather);
        }
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
            .map(|w| {
                self.cache_5day.write().unwrap().add(country_code, city, Period::For5Day.into(), w.clone());
                w
            })
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
    let rgx = regex::Regex::new("^[a-zA-Z]{2}$")?;

    if !rgx.is_match(country_code) {
        return Err(Exception::ErrorMessage(format!("Country code must by ISO 2 alphabet code like us or ru. Current code is {}!", country_code)));
    }
    Ok(())
}

#[cfg(test)]
pub mod domain_tests {
    use super::WeatherService;
    use std::sync::{Arc, Mutex, RwLock};
    use std::collections::HashMap;
    use crate::domain_logic::IWeatherService;
    use crate::adapters::{AccumaWeatherAdapter, OpenWeatherMapAdapter, CacheAdapter};
    use crate::entities::Exception;
    use std::time::Duration;

    #[test]
    pub fn datly_1day() -> Result<(), Exception> {
        let service = WeatherService::new(
            vec![
                Box::new(AccumaWeatherAdapter),
                Box::new(OpenWeatherMapAdapter),
            ],
            Arc::new(RwLock::new(CacheAdapter::new(Duration::new(60 * 60 * 24, 0)))),
            Arc::new(RwLock::new(CacheAdapter::new(Duration::new(60 * 60 * 24, 0)))),
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
            Arc::new(RwLock::new(CacheAdapter::new(Duration::new(60 * 60 * 24, 0)))),
            Arc::new(RwLock::new(CacheAdapter::new(Duration::new(60 * 60 * 24, 0)))),
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