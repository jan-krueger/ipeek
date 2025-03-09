pub mod all;
pub mod asn;
mod blocklist;
pub mod city;
pub mod country;
pub mod country_code;
pub mod docs;
pub mod ip;
pub mod region;
pub mod reverse_dns;
pub mod root;

use crate::format_middleware::FormatMiddleware;
use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("")
            .wrap(FormatMiddleware)
            .route("/", web::get().to(root::root_handler))
            .route("/ip", web::get().to(ip::ip_handler))
            .route(
                "/reverse_dns",
                web::get().to(reverse_dns::reverse_dns_handler),
            )
            .route("/country", web::get().to(country::country_handler))
            .route(
                "/country_code",
                web::get().to(country_code::country_code_handler),
            )
            .route("/city", web::get().to(city::city_handler))
            .route("/region", web::get().to(region::region_handler))
            .route("/asn", web::get().to(asn::asn_handler))
            .route("/blocklist", web::get().to(blocklist::blocklist_handler))
            .route("/all", web::get().to(all::all_handler))
            .route("/docs", web::get().to(docs::docs_handler)),
    );
}
