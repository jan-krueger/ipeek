use std::net::{IpAddr, SocketAddr};
use actix_web::{web, HttpRequest, HttpResponse};
use crate::models::{SimpleResponse, QueryOptions};
use crate::util::{format_response, get_ip};
use trust_dns_resolver::{
    config::{ResolverConfig, ResolverOpts},
    TokioAsyncResolver,
};
use trust_dns_resolver::config::{NameServerConfig, Protocol};

pub async fn reverse_dns_handler(
    req: HttpRequest,
    query: web::Query<QueryOptions>,
) -> HttpResponse {
    let result = get_reverse_dns(get_ip(&req)).await.unwrap();
    let response = SimpleResponse { value: result };
    format_response(query.format.as_deref(), &response)
}

pub async fn get_reverse_dns(ip: IpAddr) -> Option<String> {
    let name_server = NameServerConfig {
        socket_addr: SocketAddr::new("127.0.0.1".parse().unwrap(), 53),
        protocol: Protocol::Udp,
        tls_dns_name: None,
        trust_negative_responses: false,
        bind_addr: None,
    };

    let resolver = TokioAsyncResolver::tokio(
        ResolverConfig::from_parts(None, vec![], vec![name_server]),
        ResolverOpts::default(),
    );

    let ptr_lookup = resolver.reverse_lookup(ip).await.ok()?;

    Some(ptr_lookup.iter().next()?.to_string())
}