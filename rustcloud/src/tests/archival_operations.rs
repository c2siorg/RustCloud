use crate::aws::aws_apis::storage::aws_archival_storage::*;
use aws_sdk_glacier::{Client, Config};
use aws_sdk_glacier::config::Region;

#[tokio::test]
async fn test_create_vault() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let vault_name = "test-vault";
    let account_id = "123456789012";

    let result = create_vault(&client, vault_name.to_string(), account_id.to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_archive() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let account_id = "123456789012";
    let vault_name = "test-vault";
    let archive_id = "archive123";

    let result = delete_archive(&client, account_id.to_string(), vault_name.to_string(), archive_id.to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_vault() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let account_id = "123456789012";
    let vault_name = "test-vault";

    let result = delete_vault(&client, account_id.to_string(), vault_name.to_string()).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_upload() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let account_id = "123456789012";
    let vault_name = "test-vault";
    let archive_description = Some("Test archive".to_string());
    let part_size = Some("1048576".to_string());

    let result = upload(&client, account_id.to_string(), vault_name.to_string(), archive_description, part_size).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let account_id = "123456789012";
    let marker = None;
    let limit = Some(10);

    let result = list(&client, account_id.to_string(), marker, limit).await;
    assert!(result.is_ok());
}