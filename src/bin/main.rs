use weather_service;

fn main() -> Result<(),Box<dyn std::error::Error>> {
   weather_service::run("0.0.0.0:22222")
}