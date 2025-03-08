use std::net::IpAddr;
use actix_web::{web, HttpRequest, HttpResponse};
use maxminddb::Reader;
use crate::models::{SimpleResponse, QueryOptions};
use crate::util::{format_response, get_ip};

pub async fn country_handler(
    req: HttpRequest,
    geo_db: web::Data<Reader<Vec<u8>>>,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    let result = get_country(get_ip(&req), &geo_db).unwrap_or_else(|| "unknown".to_string());
    let response = SimpleResponse { value: result };
    format_response(query.format.as_deref(), &response)
}

pub fn get_country(ip: IpAddr, geo_db: &Reader<Vec<u8>>) -> Option<String> {
    let city_record: Option<crate::models::CityRecord> = geo_db.lookup(ip).ok();
    city_record.as_ref()
        .and_then(|record| record.country.as_ref())
        .and_then(|c| c.names.as_ref())
        .and_then(|names| names.get("en").cloned())
}