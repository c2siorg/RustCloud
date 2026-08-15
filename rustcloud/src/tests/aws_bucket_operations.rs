use aws_sdk_s3::config::{BehaviorVersion, Credentials, Region, SharedCredentialsProvider};
use aws_sdk_s3::{
    primitives::ByteStream,
    types::{BucketCannedAcl, CreateBucketConfiguration, MetadataDirective, ObjectCannedAcl},
    Client, Config,
};
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;

use crate::aws::aws_apis::storage::aws_storage_bucket::*;

fn mock_client(responses: &[(u16, &str)]) -> Client {
    let events = responses
        .iter()
        .map(|(status, body)| {
            let request = http::Request::builder()
                .method("GET")
                .uri("https://s3.amazonaws.com/")
                .body(SdkBody::empty())
                .unwrap();
            let response = http::Response::builder()
                .status(*status)
                .header("content-type", "application/xml")
                .body(SdkBody::from(*body))
                .unwrap();
            ReplayEvent::new(request, response)
        })
        .collect();
    let http_client = StaticReplayClient::new(events);

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

async fn create_test_bucket(client: &Client, bucket: &str) {
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(aws_sdk_s3::types::BucketLocationConstraint::UsWest2)
        .build();
    create_bucket(
        client,
        BucketCannedAcl::PublicRead,
        bucket.to_string(),
        cfg,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await
    .expect("Failed to create test bucket");
}

#[tokio::test]
async fn test_create_bucket() {
    let client = mock_client(&[(200, ""), (204, "")]);
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(aws_sdk_s3::types::BucketLocationConstraint::UsWest2)
        .build();

    let result = create_bucket(
        &client,
        BucketCannedAcl::PublicRead,
        "test-create-bucket".to_string(),
        cfg,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
    delete(&client, "test-create-bucket".to_string(), None).await.ok();
}

#[tokio::test]
async fn test_delete_bucket() {
    let client = mock_client(&[(200, ""), (204, "")]);
    create_test_bucket(&client, "test-delete-bucket").await;

    let result = delete(&client, "test-delete-bucket".to_string(), None).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_object() {
    let client = mock_client(&[(200, ""), (204, ""), (204, "")]);
    create_test_bucket(&client, "test-delete-object-bucket").await;

    let result = delete_object(
        &client,
        "test-delete-object-bucket".to_string(),
        "test-object".to_string(),
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
    delete(&client, "test-delete-object-bucket".to_string(), None)
        .await
        .ok();
}

#[tokio::test]
async fn test_list_buckets() {
    let body = r#"<?xml version="1.0" encoding="UTF-8"?>
        <ListAllMyBucketsResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
            <Owner><ID>owner-id</ID><DisplayName>owner</DisplayName></Owner>
            <Buckets>
                <Bucket><Name>test-bucket</Name><CreationDate>2026-01-01T00:00:00.000Z</CreationDate></Bucket>
            </Buckets>
        </ListAllMyBucketsResult>"#;
    let client = mock_client(&[(200, body)]);

    let result = list(&client).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_put_object() {
    let client = mock_client(&[(200, "")]);
    let body = ByteStream::from_static(b"hello from rustcloud");

    let result = put_object(
        &client,
        "test-bucket".to_string(),
        "test-object".to_string(),
        body,
        Some("text/plain".to_string()),
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
async fn test_get_object() {
    let client = mock_client(&[(200, "hello from rustcloud")]);

    let result = get_object(
        &client,
        "test-bucket".to_string(),
        "test-object".to_string(),
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_list_objects_v2() {
    let body = r#"<?xml version="1.0" encoding="UTF-8"?>
        <ListBucketResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
            <Name>test-bucket</Name>
            <KeyCount>0</KeyCount>
            <MaxKeys>100</MaxKeys>
            <IsTruncated>false</IsTruncated>
        </ListBucketResult>"#;
    let client = mock_client(&[(200, body)]);

    let result =
        list_objects_v2(&client, "test-bucket".to_string(), None, None, Some(100), None, None, None, None)
            .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_head_object() {
    let client = mock_client(&[(200, "")]);

    let result = head_object(
        &client,
        "test-bucket".to_string(),
        "test-object".to_string(),
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
async fn test_copy_object() {
    let body = r#"<?xml version="1.0" encoding="UTF-8"?>
        <CopyObjectResult xmlns="http://s3.amazonaws.com/doc/2006-03-01/">
            <ETag>"9f2a1d8b7c6e5f4a3b2c1d0e9f8a7b6c"</ETag>
            <LastModified>2026-01-01T00:00:00.000Z</LastModified>
        </CopyObjectResult>"#;
    let client = mock_client(&[(200, body)]);

    let result = copy_object(
        &client,
        "test-bucket".to_string(),
        "test-object-copy".to_string(),
        "test-bucket/test-object".to_string(),
        Some(MetadataDirective::Copy),
        Some(ObjectCannedAcl::Private),
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_list_buckets_live() {
    let config = aws_config::load_from_env().await;
    let mut builder = aws_sdk_s3::config::Builder::from(&config);
    if std::env::var("AWS_ENDPOINT_URL").is_ok() {
        builder = builder.force_path_style(true);
    }
    let client = Client::from_conf(builder.build());

    let result = list(&client).await;

    assert!(result.is_ok(), "{:?}", result);
}
