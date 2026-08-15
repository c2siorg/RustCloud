use aws_sdk_glacier::config::{BehaviorVersion, Credentials, Region, SharedCredentialsProvider};
use aws_sdk_glacier::{Client, Config};
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;

use crate::aws::aws_apis::storage::aws_archival_storage::*;

fn mock_client(status: u16, response_body: &str) -> Client {
    let request = http::Request::builder()
        .method("PUT")
        .uri("https://glacier.us-east-1.amazonaws.com/167355850481/vaults/test-vault")
        .body(SdkBody::empty())
        .unwrap();
    let response = http::Response::builder()
        .status(status)
        .header("content-type", "application/json")
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
async fn test_create_vault() {
    let client = mock_client(201, "{}");

    let result = create_vault(&client, "test-vault".to_string(), "167355850481".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_archive() {
    let client = mock_client(204, "");

    let result = delete_archive(
        &client,
        "167355850481".to_string(),
        "test-vault".to_string(),
        "archive123".to_string(),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_vault() {
    let client = mock_client(204, "");

    let result = delete_vault(&client, "167355850481".to_string(), "test-vault".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_upload_vault() {
    let client = mock_client(201, "{}");

    let result = upload(
        &client,
        "167355850481".to_string(),
        "test-vault".to_string(),
        Some("Test archive".to_string()),
        Some("1048576".to_string()),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_list_vault() {
    let client = mock_client(200, r#"{"VaultList":[]}"#);

    let result = list(&client, "167355850481".to_string(), None, Some(10)).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_list_vault_live() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let result = list(&client, "167355850481".to_string(), None, Some(10)).await;

    assert!(result.is_ok(), "{:?}", result);
}
