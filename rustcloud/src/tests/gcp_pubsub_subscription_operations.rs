use crate::gcp::gcp_apis::app_services::gcp_pubsub_subscription::*;
use std::collections::HashMap;

fn create_client() -> GcpPubSubSubscription {
    GcpPubSubSubscription::new()
}

#[tokio::test]
async fn test_create_subscription() {
    let client = create_client();

    let mut request = HashMap::new();
    request.insert("Project".to_string(), "your_project_id".to_string());
    request.insert("Topic".to_string(), "your_topic_name".to_string());
    request.insert("Subscription".to_string(), "your_subscription".to_string());
    request.insert("AckDeadlineSeconds".to_string(), "20".to_string());

    let result = client.create_subscription(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_subscriptions() {
    let client = create_client();

    let mut request = HashMap::new();
    request.insert("Project".to_string(), "your_project_id".to_string());
    request.insert("PageSize".to_string(), "10".to_string());

    let result = client.list_subscriptions(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_pull_messages() {
    let client = create_client();

    let mut request = HashMap::new();
    request.insert("Project".to_string(), "your_project_id".to_string());
    request.insert("Subscription".to_string(), "your_subscription".to_string());
    request.insert("MaxMessages".to_string(), "5".to_string());

    let result = client.pull_messages(request).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_acknowledge_messages() {
    let client = create_client();
    let ack_ids = vec!["ack_id_1".to_string(), "ack_id_2".to_string()];

    let result = client
        .acknowledge_messages("your_project_id", "your_subscription", ack_ids)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_modify_push_config() {
    let client = create_client();

    let result = client
        .modify_push_config(
            "your_project_id",
            "your_subscription",
            Some("https://example.com/push"),
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_subscription() {
    let client = create_client();

    let mut request = HashMap::new();
    request.insert("Project".to_string(), "your_project_id".to_string());
    request.insert("Subscription".to_string(), "your_subscription".to_string());

    let result = client.delete_subscription(request).await;
    assert!(result.is_ok());
}
