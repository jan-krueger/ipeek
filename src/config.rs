use serde::Deserialize;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};
use std::time::Duration;
use trust_dns_resolver::config::{NameServerConfig, Protocol, ResolverConfig, ResolverOpts};
use trust_dns_resolver::TokioAsyncResolver;

#[derive(Debug, Deserialize)]
pub struct DnsConfig {
    #[serde(default = "default_reverse_dns_timeout")]
    pub reverse_dns_timeout_ms: u64,
    #[serde(default = "default_blocklist_timeout")]
    pub blocklist_timeout_ms: u64,
}

fn default_reverse_dns_timeout() -> u64 {
    1000 // 1 second default timeout
}

fn default_blocklist_timeout() -> u64 {
    2000 // 2 seconds default timeout
}

#[derive(Debug, Deserialize)]
pub struct AppConfig {
    pub server_address: String,
    pub geo_db_path: String,
    pub asn_db_path: String,
    pub dns: DnsConfig,
}

const LOCAL_DNS: SocketAddr = SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), 53);

#[derive(Clone)]
pub struct DnsResolver {
    resolver: TokioAsyncResolver,
    reverse_dns_timeout: Duration,
    blocklist_timeout: Duration,
}

impl DnsResolver {
    pub fn new(config: &DnsConfig) -> Self {
        let name_servers = vec![
            NameServerConfig {
                socket_addr: LOCAL_DNS,
                protocol: Protocol::Udp,
                tls_dns_name: None,
                trust_negative_responses: false,
                bind_addr: None,
            }
        ];

        let resolver = TokioAsyncResolver::tokio(
            ResolverConfig::from_parts(None, vec![], name_servers),
            ResolverOpts::default(),
        );

        Self {
            resolver,
            reverse_dns_timeout: Duration::from_millis(config.reverse_dns_timeout_ms),
            blocklist_timeout: Duration::from_millis(config.blocklist_timeout_ms),
        }
    }

    pub async fn reverse_lookup(&self, ip: IpAddr) -> Option<String> {
        tokio::time::timeout(self.reverse_dns_timeout, self.resolver.reverse_lookup(ip))
            .await
            .ok()
            .and_then(|result| result.ok())
            .and_then(|ptr_lookup| ptr_lookup.iter().next().map(|name| name.to_string()))
    }

    pub async fn lookup_blocklist(&self, query: String) -> Option<IpAddr> {
        tokio::time::timeout(self.blocklist_timeout, self.resolver.lookup_ip(query))
            .await
            .ok()
            .and_then(|result| result.ok())
            .and_then(|response| response.iter().next())
    }
}

pub fn load_config<P: AsRef<std::path::Path>>(path: P) -> Result<AppConfig, config::ConfigError> {
    let settings = config::Config::builder()
        .add_source(config::File::from(path.as_ref().to_path_buf()))
        .build()?;
    settings.try_deserialize()
}
