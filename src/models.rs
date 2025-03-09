use crate::models::BlacklistReason::Unknown;
use serde::{Deserialize, Serialize};
use yaserde_derive::YaSerialize;

pub trait ToPlainText {
    fn to_plain_text(&self) -> String;
}

pub trait ToCsv<T> {
    fn to_csv_entries(&self) -> Vec<T>;
}

#[derive(Serialize, YaSerialize)]
pub struct AllResponse {
    pub ip: String,
    pub reverse_dns: String,
    pub country: String,
    pub country_code: String,
    pub region: String,
    pub city: String,
    pub asn: AsnRecord,
    pub blacklist: BlacklistRecord,
}

impl ToPlainText for AllResponse {
    fn to_plain_text(&self) -> String {
        format!(
            "IP: {}\nHostname: {}\nCountry: {} ({})\nRegion: {}\nCity: {}\nASN: {}\nBlacklist: {}",
            self.ip,
            self.reverse_dns,
            self.country,
            self.country_code,
            self.region,
            self.city,
            self.asn.to_plain_text(),
            self.blacklist.to_plain_text(),
        )
    }
}

#[derive(Serialize)]
pub struct CsvInfoEntry {
    pub ip: String,
    pub reverse_dns: String,
    pub country: String,
    pub country_code: String,
    pub region: String,
    pub city: String,
    pub asn: u32,
    pub aso: String,
    pub blacklist_ip: String,
    pub blacklisted: bool,
    pub blacklist_listed_in: String,
}

impl ToCsv<CsvInfoEntry> for AllResponse {
    fn to_csv_entries(&self) -> Vec<CsvInfoEntry> {
        vec![CsvInfoEntry {
            ip: self.ip.clone(),
            reverse_dns: self.reverse_dns.clone(),
            country: self.country.clone(),
            country_code: self.country_code.clone(),
            region: self.region.clone(),
            city: self.city.clone(),
            asn: self.asn.autonomous_system_organization.unwrap_or(0),
            aso: self
                .asn
                .autonomous_system_number
                .clone()
                .unwrap_or("".to_string())
                .clone(),
            blacklist_ip: self.blacklist.ip.clone(),
            blacklisted: self.blacklist.blacklisted,
            blacklist_listed_in: self
                .blacklist
                .listed_in
                .clone()
                .into_iter()
                .map(|entry| format!("{} ({:?})", entry.dnsbl, entry.reason))
                .collect::<Vec<String>>()
                .join(":"),
        }]
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, YaSerialize)]
pub struct AsnRecord {
    pub autonomous_system_organization: Option<u32>,
    pub autonomous_system_number: Option<String>,
}

impl ToPlainText for AsnRecord {
    fn to_plain_text(&self) -> String {
        format!(
            "ASN: {:?}\nOrganization: {}",
            self.autonomous_system_organization.unwrap_or(0),
            self.autonomous_system_number
                .clone()
                .unwrap_or_else(|| "".to_string())
        )
    }
}
impl ToCsv<AsnRecord> for AsnRecord {
    fn to_csv_entries(&self) -> Vec<AsnRecord> {
        vec![self.clone()]
    }
}

#[derive(Serialize, YaSerialize)]
pub struct SimpleResponse {
    pub value: String,
}

impl ToPlainText for SimpleResponse {
    fn to_plain_text(&self) -> String {
        self.value.clone()
    }
}

impl ToCsv<SimpleResponse> for SimpleResponse {
    fn to_csv_entries(&self) -> Vec<SimpleResponse> {
        vec![SimpleResponse {
            value: self.value.clone(),
        }]
    }
}

#[derive(Serialize, Debug, Clone, PartialEq, YaSerialize)]
pub enum BlacklistReason {
    SpamSource,
    SpamSupport,
    ExploitedOrMalicious,
    DynamicResidential,
    Unknown,
}

#[derive(Serialize, YaSerialize)]
pub struct BlacklistRecord {
    pub ip: String,
    pub blacklisted: bool,
    pub listed_in: Vec<BlacklistEntry>,
}
#[derive(Serialize, YaSerialize, Clone)]
pub struct BlacklistEntry {
    pub dnsbl: String,
    pub reason: BlacklistReason,
}

impl ToPlainText for BlacklistRecord {
    fn to_plain_text(&self) -> String {
        let mut result = format!(
            "IP: {}\nBlacklisted: {}",
            self.ip,
            if self.blacklisted { "yes" } else { "no" }
        );

        if self.blacklisted && !self.listed_in.is_empty() {
            result.push_str("\nLists:");
            for entry in &self.listed_in {
                result.push_str(&format!("\n - {} ({:?})", entry.dnsbl, entry.reason));
            }
        }

        result
    }
}

#[derive(Serialize)]
pub struct CsvBlacklistEntry {
    pub ip: String,
    pub dnsbl: String,
    pub reason: BlacklistReason,
}

impl ToCsv<CsvBlacklistEntry> for BlacklistRecord {
    fn to_csv_entries(&self) -> Vec<CsvBlacklistEntry> {
        self.listed_in
            .iter()
            .filter_map(|entry| {
                if entry.reason == Unknown {
                    None
                } else {
                    Some(CsvBlacklistEntry {
                        ip: self.ip.clone(),
                        dnsbl: entry.dnsbl.clone(),
                        reason: entry.reason.clone(),
                    })
                }
            })
            .collect()
    }
}
