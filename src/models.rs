use crate::models::BlocklistReason::Unknown;
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
    pub blocklist: BlocklistRecord,
}

impl ToPlainText for AllResponse {
    fn to_plain_text(&self) -> String {
        format!(
            "IP: {}\nHostname: {}\nCountry: {} ({})\nRegion: {}\nCity: {}\nASN: {}\nBlocklist: {}",
            self.ip,
            self.reverse_dns,
            self.country,
            self.country_code,
            self.region,
            self.city,
            self.asn.to_plain_text(),
            self.blocklist.to_plain_text(),
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
    pub blocked: bool,
    pub blocklist_listed_in: String,
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
            blocked: self.blocklist.blocked,
            blocklist_listed_in: self
                .blocklist
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
pub enum BlocklistReason {
    SpamSource,
    SpamSupport,
    ExploitedOrMalicious,
    DynamicResidential,
    Unknown,
}

#[derive(Serialize, YaSerialize)]
pub struct BlocklistRecord {
    pub ip: String,
    pub blocked: bool,
    pub listed_in: Vec<BlocklistEntry>,
}
#[derive(Serialize, YaSerialize, Clone)]
pub struct BlocklistEntry {
    pub dnsbl: String,
    pub reason: BlocklistReason,
}

impl ToPlainText for BlocklistRecord {
    fn to_plain_text(&self) -> String {
        let mut result = format!(
            "IP: {}\nBlocked: {}",
            self.ip,
            if self.blocked { "yes" } else { "no" }
        );

        if self.blocked && !self.listed_in.is_empty() {
            result.push_str("\nLists:");
            for entry in &self.listed_in {
                result.push_str(&format!("\n - {} ({:?})", entry.dnsbl, entry.reason));
            }
        }

        result
    }
}

#[derive(Serialize)]
pub struct BlocklistCsvEntry {
    pub ip: String,
    pub dnsbl: String,
    pub reason: BlocklistReason,
}

impl ToCsv<BlocklistCsvEntry> for BlocklistRecord {
    fn to_csv_entries(&self) -> Vec<BlocklistCsvEntry> {
        self.listed_in
            .iter()
            .filter_map(|entry| {
                if entry.reason == Unknown {
                    None
                } else {
                    Some(BlocklistCsvEntry {
                        ip: self.ip.clone(),
                        dnsbl: entry.dnsbl.clone(),
                        reason: entry.reason.clone(),
                    })
                }
            })
            .collect()
    }
}
