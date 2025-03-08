use std::net::{IpAddr, Ipv4Addr, SocketAddr};
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

const LOCAL_DNS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 53);
const FALLBACK_DNS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(1, 1, 1, 1)), 53);

// ✅ Build the name servers list **once** (instead of inside the function)
lazy_static::lazy_static! {
    static ref DNS_RESOLVER: TokioAsyncResolver = {
        let name_servers = vec![
            NameServerConfig {
                socket_addr: LOCAL_DNS,
                protocol: Protocol::Udp,
                tls_dns_name: None,
                trust_negative_responses: false,
                bind_addr: None,
            },
            NameServerConfig {
                socket_addr: FALLBACK_DNS,
                protocol: Protocol::Udp,
                tls_dns_name: None,
                trust_negative_responses: false,
                bind_addr: None,
            },
        ];

        TokioAsyncResolver::tokio(
            ResolverConfig::from_parts(None, vec![], name_servers),
            ResolverOpts::default(),
        )
    };
}

pub async fn get_reverse_dns(ip: IpAddr) -> Option<String> {
    let ptr_lookup = DNS_RESOLVER.reverse_lookup(ip).await.ok()?;
    Some(ptr_lookup.iter().next()?.to_string())
}