use serde::Serialize;
use yaserde_derive::YaSerialize;
use crate::models::traits::{ToCsv, ToPlainText};

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