mod config;
mod models;
mod util;
mod handlers;

use std::time::Duration;
use actix_web::{web, App, HttpServer};
use actix_web::middleware::Logger;
use env_logger::Env;
use maxminddb::Reader;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::load_config("config.toml").expect("Failed to load configuration");

    let geo_reader = Reader::open_readfile(&config.geo_db_path)
        .expect("Could not open GeoLite2 City database");
    let geo_data = web::Data::new(geo_reader);

    let asn_reader = Reader::open_readfile(&config.asn_db_path)
        .expect("Could not open GeoLite2 ASN database");
    let asn_data = web::Data::new(asn_reader);

    println!("Starting ipeek on http://{}", config.server_address);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .app_data(geo_data.clone())
            .app_data(asn_data.clone())
            .wrap(Logger::new("%a %r %s %D"))
            .configure(handlers::init_routes)
        })
        .client_request_timeout(Duration::from_secs(30))
        .bind(&config.server_address)?
        .run()
        .await
}
