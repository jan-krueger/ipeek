use crate::config::DNS_RESOLVER;
use crate::format_middleware::Format;
use crate::models::SimpleResponse;
use crate::util::{format_response, get_ip};
use actix_web::{HttpMessage, HttpRequest, HttpResponse};
use std::net::IpAddr;

pub async fn reverse_dns_handler(req: HttpRequest) -> HttpResponse {
    format_response(
        req.extensions().get::<Format>().unwrap(),
        &get_reverse_dns_response(&req).await,
        false,
    )
}
pub async fn get_reverse_dns(ip: IpAddr) -> Option<String> {
    let ptr_lookup = DNS_RESOLVER.reverse_lookup(ip).await.ok()?;
    Some(ptr_lookup.iter().next()?.to_string())
}
pub async fn get_reverse_dns_response(req: &HttpRequest) -> SimpleResponse {
    match get_reverse_dns(get_ip(&req)).await {
        Some(result) => SimpleResponse { value: result },
        None => SimpleResponse {
            value: "".to_string(),
        },
    }
}
