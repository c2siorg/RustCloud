use crate::aws::aws_apis::security::aws_iam::*;
use aws_sdk_iam::config::Region;
use aws_sdk_iam::{Client, Config};

async fn create_client() -> Client {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    return client;
}

#[tokio::test]
async fn test_attach_group_policy() {
    let client = create_client().await;

    let group_name = "TestGroup".to_string();
    let policy_arn = "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess".to_string();

    let result = attach_group_policy(&client, group_name, policy_arn).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_group() {
    let client = create_client().await;

    let path = "/".to_string();
    let group_name = "TestGroup".to_string();

    let result = create_group(&client, path, group_name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_group() {
    let client = create_client().await;

    let group_name = "TestGroup".to_string();

    let result = delete_group(&client, group_name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_detach_group_policy() {
    let client = create_client().await;

    let group_name = "TestGroup".to_string();
    let policy_arn = "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess".to_string();

    let result = detach_group_policy(&client, group_name, policy_arn).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_describe_group() {
    let client = create_client().await;

    let group_name = "TestGroup".to_string();
    let marker = None;
    let max_items = Some(100);

    let result = describe(&client, group_name, marker, max_items).await;
    assert!(result.is_ok());
}
