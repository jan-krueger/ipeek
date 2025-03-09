use std::net::IpAddr;
use std::sync::Arc;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use maxminddb::{geoip2, Reader};
use crate::AppState;
use crate::models::{SimpleResponse};
use crate::util::{format_response, get_ip};

pub async fn country_code_handler(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
) -> HttpResponse {
    format_response(req.extensions().get::<String>().unwrap(), &get_country_code_response(&req, &state))
}

pub fn get_country_code(ip: IpAddr, geo_db: &Reader<Vec<u8>>) -> Option<String> {
    geo_db.lookup::<geoip2::City>(ip).ok()?.country?
        .iso_code
        .map(|code| code.to_string())
}

pub fn get_country_code_response(req: &HttpRequest, state: &web::Data<Arc<AppState>>) -> SimpleResponse {
    let result = get_country_code(get_ip(&req), &state.geo_db).unwrap_or_else(|| "".to_string());
    SimpleResponse { value: result }
}