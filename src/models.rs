use serde::{Serialize, Deserialize};
use std::collections::HashMap;

pub trait ToPlainText {
    fn to_plain_text(&self) -> String;
}

#[derive(Serialize)]
pub struct Info {
    pub ip: String,
    pub reverse_dns: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,
    pub region: Option<String>,
}

impl ToPlainText for Info {
    fn to_plain_text(&self) -> String {
        format!(
            "IP: {}\nCountry: {}\nCity: {}\nRegion: {}",
            self.ip,
            self.country.clone().unwrap_or_else(|| "unknown".to_string()),
            self.city.clone().unwrap_or_else(|| "unknown".to_string()),
            self.region.clone().unwrap_or_else(|| "unknown".to_string())
        )
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
pub struct AsnResponse {
    pub autonomous_system_number: Option<u32>,
    pub autonomous_system_organization: Option<String>,
}

impl ToPlainText for AsnResponse {
    fn to_plain_text(&self) -> String {
        format!(
            "ASN: {}\nOrganization: {}",
            self.autonomous_system_number
                .map(|n| n.to_string())
                .unwrap_or_else(|| "unknown".to_string()),
            self.autonomous_system_organization
                .clone()
                .unwrap_or_else(|| "unknown".to_string())
        )
    }
}

#[derive(Debug, Deserialize)]
pub struct CityRecord {
    pub country: Option<CountryRecord>,
    pub city: Option<NameRecord>,
    pub subdivisions: Option<Vec<SubdivisionRecord>>,
}

#[derive(Debug, Deserialize)]
pub struct CountryRecord {
    pub names: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct NameRecord {
    pub names: Option<HashMap<String, String>>,
}

#[derive(Debug, Deserialize)]
pub struct SubdivisionRecord {
    pub names: Option<HashMap<String, String>>,
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

#[derive(Deserialize)]
pub struct QueryOptions {
    pub format: Option<String>,
}
