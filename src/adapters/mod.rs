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
//    fn daily_1day(&self, city: &str, country_code: &str) -> Result<Weather, Exception> {
//        if city == "test_city" {
//            return Ok(Weather { temperature: 2.0 });
//        }
//        let city_id = accu_weather::city(city, country_code)?.first().ok_or(Exception::AccuWeatherCityNotFound(city.to_owned()))?.key.clone();
//        let forecast = accu_weather::dayly_1day(&city_id)?;
//        let temp = forecast.daily_forecasts.first().ok_or(Exception::AccuWeatherForecastNotFound(city_id.to_owned()))?;
//        Ok(Weather { temperature: (temp.temperature.max.value + temp.temperature.min.value) / 2.0 })
//    }
//
//    fn daily_5day(&self, city: &str, country_code: &str) -> Result<[Weather; 5], Exception> {
//        if city == "test_city" {
//            let mut value: Vec<Weather> = Vec::new();
//            for i in 1..6 {
//                value.push(Weather { temperature: i as f32 })
//            }
//            return Ok(from_vec(value));
//        }
//        let city_id = accu_weather::city(city, country_code)?.first().ok_or(Exception::AccuWeatherCityNotFound(city.to_owned()))?.key.clone();
//        let forecast = accu_weather::dayly_5day(&city_id)?;
//        let temp = forecast.daily_forecasts
//            .iter()
//            .map(|d| Weather { temperature: (d.temperature.max.value + d.temperature.min.value) / 2.0 })
//            .collect();
//        Ok(temp)
//    }

    fn get_forecast(&self, city: &str, country_code: &str, period: Period) -> Result<Vec<Weather>, Exception> {
        let city_id = accu_weather::city(city, country_code)?.first().ok_or(Exception::AccuWeatherCityNotFound(city.to_owned()))?.key.clone();
        let forecast = match Period {
            Period::For5Day => accu_weather::dayly_5day(&city_id)?,
            Period::For1Day => accu_weather::dayly_1day(&city_id)?,
        };
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
            Period::For1Day =>{
                let weather = self.daily_1day(city,country_code)?;
                Ok(vec![weather])
            },
            Period::For5Day => self.daily_5day(city, country_code)
        }
    }
}

impl  OpenWeatherMapAdapter {
    fn daily_1day(&self, city: &str, country_code: &str) -> Result<Weather, Exception> {
//        if city == "test_city" {
//            return Ok(Weather { temperature: 4.0 });
//        }
        let forecast = open_weather_map::dayly_1day(city, country_code)?;
        Ok(Weather { temperature: forecast.main.temp })
    }

    fn daily_5day(&self, city: &str, country_code: &str) -> Result<[Weather; 5], Exception> {
//        if city == "test_city" {
//            let mut value: Vec<Weather> = Vec::new();
//            for i in 1..6 {
//                value.push(Weather { temperature: (i * 2) as f32 })
//            }
//            return Ok(from_vec(value));
//        }
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
        Ok(from_vec(forecasts))
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

    fn is_expired(&self, key: &str) -> bool {
        self.cache
            .get(key)
            .map(|ttl| ttl.expiration > Instant::now()).unwrap_or(false)
    }

    fn get_key(&self, country: &str, city: &str, period: &str) -> String {
        format!("/{}/{}/{}/", country, city, period)
    }

    pub fn clear_expired(&mut self) {
        self.cache = self.cache.iter().filter(|(x, y)| y.expiration > Instant::now())
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
        Err(Exception::ErrorMessage(response.text()?))
    }
}