use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use serde::Deserialize;
use trust_dns_resolver::config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts};
use trust_dns_resolver::TokioAsyncResolver;

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server_address: String,
    pub geo_db_path: String,
    pub asn_db_path: String,
}

pub fn load_config<P: AsRef<std::path::Path>>(path: P) -> Result<AppConfig, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::from(path.as_ref().to_path_buf()))
        .build()?;
    settings.try_deserialize()
}
const LOCAL_DNS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 53);

lazy_static::lazy_static! {
    pub static ref DNS_RESOLVER: TokioAsyncResolver = {
        let name_servers = vec![
            NameServerConfig {
                socket_addr: LOCAL_DNS,
                protocol: Protocol::Udp,
                tls_dns_name: None,
                trust_negative_responses: false,
                bind_addr: None,
            }
        ];

        TokioAsyncResolver::tokio(
            ResolverConfig::from_parts(None, vec![], name_servers),
            ResolverOpts::default(),
        )
    };
}
