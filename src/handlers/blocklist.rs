use crate::config::DnsResolver;
use crate::format_middleware::Format;
use crate::models::BlocklistReason;
use crate::models::{BlocklistEntry, BlocklistRecord};
use crate::util::{format_response, get_ip};
use actix_web::{HttpMessage, HttpRequest, HttpResponse};
use std::collections::HashMap;
use std::net::IpAddr;
use std::sync::Arc;
use crate::AppState;
use actix_web::web;

const BLOCKLISTS: &[&str] = &[
    "zen.spamhaus.org",
    "bl.spamcop.net",
    "b.barracudacentral.org",
];

pub async fn blocklist_handler(req: HttpRequest, state: web::Data<Arc<AppState>>) -> HttpResponse {
    format_response(
        req.extensions().get::<Format>().unwrap(),
        &get_blocklist_response(&req, &state).await,
        false,
    )
}

pub async fn get_blocklist_response(req: &HttpRequest, state: &web::Data<Arc<AppState>>) -> BlocklistRecord {
    let ip = get_ip(&req);
    get_blocklist(&ip, &state.dns_resolver).await
}

pub async fn get_blocklist(ip: &IpAddr, resolver: &DnsResolver) -> BlocklistRecord {
    let listed_info = check_blocklists(ip, resolver).await;

    BlocklistRecord {
        ip: ip.to_string(),
        blocked: !listed_info.is_empty(),
        listed_in: listed_info,
    }
}

pub async fn check_blocklists(ip: &IpAddr, resolver: &DnsResolver) -> Vec<BlocklistEntry> {
    let mut tasks = Vec::new();
    let mut listed_in: Vec<BlocklistEntry> = Vec::new();

    if let IpAddr::V4(addr) = ip {
        let reversed_ip = addr
            .octets()
            .iter()
            .rev()
            .map(|octet| octet.to_string())
            .collect::<Vec<_>>()
            .join(".");

        for &dnsbl in BLOCKLISTS {
            let query = format!("{}.{}", reversed_ip, dnsbl);
            let resolver = resolver.clone();

            tasks.push(tokio::spawn(async move {
                (dnsbl, resolver.lookup_blocklist(query).await)
            }));
        }

        for task in tasks {
            if let Ok((dnsbl, Some(addr))) = task.await {
                let reason = BlocklistReason::from(
                    dnsbl.to_string().as_str(),
                    addr.to_string().as_str(),
                );

                if reason == BlocklistReason::Unknown {
                    continue;
                }

                listed_in.push(BlocklistEntry {
                    dnsbl: dnsbl.to_string(),
                    reason,
                });
            }
        }
    }

    listed_in
}

impl BlocklistReason {
    pub fn from(provider: &str, response: &str) -> Self {
        let mappings: HashMap<&str, HashMap<&str, BlocklistReason>> = HashMap::from([
            (
                "zen.spamhaus.org",
                HashMap::from([
                    ("127.0.0.2", BlocklistReason::SpamSource),
                    ("127.0.0.3", BlocklistReason::SpamSupport),
                    ("127.0.0.4", BlocklistReason::ExploitedOrMalicious),
                    ("127.0.0.10", BlocklistReason::DynamicResidential),
                ]),
            ),
            (
                "bl.spamcop.net",
                HashMap::from([
                    ("127.0.0.2", BlocklistReason::SpamSource),
                    ("127.0.0.4", BlocklistReason::ExploitedOrMalicious),
                ]),
            ),
            (
                "b.barracudacentral.org",
                HashMap::from([
                    ("127.0.0.2", BlocklistReason::SpamSource),
                    ("127.0.0.10", BlocklistReason::DynamicResidential),
                ]),
            ),
        ]);

        mappings
            .get(provider)
            .and_then(|map| map.get(response))
            .cloned()
            .unwrap_or(BlocklistReason::Unknown)
    }
}
