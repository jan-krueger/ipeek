use std::net::IpAddr;
use std::sync::Arc;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use maxminddb::{geoip2, Reader};
use crate::AppState;
use crate::format_middleware::Format;
use crate::models::{SimpleResponse};
use crate::util::{format_response, get_ip};

pub async fn country_handler(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
) -> HttpResponse {
    format_response(req.extensions().get::<Format>().unwrap(), &get_country_response(&req, &state))
}

pub fn get_country(ip: IpAddr, geo_db: &Reader<Vec<u8>>) -> Option<String> {
    geo_db.lookup::<geoip2::City>(ip).ok()?.country?
        .names?
        .get("en")
        .cloned()
        .map(String::from)
}

pub fn get_country_response(req: &HttpRequest, state: &web::Data<Arc<AppState>>) -> SimpleResponse {
    let result = get_country(get_ip(&req), &state.geo_db).unwrap_or_else(|| "".to_string());
    SimpleResponse { value: result }
}