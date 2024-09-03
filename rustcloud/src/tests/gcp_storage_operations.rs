use crate::gcp::gcp_apis::storage::gcp_storage::*;
use serde_json::json;
use std::collections::HashMap;
use tokio::test;


async fn create_storage_client() -> GoogleStorage {
    GoogleStorage::new()
}

#[tokio::test]
async fn test_create_disk() {
    let client = create_storage_client().await;

    let mut params = HashMap::new();
    params.insert("projectid".to_string(), json!("rare-daylight-403814"));
    params.insert("Name".to_string(), json!("test-disk-1"));
    params.insert("Zone".to_string(), json!("us-central1-a"));
    params.insert("Type".to_string(), json!("pd-standard"));
    params.insert("SizeGb".to_string(), json!(10));

    let result = client.create_disk(params).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"].as_u64().unwrap(), 200);
}

#[tokio::test]
async fn test_delete_disk() {
    let client = create_storage_client().await;

    let mut params = HashMap::new();
    params.insert("projectid".to_string(), "rare-daylight-403814".to_string());
    params.insert("Zone".to_string(), "us-central1-a".to_string());
    params.insert("disk".to_string(), "test-disk-1".to_string());

    let result = client.delete_disk(params).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"].as_u64().unwrap(), 200);
}

#[tokio::test]
async fn test_create_snapshot() {
    let client = create_storage_client().await;

    let mut params = HashMap::new();
    params.insert("projectid".to_string(), json!("rare-daylight-403814"));
    params.insert("Name".to_string(), json!("test-snapshot"));
    params.insert("Zone".to_string(), json!("us-central1-a"));
    params.insert("disk".to_string(), json!("test-disk-1"));

    let result = client.create_snapshot(params).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"].as_u64().unwrap(), 200);
}
