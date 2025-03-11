use crate::config::DnsResolver;
use crate::format_middleware::Format;
use crate::models::SimpleResponse;
use crate::util::{format_response, get_ip};
use crate::AppState;
use actix_web::{web, HttpMessage, HttpRequest, HttpResponse};
use std::net::IpAddr;
use std::sync::Arc;

pub async fn reverse_dns_handler(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    format_response(
        req.extensions().get::<Format>().unwrap(),
        &get_reverse_dns_response(&req, &state).await,
        false,
    )
}

pub async fn get_reverse_dns(ip: IpAddr, resolver: &DnsResolver) -> Option<String> {
    resolver.reverse_lookup(ip).await
}

pub async fn get_reverse_dns_response(req: &HttpRequest, state: &web::Data<Arc<AppState>>) -> SimpleResponse {
    match get_reverse_dns(get_ip(&req), &state.dns_resolver).await {
        Some(result) => SimpleResponse { value: result },
        None => SimpleResponse {
            value: "".to_string(),
        },
    }
}
