pub mod root;
pub mod ip;
pub mod reverse_dns;
pub mod country;
pub mod city;
pub mod region;
pub mod asn;
pub mod all;
pub mod doc;
mod blacklist;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg
        .route("/", web::get().to(root::root_handler))
        .route("/ip", web::get().to(ip::ip_handler))
        .route("/reverse_dns", web::get().to(reverse_dns::reverse_dns_handler))
        .route("/country", web::get().to(country::country_handler))
        .route("/city", web::get().to(city::city_handler))
        .route("/region", web::get().to(region::region_handler))
        .route("/asn", web::get().to(asn::asn_handler))
        .route("/blacklist", web::get().to(blacklist::blacklist_handler))
        .route("/all", web::get().to(all::all_handler))
        .route("/docs", web::get().to(doc::doc_handler));
}
