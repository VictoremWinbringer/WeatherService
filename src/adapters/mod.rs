use crate::entities::Exception;
use reqwest::Response;
use serde::de::DeserializeOwned;
use crate::entities::{Weather, Period};
use std::collections::HashMap;
use std::time::{Instant, Duration};

pub mod accu_weather;
pub mod open_weather_map;

pub trait IWeatherAdapter {
    fn get_forecast(&self, city: &str, country_code: &str, period: Period) -> Result<Vec<Weather>, Exception>;
}

pub struct AccumaWeatherAdapter;

impl IWeatherAdapter for AccumaWeatherAdapter {
    fn get_forecast(&self, city: &str, country_code: &str, period: Period) -> Result<Vec<Weather>, Exception> {
        let city_id = accu_weather::city(city, country_code)?.first().ok_or(Exception::AccuWeatherCityNotFound(city.to_owned()))?.key.clone();
        let forecast = match period {
            Period::For5Day => accu_weather::dayly_5day(&city_id),
            Period::For1Day => accu_weather::dayly_1day(&city_id),
        }?;
        let temp = forecast.daily_forecasts
            .iter()
            .map(|d| Weather { temperature: (d.temperature.max.value + d.temperature.min.value) / 2.0 })
            .collect();
        Ok(temp)
    }
}

pub struct OpenWeatherMapAdapter;

impl IWeatherAdapter for OpenWeatherMapAdapter {
    fn get_forecast(&self, city: &str, country_code: &str, period: Period) -> Result<Vec<Weather>, Exception> {
        match period {
            Period::For1Day => {
                let weather = self.daily_1day(city, country_code)?;
                Ok(vec![weather])
            }
            Period::For5Day => self.daily_5day(city, country_code),
        }
    }
}

impl OpenWeatherMapAdapter {
    fn daily_1day(&self, city: &str, country_code: &str) -> Result<Weather, Exception> {
        let forecast = open_weather_map::dayly_1day(city, country_code)?;
        Ok(Weather { temperature: forecast.main.temp })
    }

    fn daily_5day(&self, city: &str, country_code: &str) -> Result<Vec<Weather>, Exception> {
        let mut i = 0;
        let forecasts = open_weather_map::dayly_5day(city, country_code)?.list
            .iter()
            .map(|f| Weather { temperature: f.main.temp })
            .fold(Vec::<Weather>::new(), |a, weather| {
                let mut acc = a;
                i += 1;
                if i == 1 {
                    acc.push(weather)
                } else {
                    acc.last_mut().map(|w| w.temperature = (w.temperature + weather.temperature) / 2.0);
                }
                if i % 8 == 0 {
                    i = 0;
                }
                acc
            });
        Ok(forecasts)
    }
}

#[derive(Debug, Clone)]
pub struct TtlWeather<T: Clone> {
    weather: T,
    expiration: Instant,
}

pub struct CacheAdapter<T: Clone> {
    cache: HashMap<String, TtlWeather<T>>,
    expiration: Duration,
}

impl<T> CacheAdapter<T> where T: Clone {
    pub fn new(expiration: Duration) -> Self {
        CacheAdapter { cache: HashMap::new(), expiration }
    }

    fn get_key(&self, country: &str, city: &str, period: &str) -> String {
        format!("/{}/{}/{}/", country, city, period)
    }

    pub fn clear_expired(&mut self) {
        self.cache = self.cache.iter().filter(|(_x, y)| y.expiration > Instant::now())
            .map(|(x, y)| (x.to_owned(), y.clone()))
            .collect();
    }

    pub fn add(&mut self, country: &str, city: &str, period: &str, value: T) {
        let key = self.get_key(country, city, period);
        let value = TtlWeather { weather: value, expiration: Instant::now() + self.expiration };
        self.cache.insert(key, value);
    }

    pub fn get(&self, country: &str, city: &str, period: &str) -> Option<T> {
        let key = self.get_key(country, city, period);
        self.cache.get(&key).map(|x| x.clone().weather)
    }
}

fn try_parse<T: DeserializeOwned>(mut response: Response) -> Result<T, Exception> {
    if response.status() == 200 {
        match response.json() {
            Ok(weather) => Ok(weather),
            _ => Err(Exception::ErrorMessage(response.text()?))
        }
    } else {
        Err(Exception::JsonError(response.text()?))
    }
}