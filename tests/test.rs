use reqwest;
use weather_service;
use std::thread;

#[test]
fn get_weather_today() -> Result<(), Box<std::error::Error>> {
    let _handerl = thread::spawn (||{
        weather_service::run("127.0.0.1:8080");
    });
    let result = reqwest::get("http://127.0.0.1:8080/api/v1/weather/Moscow City/Today")?.text()?;
    assert!(false, result);
    Ok(())
}

#[test]
fn get_weather_for_week() -> Result<(), Box<std::error::Error>> {
    let _handerl = thread::spawn (||{
        weather_service::run("127.0.0.1:8081");
    });
    let result = reqwest::get("http://127.0.0.1:8081/api/v1/weather/Moscow/Week")?.text()?;
    assert!(false, result);
    Ok(())
}

