
use crate::entities::Exception;
use reqwest::Response;
use serde::de::DeserializeOwned;
use crate::entities::Weather;

pub mod accu_weather;
pub mod open_weather_map;

trait IWeatherAdapter {
    fn daily_1day(&self, city: &str, country_code: &str) -> Result<Weather, Exception>;
    fn daily_5day(&self, city: &str, country_code: &str) -> Result<Vec<Weather>, Exception>;
}

pub struct AccumaWeatherAdapter;

impl IWeatherAdapter for AccumaWeatherAdapter {
    fn daily_1day(&self, city: &str, country_code: &str) -> Result<Weather, Exception> {
        let city_id = accu_weather::city(city, country_code)?.first().ok_or(Exception::AccuWeatherCityNotFound(city.to_owned()))?.key.clone();
        let forecast = accu_weather::dayly_1day(&city_id)?;
        let temp = forecast.daily_forecasts.first().ok_or(Exception::AccuWeatherForecastNotFound(city_id.to_owned()))?;
        Ok(Weather { temperature: (temp.temperature.max.value + temp.temperature.min.value) / 2.0 })
    }

    fn daily_5day(&self, city: &str, country_code: &str) -> Result<Vec<Weather>, Exception> {
        let city_id = accu_weather::city(city, country_code)?.first().ok_or(Exception::AccuWeatherCityNotFound(city.to_owned()))?.key.clone();
        let forecast = accu_weather::dayly_5day(&city_id)?;
        let temp = forecast.daily_forecasts
            .iter()
            .map(|d| Weather { temperature: (d.temperature.max.value + d.temperature.min.value) / 2.0 })
            .collect();
        Ok(temp)
    }
}

fn try_parse<T: DeserializeOwned>(mut response: Response) -> Result<T, Exception> {
    if response.status() == 200 {
        let weather: T = response.json()?;
        Ok(weather)
    } else {
        Err(Exception::ErrorMessage(response.text()?))
    }
}

pub struct OpenWeatherMapAdapter;

impl IWeatherAdapter for OpenWeatherMapAdapter {
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