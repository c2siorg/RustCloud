use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum State {
    StateUnspecified,
    Active,
    IngestionResourceError,
    ResourceError,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageStoragePolicy {
    pub allowed_persistence_regions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SchemaSettings {
    pub encoding: Option<String>,
    pub first_revision_id: Option<String>,
    pub last_revision_id: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct IngestionDataSourceSettings {
    pub aws_kinesis: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTopicRequest {
    pub name: String,
    pub labels: Option<HashMap<String, String>>,
    pub message_storage_policy: Option<MessageStoragePolicy>,
    pub kms_key_name: Option<String>,
    pub schema_settings: Option<SchemaSettings>,
    pub satisfies_pzs: Option<bool>,
    pub message_retention_duration: Option<String>,
    pub ingestion_data_source_settings: Option<IngestionDataSourceSettings>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTopicRequest {
    // Define fields as per GCP Pub/Sub API
    pub topic: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTopicAttributesRequest {
    // Define fields as per GCP Pub/Sub API
    pub topic: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListSubscriptionsRequest {
    pub project: String,
    pub page_size: Option<i128>,
    pub page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubscriptionRequest {
    pub name: String,
    pub topic: String,
    pub push_config: Option<HashMap<String, String>>,
    pub bigquery_config: Option<HashMap<String, String>>,
    pub cloud_storage_config: Option<HashMap<String, String>>,
    pub ack_deadline_seconds: Option<i64>,
    pub retain_acked_messages: Option<bool>,
    pub message_retention_duration: Option<String>,
    pub labels: Option<HashMap<String, String>>,
    pub enable_message_ordering: Option<bool>,
    pub expiration_policy: Option<HashMap<String, String>>,
    pub dead_letter_policy: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishRequest {
    // Define fields as per GCP Pub/Sub API
    pub topic: String,
    pub messages: Vec<Message>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ListTopicsRequest {
    pub project: String,
    pub page_size: Option<i128>,
    pub page_token: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteSubscriptionRequest {
    // Define fields as per GCP Pub/Sub API
    pub subscription: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Message {
    // Define fields as per GCP Pub/Sub API
    pub data: String,
}
