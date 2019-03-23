use crate::adapters::accu_weather::City;
use std::fmt::{Error, Formatter};
use crate::entities::Exception;

pub mod accu_weather;
pub mod open_weather_map;

pub struct Weather{
    date:String,
    temperature:f32
}

trait IWeatherAdapter {
    fn daily_1day(city:&str) -> Result<Weather, Exception>;
    fn daily_5day(city:&str) -> Result<Vec<Weather>, Exception>;
}

pub struct AccumaWeatherAdapter;

impl IWeatherAdapter for AccumaWeatherAdapter {
    fn daily_1day(city: &str) -> Result<Weather, Exception> {
       let city_id= accu_weather::city(city)?.first().ok_or(Exception::AccuWeatherCityNotFound(city.to_owned()))?.key.clone();
        let forecast = accu_weather::dayly_1day(&city_id)?;
        let temp = forecast.daily_forecasts.first().ok_or(Exception::AccuWeatherForecastNotFound(city_id.to_owned()))?;
      Ok(Weather{ date: temp.date.clone(), temperature:(temp.temperature.max.value + temp.temperature.min.value)/2.0})
    }

    fn daily_5day(city: &str) -> Result<Vec<Weather>, Exception> {
        let city_id= accu_weather::city(city)?.first().ok_or(Exception::AccuWeatherCityNotFound(city.to_owned()))?.key.clone();
        let forecast = accu_weather::dayly_5day(&city_id)?;
        let temp = forecast.daily_forecasts
            .iter()
            .map(|d| Weather{ date: d.date.clone(), temperature:(d.temperature.max.value + d.temperature.min.value)/2.0})
            .collect();
        Ok(temp)
    }
}