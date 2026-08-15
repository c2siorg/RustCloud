use aws_sdk_ec2::config::{BehaviorVersion, Credentials, Region, SharedCredentialsProvider};
use aws_sdk_ec2::types::{VolumeAttributeName, VolumeType};
use aws_sdk_ec2::{Client, Config};
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;

use crate::aws::aws_apis::storage::aws_block_storage::*;

fn mock_client(response_body: &str) -> Client {
    let request = http::Request::builder()
        .method("POST")
        .uri("https://ec2.us-east-1.amazonaws.com/")
        .body(SdkBody::empty())
        .unwrap();
    let response = http::Response::builder()
        .status(200)
        .header("content-type", "text/xml")
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

const REQUEST_ID: &str = "<requestId>e1a2b3c4-d5e6-7f89-0123-456789abcdef</requestId>";

#[tokio::test]
async fn test_create_volume() {
    let body = format!(
        r#"<?xml version="1.0"?><CreateVolumeResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">
            {REQUEST_ID}
            <volumeId>vol-1234567890abcdef0</volumeId>
            <size>8</size>
            <availabilityZone>us-west-2a</availabilityZone>
            <status>creating</status>
            <createTime>2026-01-01T00:00:00.000Z</createTime>
            <volumeType>gp2</volumeType>
        </CreateVolumeResponse>"#
    );
    let client = mock_client(&body);

    let result = create(
        &client,
        "us-west-2a".to_string(),
        Some(8),
        Some(VolumeType::Gp2),
        None,
        Some(false),
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_volume() {
    let body = format!(
        r#"<?xml version="1.0"?><DeleteVolumeResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">{REQUEST_ID}</DeleteVolumeResponse>"#
    );
    let client = mock_client(&body);

    let result = delete(&client, "vol-1234567890abcdef0".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_describe_volume() {
    let body = format!(
        r#"<?xml version="1.0"?><DescribeVolumeAttributeResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">
            {REQUEST_ID}
            <volumeId>vol-1234567890abcdef0</volumeId>
            <autoEnableIO><value>false</value></autoEnableIO>
        </DescribeVolumeAttributeResponse>"#
    );
    let client = mock_client(&body);

    let result = describe(
        &client,
        "vol-1234567890abcdef0".to_string(),
        VolumeAttributeName::AutoEnableIo,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_list_volumes() {
    let body = format!(
        r#"<?xml version="1.0"?><DescribeVolumesResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">
            {REQUEST_ID}
            <volumeSet/>
        </DescribeVolumesResponse>"#
    );
    let client = mock_client(&body);

    let result = list(&client, None, None, Some(10), None).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_list_volumes_live() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let result = list(&client, None, None, Some(10), None).await;

    assert!(result.is_ok(), "{:?}", result);
}
