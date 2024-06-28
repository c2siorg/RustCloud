use crate::aws::aws_apis::security::aws_iam::*;
use aws_sdk_iam::{Client, Config};
use aws_sdk_iam::config::Region;

#[tokio::test]
async fn test_attach_group_policy() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let group_name = "TestGroup".to_string();
    let policy_arn = "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess".to_string();

    let result = attach_group_policy(&client, group_name, policy_arn).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_group() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let path = "/".to_string();
    let group_name = "TestGroup".to_string();

    let result = create_group(&client, path, group_name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_group() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let group_name = "TestGroup".to_string();

    let result = delete_group(&client, group_name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_detach_group_policy() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let group_name = "TestGroup".to_string();
    let policy_arn = "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess".to_string();

    let result = detach_group_policy(&client, group_name, policy_arn).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_describe() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let group_name = "TestGroup".to_string();
    let marker = "".to_string();
    let max_items = 100;

    let result = describe(&client, group_name, marker, max_items).await;
    assert!(result.is_ok());
}