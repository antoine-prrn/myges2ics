extern crate reqwest;
mod config;
mod auth;
mod cal;
mod handlers;
mod projects;
use actix_web::{App, HttpServer};
#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenvy::dotenv().ok();
    let config = config::Config::init();
    let _ = HttpServer::new(|| {
        App::new().service(handlers::get_calendar_handler)
    })
    .bind((config.web_server_url, config.webserver_port))?
    .run()
    .await;

    Ok(())
}