use std::net::{IpAddr};
use actix_web::{web, HttpRequest, HttpResponse};
use crate::models::{SimpleResponse};
use crate::util::{format_response, get_ip, QueryOptions};
use crate::config::DNS_RESOLVER;

pub async fn reverse_dns_handler(
    req: HttpRequest,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    let result = get_reverse_dns(get_ip(&req)).await.unwrap();
    let response = SimpleResponse { value: result };
    format_response(query.format.as_deref(), &response)
}
pub async fn get_reverse_dns(ip: IpAddr) -> Option<String> {
    let ptr_lookup = DNS_RESOLVER.reverse_lookup(ip).await.ok()?;
    Some(ptr_lookup.iter().next()?.to_string())
}