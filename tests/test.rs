use reqwest;
use weather_service;
use std::thread;
use actix_web::http::StatusCode;

#[test]
fn get_weather_today() -> Result<(), Box<std::error::Error>> {
    let _ = thread::spawn (||{
        weather_service::run("127.0.0.1:8080");
    });
    let mut response = reqwest::get("http://127.0.0.1:8080/api/v1/weather/RU/Moscow/1day")?;
    assert!(response.status() == StatusCode::OK, "{}, {}", response.text()?, response.status());
    Ok(())
}

#[test]
fn get_weather_for_week() -> Result<(), Box<std::error::Error>> {
    let _ = thread::spawn (||{
        weather_service::run("127.0.0.1:8081");
    });
    let mut response = reqwest::get("http://127.0.0.1:8081/api/v1/weather/RU/Moscow/5day")?;
    assert!(response.status() == StatusCode::OK, "{},{}", response.text()?, response.status());
    Ok(())
}

