use crate::config::DNS_RESOLVER;
use crate::format_middleware::Format;
use crate::models::BlacklistReason::Unknown;
use crate::models::{BlacklistEntry, BlacklistReason, BlacklistRecord};
use crate::util::{format_response, get_ip};
use actix_web::{HttpMessage, HttpRequest, HttpResponse};
use std::collections::HashMap;
use std::net::IpAddr;

const BLACKLISTS: &[&str] = &[
    "zen.spamhaus.org",
    "bl.spamcop.net",
    "b.barracudacentral.org",
];

pub async fn blacklist_handler(req: HttpRequest) -> HttpResponse {
    format_response(
        req.extensions().get::<Format>().unwrap(),
        &get_blacklist_response(&req).await,
        false,
    )
}

pub async fn get_blacklist_response(req: &HttpRequest) -> BlacklistRecord {
    let ip = get_ip(&req);
    get_blacklist(&ip).await
}

pub async fn get_blacklist(ip: &IpAddr) -> BlacklistRecord {
    let listed_info = check_blacklists(ip).await;

    BlacklistRecord {
        ip: ip.to_string(),
        blacklisted: !listed_info.is_empty(),
        listed_in: listed_info,
    }
}

pub async fn check_blacklists(ip: &IpAddr) -> Vec<BlacklistEntry> {
    let mut tasks = Vec::new();
    let mut listed_in: Vec<BlacklistEntry> = Vec::new();

    if let IpAddr::V4(addr) = ip {
        let reversed_ip = addr
            .octets()
            .iter()
            .rev()
            .map(|octet| octet.to_string())
            .collect::<Vec<_>>()
            .join(".");

        for &dnsbl in BLACKLISTS {
            let query = format!("{}.{}", reversed_ip, dnsbl);

            tasks.push(tokio::spawn(async move {
                (dnsbl, DNS_RESOLVER.lookup_ip(query).await.ok())
            }));
        }

        for task in tasks {
            if let Ok((dnsbl, Some(response))) = task.await {
                if let Some(addr) = response.iter().next() {
                    let reason = BlacklistReason::from(
                        dnsbl.to_string().as_str(),
                        addr.to_string().as_str(),
                    );

                    if reason == Unknown {
                        continue;
                    }

                    listed_in.push(BlacklistEntry {
                        dnsbl: "".to_string(),
                        reason,
                    });
                }
            }
        }
    }

    listed_in
}

impl BlacklistReason {
    pub fn from(provider: &str, response: &str) -> Self {
        let mappings: HashMap<&str, HashMap<&str, BlacklistReason>> = HashMap::from([
            (
                "zen.spamhaus.org",
                HashMap::from([
                    ("127.0.0.2", BlacklistReason::SpamSource),
                    ("127.0.0.3", BlacklistReason::SpamSupport),
                    ("127.0.0.4", BlacklistReason::ExploitedOrMalicious),
                    ("127.0.0.10", BlacklistReason::DynamicResidential),
                ]),
            ),
            (
                "bl.spamcop.net",
                HashMap::from([
                    ("127.0.0.2", BlacklistReason::SpamSource),
                    ("127.0.0.4", BlacklistReason::ExploitedOrMalicious),
                ]),
            ),
            (
                "b.barracudacentral.org",
                HashMap::from([
                    ("127.0.0.2", BlacklistReason::SpamSource),
                    ("127.0.0.10", BlacklistReason::DynamicResidential),
                ]),
            ),
        ]);

        mappings
            .get(provider)
            .and_then(|map| map.get(response))
            .cloned()
            .unwrap_or(BlacklistReason::Unknown)
    }
}
