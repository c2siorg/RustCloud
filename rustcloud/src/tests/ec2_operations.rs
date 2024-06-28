use crate::aws::aws_apis::compute::aws_ec2::*;

use aws_sdk_ec2::{Client, Config};
use aws_sdk_ec2::config::Region;
// use aws_sdk_ec2::config::Region;


#[tokio::test]
async fn test_create_instance() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let ami_id = "ami-1234567890abcdef0"; // Replace with a valid AMI ID

    let result = create_instance(&client, ami_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_show_state() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    // Optionally provide instance IDs to test specific instances
    let ids = None;

    let result = show_state(&client, ids).await;
    assert!(result.is_ok());
}


#[tokio::test]
async fn test_show_all_events() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let result = show_all_events(&client).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_enable_monitoring() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let instance_id = "i-1234567890abcdef0"; // Replace with a valid instance ID

    let result = enable_monitoring(&client, instance_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_reboot_instance() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let instance_id = "i-1234567890abcdef0"; // Replace with a valid instance ID

    let result = reboot_instance(&client, instance_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_start_instance() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let instance_id = "i-1234567890abcdef0";

    let result = start_instance(&client, instance_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_stop_instance() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let instance_id = "i-1234567890abcdef0"; // Replace with a valid instance ID

    let result = stop_instance(&client, instance_id).await;
    assert!(result.is_ok());
}

