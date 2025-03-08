use std::net::IpAddr;
use std::sync::Arc;
use actix_web::{web, HttpRequest, HttpResponse};
use maxminddb::{geoip2, Reader};
use crate::AppState;
use crate::models::{SimpleResponse, QueryOptions};
use crate::util::{format_response, get_ip};

pub async fn country_handler(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    let result = get_country(get_ip(&req), &state.geo_db).unwrap_or_else(|| "".to_string());
    let response = SimpleResponse { value: result };
    format_response(query.format.as_deref(), &response)
}

pub fn get_country(ip: IpAddr, geo_db: &Reader<Vec<u8>>) -> Option<String> {
    geo_db.lookup::<geoip2::City>(ip).ok()?.country?
        .names?
        .get("en")
        .cloned()
        .map(String::from)
}