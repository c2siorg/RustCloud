use crate::aws::aws_apis::app_services::aws_sqs;
use aws_sdk_sqs::Client;

async fn get_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}

#[tokio::test]
async fn test_create_queue() {
    let client = get_client().await;
    let result = aws_sqs::create_queue(&client, "rustcloud-test-queue").await;
    assert!(result.is_ok());
    let url = result.unwrap();
    assert!(!url.is_empty());
    println!("Created queue: {}", url);
}

#[tokio::test]
async fn test_list_queues() {
    let client = get_client().await;
    let result = aws_sqs::list_queues(&client, Some("rustcloud".to_string())).await;
    assert!(result.is_ok());
    println!("Queues: {:?}", result.unwrap());
}

#[tokio::test]
async fn test_send_and_receive_message() {
    let client = get_client().await;

    let queue_url = aws_sqs::get_queue_url(&client, "rustcloud-test-queue")
        .await
        .expect("queue must exist — run test_create_queue first");

    let msg_id = aws_sqs::send_message(&client, &queue_url, "hello from rustcloud", None)
        .await
        .unwrap();
    assert!(!msg_id.is_empty());

    let messages =
        aws_sqs::receive_messages(&client, &queue_url, Some(1), Some(5)).await.unwrap();
    assert!(!messages.is_empty());
    println!("Body: {:?}", messages[0].body());
}

#[tokio::test]
async fn test_delete_message() {
    let client = get_client().await;

    let queue_url = aws_sqs::get_queue_url(&client, "rustcloud-test-queue")
        .await
        .expect("queue must exist");

    aws_sqs::send_message(&client, &queue_url, "message to delete", None)
        .await
        .unwrap();

    let messages =
        aws_sqs::receive_messages(&client, &queue_url, Some(1), Some(5)).await.unwrap();

    if let Some(msg) = messages.first() {
        if let Some(handle) = msg.receipt_handle() {
            let result = aws_sqs::delete_message(&client, &queue_url, handle).await;
            assert!(result.is_ok());
            println!("Deleted message successfully");
        }
    }
}

#[tokio::test]
async fn test_delete_queue() {
    let client = get_client().await;

    let queue_url = aws_sqs::get_queue_url(&client, "rustcloud-test-queue")
        .await
        .expect("queue must exist");

    let result = aws_sqs::delete_queue(&client, &queue_url).await;
    assert!(result.is_ok());
    println!("Queue deleted");
}
