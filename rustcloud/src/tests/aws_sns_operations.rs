use crate::aws::aws_apis::app_services::aws_sns::*;
use aws_sdk_sns::Client;

async fn create_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}

#[tokio::test]
async fn test_create_topic() {
    let client = create_client().await;
    let result = create_topic(&client, "test-topic").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_topic() {
    let client = create_client().await;
    let topic_arn = "arn:aws:sns:us-east-1:123456789012:test-topic";
    let result = delete_topic(&client, topic_arn).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_topics() {
    let client = create_client().await;
    let result = list_topics(&client).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_publish() {
    let client = create_client().await;
    let topic_arn = "arn:aws:sns:us-east-1:123456789012:test-topic";
    let result = publish(&client, topic_arn, "Hello from RustCloud", Some("Test subject".to_string())).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_subscribe() {
    let client = create_client().await;
    let topic_arn = "arn:aws:sns:us-east-1:123456789012:test-topic";
    let result = subscribe(&client, topic_arn, "email", "test@example.com").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_unsubscribe() {
    let client = create_client().await;
    let subscription_arn = "arn:aws:sns:us-east-1:123456789012:test-topic:abc123";
    let result = unsubscribe(&client, subscription_arn).await;
    assert!(result.is_ok());
}
