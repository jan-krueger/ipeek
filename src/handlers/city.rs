use crate::format_middleware::Format;
use crate::models::SimpleResponse;
use crate::util::{format_response, get_ip};
use crate::AppState;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use maxminddb::{geoip2, Reader};
use std::net::IpAddr;
use std::sync::Arc;

pub async fn city_handler(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    format_response(
        req.extensions().get::<Format>().unwrap(),
        &get_city_response(&req, &state),
        false,
    )
}

pub fn get_city(ip: IpAddr, geo_db: &Reader<Vec<u8>>) -> Option<String> {
    geo_db
        .lookup::<geoip2::City>(ip)
        .ok()?
        .city?
        .names?
        .get("en")
        .cloned()
        .map(String::from)
}

pub fn get_city_response(req: &HttpRequest, state: &web::Data<Arc<AppState>>) -> SimpleResponse {
    let result = get_city(get_ip(&req), &state.geo_db).unwrap_or_else(|| "".to_string());
    SimpleResponse { value: result }
}
