use std::net::IpAddr;
use std::sync::Arc;
use actix_web::{web, HttpRequest, HttpResponse};
use maxminddb::{geoip2, Reader};
use crate::AppState;
use crate::models::{SimpleResponse};
use crate::util::{format_response, get_ip, QueryOptions};

pub async fn country_code_handler(
    req: HttpRequest,
    state: web::Data<Arc<AppState>>,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    let result = get_country_code(get_ip(&req), &state.geo_db).unwrap_or_else(|| "".to_string());
    let response = SimpleResponse { value: result };
    format_response(query.format.as_deref(), &response)
}

pub fn get_country_code(ip: IpAddr, geo_db: &Reader<Vec<u8>>) -> Option<String> {
    geo_db.lookup::<geoip2::City>(ip).ok()?.country?
        .iso_code
        .map(|code| code.to_string())
}
