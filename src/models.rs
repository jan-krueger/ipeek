use serde::{Serialize, Deserialize};

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
            "IP: {}\nHostname: {}\nCountry: {}\nRegion: {}\nCity: {}",
            self.ip,
            self.reverse_dns.clone().unwrap_or_else(|| "".to_string()),
            self.country.clone().unwrap_or_else(|| "".to_string()),
            self.region.clone().unwrap_or_else(|| "".to_string()),
            self.city.clone().unwrap_or_else(|| "".to_string()),
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
                .unwrap_or_else(|| "".to_string()),
            self.autonomous_system_organization
                .clone()
                .unwrap_or_else(|| "".to_string())
        )
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

#[derive(Deserialize)]
pub struct QueryOptions {
    pub format: Option<String>,
}
