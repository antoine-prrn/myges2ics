#[derive(Debug, Clone)]
pub struct Config {
    pub webserver_port: u16,
    pub days_to_fetch: u64,
    pub days_before_to_fetch: u64,
}

impl Config {
    pub fn init() -> Config {
        let webserver_port = std::env::var("WEBSERVER_PORT").expect("WEBSERVER_PORT not set").parse::<u16>().unwrap();
        let days_to_fetch = std::env::var("DAYS_TO_FETCH").expect("DAYS_TO_FETCH not set").parse::<u64>().unwrap();
        let days_before_to_fetch = std::env::var("DAYS_BEFORE_TO_FETCH").expect("DAYS_BEFORE_TO_FETCH not set").parse::<u64>().unwrap();
        Config {
            webserver_port,
            days_to_fetch,
            days_before_to_fetch,
        }
    }
}