use reqwest;
use weather_service;
use std::thread;
use actix_web::http::StatusCode;

#[test]
fn get_weather_today() -> Result<(), Box<std::error::Error>> {
    let _ = thread::spawn(|| {
        weather_service::run("127.0.0.1:8080");
    });
    let mut response = reqwest::get("http://127.0.0.1:8080/api/v1/weather/RU/test_city/1day")?;
    assert!(response.status() == 200, "{}", response.text()?);
    assert_eq!(r#"{"temperature":3.0}"#, response.text()?);
    Ok(())
}

#[test]
fn get_weather_for_week() -> Result<(), Box<std::error::Error>> {
    let _ = thread::spawn(|| {
        weather_service::run("127.0.0.1:8081");
    });
    let mut response = reqwest::get("http://127.0.0.1:8081/api/v1/weather/RU/test_city/5day")?;
    assert!(response.status() == 200, "{}", response.text()?);
    assert_eq!(r#"[{"temperature":1.5},{"temperature":3.0},{"temperature":4.5},{"temperature":6.0},{"temperature":7.5}]"#, response.text()?);
    Ok(())
}

#[test]
fn get_weather_service_should_return_not_found_for_uncorrected_period() -> Result<(), Box<std::error::Error>> {
    let _ = thread::spawn(|| {
        weather_service::run("127.0.0.1:8082");
    });
    let mut response = reqwest::get("http://127.0.0.1:8082/api/v1/weather/RU/test_city/sasdfa")?;
    assert!(response.status() == 404, "{}", response.text()?);
    Ok(())
}

#[test]
fn get_weather_service_should_return_error_for_to_short_country_code() -> Result<(), Box<std::error::Error>> {
    let _ = thread::spawn(|| {
        weather_service::run("127.0.0.1:8083");
    });
    let mut response = reqwest::get("http://127.0.0.1:8083/api/v1/weather/R/test_city/1day")?;
    assert!(response.status() == 500, "{}", response.text()?);
    Ok(())
}

#[test]
fn get_weather_service_should_return_error_for_to_long_country_code() -> Result<(), Box<std::error::Error>> {
    let _ = thread::spawn(|| {
        weather_service::run("127.0.0.1:8084");
    });
    let mut response = reqwest::get("http://127.0.0.1:8084/api/v1/weather/RUR/test_city/1day")?;
    assert!(response.status() == 500, "{}", response.text()?);
    Ok(())
}

#[test]
fn get_weather_service_should_return_error_for_invalid_chars_country_code() -> Result<(), Box<std::error::Error>> {
    let _ = thread::spawn(|| {
        weather_service::run("127.0.0.1:8085");
    });
    let mut response = reqwest::get("http://127.0.0.1:8085/api/v1/weather/Rш/test_city/1day")?;
    assert!(response.status() == 500, "{}", response.text()?);
    Ok(())
}

#[test]
fn get_weather_service_should_return_error_for_invalid_chars_city() -> Result<(), Box<std::error::Error>> {
    let _ = thread::spawn(|| {
        weather_service::run("127.0.0.1:8086");
    });
    let mut response = reqwest::get("http://127.0.0.1:8086/api/v1/weather/ru/    /1day")?;
    assert!(response.status() == 500, "{}", response.text()?);
    Ok(())
}

#[test]
fn get_weather_today_moscow() -> Result<(), Box<std::error::Error>> {
    let _ = thread::spawn(|| {
        weather_service::run("127.0.0.1:8087");
    });
    let mut response = reqwest::get("http://127.0.0.1:8087/api/v1/weather/ru/moscow/1day")?;
    assert!(response.status() == 200, "{}", response.text()?);
    Ok(())
}

#[test]
fn get_weather_for_week_moscow() -> Result<(), Box<std::error::Error>> {
    let _ = thread::spawn(|| {
        weather_service::run("127.0.0.1:8088");
    });
    let mut response = reqwest::get("http://127.0.0.1:8088/api/v1/weather/ru/moscow/5day")?;
    assert!(response.status() == 200, "{}", response.text()?);
    Ok(())
}