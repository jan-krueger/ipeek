use std::collections::HashMap;
use serde::{Serialize, Deserialize};
use crate::models::BlacklistReason::Unknown;

pub trait ToPlainText {
    fn to_plain_text(&self) -> String;
}

pub trait ToCsv<T> {
    fn to_csv_entries(&self) -> Vec<T>;
}

#[derive(Serialize)]
pub struct Info {
    pub ip: String,
    pub reverse_dns: String,
    pub country: String,
    pub country_code: String,
    pub city: String,
    pub region: String,
}

impl ToPlainText for Info {
    fn to_plain_text(&self) -> String {
        format!(
            "IP: {}\nHostname: {}\nCountry: {} ({})\nRegion: {}\nCity: {}",
            self.ip,
            self.reverse_dns,
            self.country,
            self.country_code,
            self.region,
            self.city,
        )
    }
}

#[derive(Serialize)]
pub struct CsvInfoEntry {
    pub ip: String,
    pub reverse_dns: String,
    pub country: String,
    pub country_code: String,
    pub city: String,
    pub region: String,
}

impl ToCsv<CsvInfoEntry> for Info {
    fn to_csv_entries(&self) -> Vec<CsvInfoEntry> {
        vec![CsvInfoEntry {
            ip: self.ip.clone(),
            reverse_dns: self.reverse_dns.clone(),
            country: self.country.clone(),
            country_code: self.country_code.clone(),
            city: self.city.clone(),
            region: self.region.clone(),
        }]
    }
}

#[derive(Debug, Deserialize, Serialize)]
pub struct AsnRecord {
    #[serde(rename = "autonomous_system_number")]
    pub autonomous_system_number: Option<u32>,
    #[serde(rename = "autonomous_system_organization")]
    pub autonomous_system_organization: Option<String>,
}

#[derive(Serialize)]
pub struct CsvAsnEntry {
    pub autonomous_system_number: Option<u32>,
    pub autonomous_system_organization: Option<String>,
}

impl ToCsv<CsvAsnEntry> for AsnRecord {
    fn to_csv_entries(&self) -> Vec<CsvAsnEntry> {
        vec![CsvAsnEntry {
            autonomous_system_number: self.autonomous_system_number,
            autonomous_system_organization: self.autonomous_system_organization.clone(),
        }]
    }
}

#[derive(Serialize)]
pub struct AsnResponse {
    pub autonomous_system_number: Option<u32>,
    pub autonomous_system_organization: Option<String>,
}

impl ToPlainText for AsnResponse {
    fn to_plain_text(&self) -> String {
        format!(
            "ASN: {}\nOrganization: {}",
            self.autonomous_system_number.map(|n| n.to_string()).unwrap_or_default(),
            self.autonomous_system_organization.clone().unwrap_or_default()
        )
    }
}

impl ToCsv<CsvAsnEntry> for AsnResponse {
    fn to_csv_entries(&self) -> Vec<CsvAsnEntry> {
        vec![CsvAsnEntry {
            autonomous_system_number: self.autonomous_system_number,
            autonomous_system_organization: self.autonomous_system_organization.clone(),
        }]
    }
}

#[derive(Serialize)]
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

#[derive(Serialize, Debug, Clone, PartialEq)]
pub enum BlacklistReason {
    SpamSource,
    SpamSupport,
    ExploitedOrMalicious,
    DynamicResidential,
    Unknown,
}

#[derive(Serialize)]
pub struct BlacklistResponse {
    pub ip: String,
    pub blacklisted: bool,
    pub listed_in: HashMap<String, BlacklistReason>,
}

impl ToPlainText for BlacklistResponse {
    fn to_plain_text(&self) -> String {
        let mut result = format!(
            "IP: {}\nBlacklisted: {}",
            self.ip,
            if self.blacklisted { "yes" } else { "no" }
        );

        if self.blacklisted && !self.listed_in.is_empty() {
            result.push_str("\nLists:");
            for (dnsbl, reason) in &self.listed_in {
                if reason == &Unknown {
                    continue;
                }
                result.push_str(&format!("\n - {} ({:?})", dnsbl, reason));
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

impl ToCsv<CsvBlacklistEntry> for BlacklistResponse {
    fn to_csv_entries(&self) -> Vec<CsvBlacklistEntry> {
        self.listed_in.iter().filter_map(|(dnsbl, reason)| {
            if reason == &Unknown {
                None
            } else {
                Some(CsvBlacklistEntry {
                    ip: self.ip.clone(),
                    dnsbl: dnsbl.clone(),
                    reason: reason.clone(),
                })
            }
        }).collect()
    }
}