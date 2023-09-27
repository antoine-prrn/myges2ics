#[derive(Debug, Clone)]
pub struct Config {
    pub webserver_port: u16,
    pub days_to_fetch: u64,
    pub days_before_to_fetch: u64,
    pub web_server_url: String,
}

impl Config {
    pub fn init() -> Config {
    let webserver_port = std::env::var("WEBSERVER_PORT")
        .unwrap_or("8080".to_string()) // Valeur par défaut si WEBSERVER_PORT n'est pas défini
        .parse::<u16>()
        .expect("Unable to parse WEBSERVER_PORT");

    let days_to_fetch = std::env::var("DAYS_TO_FETCH")
        .unwrap_or("60".to_string()) // Valeur par défaut si DAYS_TO_FETCH n'est pas défini
        .parse::<u64>()
        .expect("Unable to parse DAYS_TO_FETCH");

    let days_before_to_fetch = std::env::var("DAYS_BEFORE_TO_FETCH")
        .unwrap_or("7".to_string())
        .parse::<u64>()
        .expect("Unable to parse DAYS_BEFORE_TO_FETCH");

    let web_server_url = std::env::var("WEB_SERVER_URL")
        .unwrap_or("0.0.0.0".to_string())
        .to_string();
        Config {
            webserver_port,
            days_to_fetch,
            days_before_to_fetch,
            web_server_url,
        }
    }
}