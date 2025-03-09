use std::net::{IpAddr};
use actix_web::{HttpMessage, HttpRequest, HttpResponse};
use crate::models::{SimpleResponse};
use crate::util::{format_response, get_ip};
use crate::config::DNS_RESOLVER;

pub async fn reverse_dns_handler(
    req: HttpRequest,
) -> HttpResponse {
    format_response(req.extensions().get::<String>().unwrap(), &get_reverse_dns_response(&req).await)
}
pub async fn get_reverse_dns(ip: IpAddr) -> Option<String> {
    let ptr_lookup = DNS_RESOLVER.reverse_lookup(ip).await.ok()?;
    Some(ptr_lookup.iter().next()?.to_string())
}
pub async fn get_reverse_dns_response(req: &HttpRequest) -> SimpleResponse {
    match get_reverse_dns(get_ip(&req)).await {
        Some(result) => { SimpleResponse { value: result } },
        None => { SimpleResponse { value: "".to_string() } }
    }
}