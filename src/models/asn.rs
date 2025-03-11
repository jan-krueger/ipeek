use serde::{Deserialize, Serialize};
use yaserde_derive::YaSerialize;
use crate::models::traits::{ToCsv, ToPlainText};

#[derive(Debug, Deserialize, Serialize, Clone, YaSerialize)]
pub struct AsnRecord {
    #[serde(rename(deserialize = "autonomous_system_organization"))]
    pub aso: Option<String>,

    #[serde(rename(deserialize = "autonomous_system_number"))]
    pub asn: Option<u32>,
}

impl ToPlainText for AsnRecord {
    fn to_plain_text(&self) -> String {
        format!(
            "ASN: {}\nOrganization: {}",
            self.asn.clone().unwrap_or_else(|| 0),
            self.aso.clone().unwrap_or("".to_string())
        )
    }
}

impl ToCsv<AsnRecord> for AsnRecord {
    fn to_csv_entries(&self) -> Vec<AsnRecord> {
        vec![self.clone()]
    }
} 