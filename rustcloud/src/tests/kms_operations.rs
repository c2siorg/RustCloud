use crate::aws::aws_apis::security::aws_keymanagement::*;
use aws_sdk_kms::{Client, Config};
use aws_sdk_kms::config::Region;
use aws_sdk_kms::types::{KeySpec, KeyUsageType, OriginType, Tag};

#[tokio::test]
async fn test_create_key() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let policy = r#"{"Version": "2012-10-17","Statement": [{"Sid": "Enable IAM User Permissions","Effect": "Allow","Principal": {"AWS": "arn:aws:iam::111122223333:root"},"Action": "kms:*","Resource": "*"}]}"#.to_string();
    let description = Some("Test Key".to_string());
    let key_usage = Some(KeyUsageType::EncryptDecrypt);
    let key_spec = Some(KeySpec::SymmetricDefault);
    let origin = Some(OriginType::AwsKms);

    let result = create_key(&client, policy, description, key_usage, key_spec, origin, None, None, None, None, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_key() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let custom_key_store_id = "cks-1234567890abcdef0".to_string();

    let result = delete_key(&client, custom_key_store_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_describe_key() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let key_id = "1234abcd-12ab-34cd-56ef-1234567890ab".to_string();
    let grant_tokens = None;

    let result = describe_key(&client, key_id, grant_tokens).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_put_key_policy() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let key_id = "1234abcd-12ab-34cd-56ef-1234567890ab".to_string();
    let policy_name = "default".to_string();
    let policy = r#"{"Version": "2012-10-17","Statement": [{"Sid": "Enable IAM User Permissions","Effect": "Allow","Principal": {"AWS": "arn:aws:iam::111122223333:root"},"Action": "kms:*","Resource": "*"}]}"#.to_string();
    let bypass_policy_lockout_safety_check = None;

    let result = put_key_policy(&client, key_id, policy_name, policy, bypass_policy_lockout_safety_check).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let key_id = "1234abcd-12ab-34cd-56ef-1234567890ab".to_string();
    let description = Some("Updated Test Key".to_string());

    let result = update(&client, key_id, description).await;
    assert!(result.is_ok());
}