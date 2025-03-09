use serde::{Serialize, Deserialize};

pub trait ToPlainText {
    fn to_plain_text(&self) -> String;
}

#[derive(Serialize)]
pub struct Info {
    pub ip: String,
    pub reverse_dns: String,
    pub country: String,
    pub city: String,
    pub region: String,
}

impl ToPlainText for Info {
    fn to_plain_text(&self) -> String {
        format!(
            "IP: {}\nHostname: {}\nCountry: {}\nRegion: {}\nCity: {}",
            self.ip,
            self.reverse_dns.clone(),
            self.country.clone(),
            self.region.clone(),
            self.city.clone(),
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
                .unwrap_or_default(),
            self.autonomous_system_organization
                .clone()
                .unwrap_or_default()
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
