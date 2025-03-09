mod config;
mod format_middleware;
mod handlers;
mod models;
mod util;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};
use env_logger::Env;
use maxminddb::Reader;
use std::sync::Arc;
use std::time::Duration;

struct AppState {
    geo_db: Reader<Vec<u8>>,
    asn_db: Reader<Vec<u8>>,
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = config::load_config("config.toml").expect("Failed to load configuration");

    let geo_reader =
        Reader::open_readfile(&config.geo_db_path).expect("Could not open GeoLite2 City database");
    let asn_reader =
        Reader::open_readfile(&config.asn_db_path).expect("Could not open GeoLite2 ASN database");

    let shared_state = Arc::new(AppState {
        geo_db: geo_reader,
        asn_db: asn_reader,
    });

    println!("Starting ipeek on http://{}", config.server_address);

    env_logger::Builder::from_env(Env::default().default_filter_or("info")).init();

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(shared_state.clone()))
            .wrap(Logger::new("%a %r %s %D"))
            .configure(handlers::init_routes)
    })
    .client_request_timeout(Duration::from_secs(30))
    .bind(&config.server_address)?
    .run()
    .await
}
