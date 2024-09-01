use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Structs for request payloads

pub enum State {
    STATE_UNSPECIFIED,
    ACTIVE,
    INGESTION_RESOURCE_ERROR,
    RESOURCE_ERROR,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct MessageStoragePolicy {
    pub allowed_persistence_regions: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SchemaSettings {
    pub encoding: Option<String>,
    pub firstRevisionId: Option<String>,
    lastRevisionId: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct IngestionDataSourceSettings {
    pub awsKinesis: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTopicRequest {
    // Define fields as per GCP Pub/Sub API
    pub name: String,
    pub labels: Option<HashMap<String, String>>,
    pub message_storage_policy: Option<MessageStoragePolicy>,
    pub kmsKeyName: Option<String>,
    pub schemaSettings: Option<SchemaSettings>,
    pub satisfiesPzs: Option<bool>,
    pub messageRetentionDuration: Option<String>,
    pub ingestionDataSourceSettings: Option<IngestionDataSourceSettings>,
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
pub struct ListSubscriptionsRequest {
    // Define fields as per GCP Pub/Sub API
    pub project: String,
    pub pageSize: Option<i128>,
    pub pageToken: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    // Define fields as per GCP Pub/Sub API
    pub name: String,
    pub topic: String,
    pub pushConfig: Option<HashMap<String, String>>,
    pub bigqueryConfig: Option<HashMap<String, String>>,
    pub cloudStorageConfig: Option<HashMap<String, String>>,
    pub ackDeadlineSeconds: Option<i64>,
    pub retainAckedMessages: Option<bool>,
    pub messageRetentionDuration: Option<String>,
    pub labels: Option<HashMap<String, String>>,
    pub enableMessageOrdering: Option<bool>,
    pub expirationPolicy: Option<HashMap<String, String>>,
    pub deadLetterPolicy: Option<HashMap<String, String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct PublishRequest {
    // Define fields as per GCP Pub/Sub API
    pub topic: String,
    pub messages: Vec<Message>,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListTopicsRequest {
    // Define fields as per GCP Pub/Sub API
    pub project: String,
    pub pageSize: Option<i128>,
    pub pageToken: Option<String>,
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
