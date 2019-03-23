use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
pub struct City {
    #[serde(rename = "Key")]
    pub key: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Forecast {
    #[serde(rename = "DailyForecasts")]
    pub daily_forecasts: Vec<DailyForecasts>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DailyForecasts {
    #[serde(rename = "Date")]
    pub date: String,
    #[serde(rename = "Temperature")]
    pub  temperature: Temperature,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Temperature {
    #[serde(rename = "Minimum")]
    pub min: TemperatureValue,
    #[serde(rename = "Maximum")]
    pub max: TemperatureValue,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemperatureValue {
    #[serde(rename = "Value")]
    pub value: f32
}

const API_KEY: &'static str = "C6QDCUZmXMBAho8pi6PFXUmiDeE9AFWV";
const API_ROOT: &'static str = "http://dataservice.accuweather.com";

pub fn city(name: &str) -> Result<Vec<City>, Box<std::error::Error>> {
    let path = &format!("{}/locations/v1/cities/search?apikey={}&q={}", API_ROOT, API_KEY, name);
    let result: Vec<City> = reqwest::get(path)?
        .json()?;
    Ok(result)
}

pub fn dayly_1day(city_id: &str) -> Result<Forecast, Box<std::error::Error>> {
    let path = &format!("{}/forecasts/v1/daily/1day/{}?apikey={}&metric=true", API_ROOT, city_id, API_KEY);
    let result: Forecast = reqwest::get(path)?
        .json()?;
    Ok(result)
}

pub fn dayly_5day(city_id: &str) -> Result<Forecast, Box<std::error::Error>> {
    let path = &format!("{}/forecasts/v1/daily/5day/{}?apikey={}&metric=true", API_ROOT, city_id, API_KEY);
    let result: Forecast = reqwest::get(path)?
        .json()?;
    Ok(result)
}

#[cfg(test)]
mod accu_weather_test {
    use crate::adapters::accu_weather;

    #[test]
    fn test_weather_1day_for_moscow() -> Result<(), Box<std::error::Error>> {
        let cities = accu_weather::city("Moscow")?;
        assert!(cities.len() > 0, "{:?}", cities);
        let dayli = accu_weather::dayly_1day(&cities.first().unwrap().key)?;
        assert!(dayli.daily_forecasts.len() == 1, "{:?}", dayli);
        Ok(())
    }

    #[test]
    fn test_weather_5day_for_moscow() -> Result<(), Box<std::error::Error>> {
        let cities = accu_weather::city("Moscow")?;
        assert!(cities.len() > 0, "{:?}", cities);
        let dayli = accu_weather::dayly_5day(&cities.first().unwrap().key)?;
        assert!(dayli.daily_forecasts.len() == 5, "{:?}", dayli);
        Ok(())
    }
}