use std::net::{IpAddr};
use actix_web::{web, HttpRequest, HttpResponse};
use crate::models::{SimpleResponse};
use crate::util::{format_response, get_ip, QueryOptions};
use crate::config::DNS_RESOLVER;

pub async fn reverse_dns_handler(
    req: HttpRequest,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    format_response(query.format.as_deref(), &get_reverse_dns_response(&req).await)
}
pub async fn get_reverse_dns(ip: IpAddr) -> Option<String> {
    let ptr_lookup = DNS_RESOLVER.reverse_lookup(ip).await.ok()?;
    Some(ptr_lookup.iter().next()?.to_string())
}
pub async fn get_reverse_dns_response(req: &HttpRequest) -> SimpleResponse {
    let result = get_reverse_dns(get_ip(&req)).await.unwrap();
    SimpleResponse { value: result }
}