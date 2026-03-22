use crate::aws::aws_apis::compute::aws_ec2::*;
use aws_sdk_ec2::Client;

fn test_ami_id() -> String {
    std::env::var("TEST_AMI_ID").unwrap_or_else(|_| "ami-0aff18ec83b712f05".to_string())
}

async fn create_client() -> Client {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    Client::new(&config)
}

async fn create_test_instance(client: &Client) -> String {
    create_instance(client, &test_ami_id())
        .await
        .expect("Failed to create test instance")
}

async fn cleanup_test_instance(client: &Client, instance_id: &str) {
    if let Err(e) = terminate_instance(client, instance_id).await {
        eprintln!("Warning: failed to terminate instance {instance_id}: {e:?}");
    }
}

async fn wait_for_instance_state(client: &Client, instance_id: &str, target_state: &str) {
    for _ in 0..30 {
        let resp = client
            .describe_instances()
            .instance_ids(instance_id)
            .send()
            .await
            .expect("describe_instances failed");

        let state = resp
            .reservations()
            .first()
            .and_then(|r| r.instances().first())
            .and_then(|i| i.state())
            .and_then(|s| s.name())
            .map(|n| n.as_str())
            .unwrap_or("");

        if state == target_state {
            return;
        }
        tokio::time::sleep(std::time::Duration::from_millis(500)).await;
    }
    panic!("Instance {instance_id} did not reach state '{target_state}' in time");
}

#[tokio::test]
async fn test_create_instance() {
    let client = create_client().await;
    let result = create_instance(&client, &test_ami_id()).await;
    if let Ok(ref id) = result {
        cleanup_test_instance(&client, id).await;
    }
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_show_state() {
    let client = create_client().await;
    let instance_id = create_test_instance(&client).await;
    let result = show_state(&client, Some(vec![instance_id.clone()])).await;
    cleanup_test_instance(&client, &instance_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_show_all_events() {
    let client = create_client().await;
    let result = show_all_events(&client).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_enable_monitoring() {
    let client = create_client().await;
    let instance_id = create_test_instance(&client).await;
    let result = enable_monitoring(&client, &instance_id).await;
    cleanup_test_instance(&client, &instance_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_reboot_instance() {
    let client = create_client().await;
    let instance_id = create_test_instance(&client).await;
    wait_for_instance_state(&client, &instance_id, "running").await;
    let result = reboot_instance(&client, &instance_id).await;
    cleanup_test_instance(&client, &instance_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_instance() {
    let client = create_client().await;
    let instance_id = create_test_instance(&client).await;
    wait_for_instance_state(&client, &instance_id, "running").await;
    stop_instance(&client, &instance_id).await.expect("stop_instance failed");
    wait_for_instance_state(&client, &instance_id, "stopped").await;
    let result = start_instance(&client, &instance_id).await;
    cleanup_test_instance(&client, &instance_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stop_instance() {
    let client = create_client().await;
    let instance_id = create_test_instance(&client).await;
    wait_for_instance_state(&client, &instance_id, "running").await;
    let result = stop_instance(&client, &instance_id).await;
    cleanup_test_instance(&client, &instance_id).await;
    assert!(result.is_ok());
}
