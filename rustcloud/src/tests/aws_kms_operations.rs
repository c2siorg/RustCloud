use aws_sdk_kms::config::{BehaviorVersion, Credentials, Region, SharedCredentialsProvider};
use aws_sdk_kms::types::{KeySpec, KeyUsageType, OriginType};
use aws_sdk_kms::{Client, Config};
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;

use crate::aws::aws_apis::security::aws_keymanagement::*;

fn mock_client(response_body: &str) -> Client {
    let request = http::Request::builder()
        .method("POST")
        .uri("https://kms.us-east-1.amazonaws.com/")
        .body(SdkBody::empty())
        .unwrap();
    let response = http::Response::builder()
        .status(200)
        .body(SdkBody::from(response_body))
        .unwrap();
    let http_client = StaticReplayClient::new(vec![ReplayEvent::new(request, response)]);

    let config = Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new("us-east-1"))
        .credentials_provider(SharedCredentialsProvider::new(Credentials::new(
            "fake-access-key",
            "fake-secret-key",
            None,
            None,
            "test",
        )))
        .http_client(http_client)
        .build();

    Client::from_conf(config)
}

#[tokio::test]
async fn test_create_key() {
    let client = mock_client(r#"{"KeyMetadata":{"KeyId":"test-key-id"}}"#);

    let result = create_key(
        &client,
        "policy".to_string(),
        None,
        Some(KeyUsageType::EncryptDecrypt),
        Some(KeySpec::SymmetricDefault),
        Some(OriginType::AwsKms),
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_key() {
    let client = mock_client("{}");

    let result = delete_key(&client, "cks-1234567890abcdef0".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_describe_key() {
    let client = mock_client(r#"{"KeyMetadata":{"KeyId":"test-key-id"}}"#);

    let result = describe_key(&client, "test-key-id".to_string(), None).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_put_key_policy() {
    let client = mock_client("{}");

    let result = put_key_policy(
        &client,
        "test-key-id".to_string(),
        "default".to_string(),
        "policy".to_string(),
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_update_key() {
    let client = mock_client("{}");

    let result = update(
        &client,
        "test-key-id".to_string(),
        Some("Updated Test Key".to_string()),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_describe_key_live() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let result = describe_key(&client, "test-key-id".to_string(), None).await;

    assert!(result.is_ok(), "{:?}", result);
}
