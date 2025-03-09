use crate::format_middleware::Format;
use crate::handlers::asn::get_asn_info;
use crate::handlers::blocklist::get_blocklist;
use crate::handlers::city::get_city;
use crate::handlers::country::get_country;
use crate::handlers::country_code::get_country_code;
use crate::handlers::region::get_region;
use crate::handlers::reverse_dns::get_reverse_dns;
use crate::models::AllResponse;
use crate::util::{format_response, get_ip};
use crate::AppState;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use std::sync::Arc;

pub async fn all_handler(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    let info = get_all_response(&req, &state).await;
    format_response(req.extensions().get::<Format>().unwrap(), &info, false)
}

pub async fn get_all_response(req: &HttpRequest, state: &web::Data<Arc<AppState>>) -> AllResponse {
    let ip = get_ip(&req);
    let reverse_dns = get_reverse_dns(ip).await.unwrap_or("".to_string());
    let country = get_country(ip, &state.geo_db).unwrap_or("".to_string());
    let country_code = get_country_code(ip, &state.geo_db).unwrap_or("".to_string());
    let region = get_region(ip, &state.geo_db).unwrap_or("".to_string());
    let city = get_city(ip, &state.geo_db).unwrap_or("".to_string());
    let asn = get_asn_info(&req, &state.asn_db);
    let blocklist = get_blocklist(&ip).await;

    AllResponse {
        ip: ip.to_string(),
        reverse_dns,
        country,
        country_code,
        region,
        city,
        asn,
        blocklist,
    }
}
