use crate::azure::azure_apis::app_services::azure_service_bus::AzureServiceBus;

fn create_client() -> AzureServiceBus {
    AzureServiceBus::with_config("rustcloud-test", "test-token")
}

#[tokio::test]
async fn test_create_queue() {
    let client = create_client();
    let result = client.create_queue("rustcloud-test-queue").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_queues() {
    let client = create_client();
    let result = client.list_queues().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_send_message() {
    let client = create_client();
    let result = client
        .send_message("rustcloud-test-queue", r#"{"event": "test", "source": "rustcloud"}"#)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_peek_lock_message() {
    let client = create_client();
    let result = client.peek_lock_message("rustcloud-test-queue", 30).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_receive_message() {
    let client = create_client();
    let result = client.receive_message("rustcloud-test-queue").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_complete_message() {
    let client = create_client();
    let result = client
        .complete_message("rustcloud-test-queue", "1", "lock-token-abc123")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_queue() {
    let client = create_client();
    let result = client.delete_queue("rustcloud-test-queue").await;
    assert!(result.is_ok());
}
