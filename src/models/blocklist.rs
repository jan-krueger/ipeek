use serde::Serialize;
use yaserde_derive::YaSerialize;
use crate::models::traits::{ToCsv, ToPlainText};

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

#[derive(Serialize)]
pub struct BlocklistCsvEntry {
    pub ip: String,
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

impl ToCsv<BlocklistCsvEntry> for BlocklistRecord {
    fn to_csv_entries(&self) -> Vec<BlocklistCsvEntry> {
        self.listed_in
            .iter()
            .filter_map(|entry| {
                if entry.reason == BlocklistReason::Unknown {
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