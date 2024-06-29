use serde::{Deserialize, Serialize};

// Structs for request payloads

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateTopicRequest {
    // Define fields as per GCP Pub/Sub API
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct DeleteTopicRequest {
    // Define fields as per GCP Pub/Sub API
    pub topic: String,
    pub name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct GetTopicAttributesRequest {
    // Define fields as per GCP Pub/Sub API
    pub topic: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ListSubscriptionsRequest {
    // Define fields as per GCP Pub/Sub API
    pub topic: String,
    pub name: String
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CreateSubscriptionRequest {
    // Define fields as per GCP Pub/Sub API
    pub name: String,
    pub topic: String,
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
