use serde::Serialize;
use yaserde_derive::YaSerialize;
use crate::models::traits::{ToCsv, ToPlainText};
use crate::models::{AsnRecord, BlocklistRecord};

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

impl ToCsv<CsvInfoEntry> for AllResponse {
    fn to_csv_entries(&self) -> Vec<CsvInfoEntry> {
        vec![CsvInfoEntry {
            ip: self.ip.clone(),
            reverse_dns: self.reverse_dns.clone(),
            country: self.country.clone(),
            country_code: self.country_code.clone(),
            region: self.region.clone(),
            city: self.city.clone(),
            aso: self.asn.aso.clone().unwrap_or("".to_string()),
            asn: self.asn.asn.unwrap_or(0),
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