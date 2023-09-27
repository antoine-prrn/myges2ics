#[derive(Debug, Clone)]
pub struct Config {
    pub webserver_port: u16,
    pub days_to_fetch: u64,
    pub days_before_to_fetch: u64,
    pub web_server_url: String,
}

impl Config {
    pub fn init() -> Config {
        let webserver_port = std::env::var("WEBSERVER_PORT").expect("WEBSERVER_PORT not set").parse::<u16>().unwrap();
        let days_to_fetch = std::env::var("DAYS_TO_FETCH").expect("DAYS_TO_FETCH not set").parse::<u64>().unwrap();
        let days_before_to_fetch = std::env::var("DAYS_BEFORE_TO_FETCH").expect("DAYS_BEFORE_TO_FETCH not set").parse::<u64>().unwrap();
        let web_server_url = std::env::var("WEB_SERVER_URL").expect("WEB_SERVER_URL not set");
        Config {
            webserver_port,
            days_to_fetch,
            days_before_to_fetch,
            web_server_url,
        }
    }
}