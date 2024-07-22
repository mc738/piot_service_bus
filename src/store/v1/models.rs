use std::ffi::CString;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct NewTopic {
    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct TopicDetails {
    pub id: String,
    pub name: String,
    pub created_on: DateTime<Utc>,
    pub serial: i32,
    pub metadata: Vec<MetadataItem>,
    pub labels: Vec<String>,
    pub tags: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct MetadataItem {
    pub key: String,
    pub value: String,
}
