use crate::entities::{Weather, Exception};

trait IWeatherService {
    fn daily_1day(&self, city: &str, country_code: &str) -> Result<Weather, Exception>;
    fn daily_5day(&self, city: &str, country_code: &str) -> Result<Vec<Weather>, Exception>;
}

struct WeatherService{
    cache: Arc<Mutex<HashMap<String, HashMap<String, HashMap<String, Weather>>>>>,
    sources: Vec<Box<dyn IWeatherAdapter>>,
}

impl WeatherService {
    fn new(
        cache: Arc<Mutex<HashMap<String, HashMap<String, HashMap<String, Weather>>>>>,
        sources: Vec<Box<dyn IWeatherAdapter>>,
    ) -> WeatherService {
        WeatherService { cache, sources }
    }
}

impl IWeatherService for WeatherService {
    fn daily_1day(&self, city: &str, country_code: &str) -> Result<Weather, Exception> {
        unimplemented!() // self.sources.iter().map(|wa| wa.daily_1day(city, country_code))
    }

    fn daily_5day(&self, city: &str, country_code: &str) -> Result<Vec<Weather>, Exception> {
        unimplemented!()
    }
}


#[cfg(test)]
pub mod domain_tests{
    #[test]
    pub fn datly_1day(){
        assert!(false,"FFFF");
       // let service = WeatherService::new(Arc::new(Mutex::new(HashMap::new(HashMap::new(HashMap::new())))));
    }

    #[test]
    pub fn datly_5day(){

    }
}