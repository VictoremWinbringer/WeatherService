use crate::entities::{Weather, Exception, Period};
use std::sync::{Arc, Mutex, RwLock};
use std::collections::HashMap;
use crate::adapters::{IWeatherAdapter, AccumaWeatherAdapter, OpenWeatherMapAdapter, CacheAdapter};
use serde_json::error::ErrorCode::ExpectedColon;

pub trait IWeatherService {
    fn get_forecast(&self, city: &str, country_code: &str, period: Period) -> Result<Vec<Weather>, Exception>;
}

pub struct WeatherService {
    sources: Vec<Box<dyn IWeatherAdapter>>,
    cache: Arc<RwLock<CacheAdapter<Vec<Weather>>>>,
}

impl WeatherService {
    pub fn new(
        sources: Vec<Box<dyn IWeatherAdapter>>,
        cache: Arc<RwLock<CacheAdapter<Vec<Weather>>>>,
    ) -> WeatherService {
        WeatherService { sources, cache }
    }
}

impl IWeatherService for WeatherService {
    fn get_forecast(&self, city: &str, country_code: &str, period: Period) -> Result<Vec<Weather>, Exception> {
        validate_city(city)?;
        validate_country_code(country_code)?;
        if let Some(weather) = self.cache.read().unwrap().get(country_code, city, period.into()) {
            return Ok(weather);
        }
        let mut weathers: Vec<Result<Vec<Weather>, Exception>> = self.sources
            .iter()
            .map(|wa| wa.get_forecast(city, country_code, period))
            .collect();
        let seed = weathers
            .pop()
            .ok_or(Exception::ErrorMessage("Empty weathers in forecast".to_owned()))?;
        weathers
            .into_iter()
            .fold(seed, |seed, current| zip_weathers(seed, current))
            .map(|w| {
                self.cache.write().unwrap().add(country_code, city, period.into(), w.clone());
                w
            })
    }
}

fn zip_weathers(lhs: Result<Vec<Weather>, Exception>, rhs: Result<Vec<Weather>, Exception>) -> Result<Vec<Weather>, Exception> {
    let x = lhs?;
    let y = rhs?;
    let weathers = x
        .iter()
        .zip(&y)
        .map(|(a, b)| Weather { temperature: (a.temperature + b.temperature) / 2.0 })
        .collect();
    Ok(weathers)
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
    use crate::adapters::{AccumaWeatherAdapter, OpenWeatherMapAdapter, CacheAdapter, IWeatherAdapter};
    use crate::entities::{Exception, Weather, Period};
    use std::time::Duration;

    struct TestWeatherAdapter {
        for_1day: Vec<Weather>,
        for_5day: Vec<Weather>,
    }

    impl IWeatherAdapter for TestWeatherAdapter {
        fn get_forecast(&self, city: &str, country_code: &str, period: Period) -> Result<Vec<Weather>, Exception> {
            match period {
                Period::For1Day => Ok(self.for_1day.clone()),
                Period::For5Day => Ok(self.for_5day.clone()),
            }
        }
    }

    fn create_service() -> WeatherService {
        WeatherService::new(
            vec![
                Box::new(TestWeatherAdapter {
                    for_1day: vec![Weather { temperature: 2.0 }],
                    for_5day: {
                        let mut value: Vec<Weather> = Vec::new();
                        for i in 1..6 {
                            value.push(Weather { temperature: i as f32 })
                        }
                        value
                    },
                }),
                Box::new(TestWeatherAdapter {
                    for_1day: vec![Weather { temperature: 4.0 }],
                    for_5day: {
                        let mut value: Vec<Weather> = Vec::new();
                        for i in 1..6 {
                            value.push(Weather { temperature: (i * 2) as f32 })
                        }
                        value
                    },
                }),
            ],
            Arc::new(RwLock::new(CacheAdapter::new(Duration::new(60 * 60 * 24, 0)))),
        )
    }

    #[test]
    pub fn datly_1day() -> Result<(), Exception> {
        let service = create_service();
        let forecast = service.get_forecast("test_city", "ru", Period::For1Day)?;
        assert!(forecast.first().unwrap().temperature == 3.0, "{:?}", forecast);
        Ok(())
    }

    #[test]
    pub fn datly_5day() -> Result<(), Exception> {
        let service = create_service();
        let forecast = service.get_forecast("test_city", "ru", Period::For5Day)?;
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