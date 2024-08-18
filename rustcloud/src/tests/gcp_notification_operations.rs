use crate::gcp::gcp_apis::app_services::gcp_notification_service::*;
use std::collections::HashMap;
use tokio::test;

async fn create_client() -> Googlenotification {
    Googlenotification::new()
}

#[tokio::test]
async fn test_list_topic() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("Project".to_string(), "your_project_id".to_string());
    request.insert("PageSize".to_string(), "10".to_string());
    request.insert("PageToken".to_string(), "token".to_string());

    let result = client.list_topic(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_topic() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("Project".to_string(), "your_project_id".to_string());
    request.insert("Topic".to_string(), "your_topic_name".to_string());

    let result = client.get_topic(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_topic() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("Project".to_string(), "your_project_id".to_string());
    request.insert("Topic".to_string(), "your_topic_name".to_string());

    let result = client.delete_topic(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_topic() {
    let client = create_client().await;

    let mut request = HashMap::new();
    request.insert("Project".to_string(), "your_project_id".to_string());
    request.insert("Topic".to_string(), "your_topic_name".to_string());

    let result = client.create_topic(request).await;
    assert!(result.is_ok());
}
