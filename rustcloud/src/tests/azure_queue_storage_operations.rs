use crate::azure::azure_apis::storage::azure_queue_storage::AzureQueueClient;

async fn create_client() -> AzureQueueClient {
    let account = std::env::var("AZURE_STORAGE_ACCOUNT").expect("AZURE_STORAGE_ACCOUNT not set");

    AzureQueueClient::new(account)
}

#[tokio::test]
async fn test_list_queues_storage() {
    let client = create_client().await;

    let result = client.list_queues().await;

    println!("{:?}", result);

    assert!(result.is_ok(), "List queues failed: {:?}", result);
}

#[tokio::test]
async fn test_create_queue_storage() {
    let client = create_client().await;

    let queue = "test-queue";

    let result = client.create_queue(queue).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Create queue failed: {:?}", result);
}

#[tokio::test]
async fn test_delete_queue_storage() {
    let client = create_client().await;

    let queue = "test-queue";

    let result = client.delete_queue(queue).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Delete queue failed: {:?}", result);
}

#[tokio::test]
async fn test_send_message() {
    let client = create_client().await;

    let queue = "test-queue";

    let message = "Hello from RustCloud Queue!";

    let result = client.send_message(queue, message).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Send message failed: {:?}", result);
}

#[tokio::test]
async fn test_receive_messages() {
    let client = create_client().await;

    let queue = "test-queue";

    let result = client.receive_messages(queue).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Receive messages failed: {:?}", result);
}
