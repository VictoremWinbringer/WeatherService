use crate::entities::Exception;

#[derive(Debug, Serialize, Deserialize)]
pub struct Forecast {
    pub  list: Vec<Weather>
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    pub main: TemperatureInfo,
    pub dt: i64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemperatureInfo {
    pub temp: f32
}

const API_ROOT: &'static str = "http://api.openweathermap.org/data/2.5";
const API_KEY: &'static str = "55f8c734fd0830fcfcf21238f256df49";

pub fn dayly_1day(city: &str, country_code: &str) -> Result<Weather, Exception> {
    let path = &format!("{}/weather?appid={}&q={},{}&units=metric&cnt=1", API_ROOT, API_KEY, city, country_code);
    super::try_parse(reqwest::get(path)?)
}

pub fn dayly_5day(city: &str, country_code: &str) -> Result<Forecast, Exception> {
    let path = &format!("{}/forecast?appid={}&q={},{}&units=metric&cnt=40", API_ROOT, API_KEY, city, country_code);
    super::try_parse(reqwest::get(path)?)
}

#[cfg(test)]
mod open_weather_map_test {
    use crate::entities::Exception;

    #[test]
    fn test_weather_1day_for_moscow() -> Result<(), Exception> {
        let weather = super::dayly_1day("Moscow", "ru")?;
        assert!(true, "{:?}", weather);
        Ok(())
    }

    #[test]
    fn test_weather_5day_for_moscow() -> Result<(), Exception> {
        let forecast = super::dayly_5day("Moscow", "RU")?;
        let path = &format!("{}/forecast?appid={}&q={},{}&units=metric&cnt=3", super::API_ROOT, super::API_KEY, "Moscow", "RU");
        assert!(forecast.list.len() <= 40, "{:?}, {}, {}", forecast, forecast.list.len(), path);
        Ok(())
    }
}