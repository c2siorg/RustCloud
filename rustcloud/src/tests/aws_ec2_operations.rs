use crate::aws::aws_apis::compute::aws_ec2::*;
use aws_config::meta::region::RegionProviderChain;
use aws_config::BehaviorVersion;
use aws_sdk_ec2::config::Region;
use aws_sdk_ec2::{Client, Config};

async fn create_client() -> Client {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    return client;
}

#[tokio::test]
async fn test_create_instance() {
    let client = create_client().await;
    let ami_id = "ami-0aff18ec83b712f05";
    let result = create_instance(&client, ami_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_show_state() {
    let client = create_client().await;
    let ids = Some((vec!["i-0374a5ac799ffd4d2".to_string()]));
    let result = show_state(&client, ids).await;
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

    let instance_id = "i-0a06238e5fc156e3f";

    let result = enable_monitoring(&client, instance_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_reboot_instance() {
    let client = create_client().await;

    let instance_id = "i-0a06238e5fc156e3f"; // Replace with a valid instance ID

    let result = reboot_instance(&client, instance_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_instance() {
    let client = create_client().await;

    let instance_id = "i-0a06238e5fc156e3f";

    let result = start_instance(&client, instance_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stop_instance() {
    let client = create_client().await;

    let instance_id = "i-0271bcdf16391102c"; // Replace with a valid instance ID

    let result = stop_instance(&client, instance_id).await;
    assert!(result.is_ok());
}

// Add terminate instance
