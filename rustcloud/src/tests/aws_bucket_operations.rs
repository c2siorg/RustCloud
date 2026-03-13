use crate::aws::aws_apis::storage::aws_storage_bucket::*;
use aws_sdk_s3::{
    primitives::ByteStream,
    types::{
        BucketCannedAcl, CreateBucketConfiguration, MetadataDirective, ObjectCannedAcl,
        ObjectOwnership, RequestPayer,
    },
    Client, Error,
};

async fn create_client() -> Client {
    let config = aws_config::load_from_env().await;
    let mut builder = aws_sdk_s3::config::Builder::from(&config);
    if std::env::var("AWS_ENDPOINT_URL").is_ok() {
        builder = builder.force_path_style(true);
    }
    Client::from_conf(builder.build())
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
    let client = create_client().await;
    let bucket = "test-create-bucket".to_string();
    let cfg = CreateBucketConfiguration::builder()
        .location_constraint(aws_sdk_s3::types::BucketLocationConstraint::UsWest2)
        .build();

    let result = create_bucket(
        &client,
        BucketCannedAcl::PublicRead,
        bucket.clone(),
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

    assert!(result.is_ok());
    delete(&client, bucket, None).await.ok();
}

#[tokio::test]
async fn test_delete_bucket() {
    let client = create_client().await;
    let bucket = "test-delete-bucket";
    create_test_bucket(&client, bucket).await;

    let result = delete(&client, bucket.to_string(), None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_object() {
    let client = create_client().await;
    let bucket = "test-delete-object-bucket";
    create_test_bucket(&client, bucket).await;

    let result = delete_object(
        &client,
        bucket.to_string(),
        "test-object".to_string(),
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok());
    delete(&client, bucket.to_string(), None).await.ok();
}

#[tokio::test]
async fn test_list_buckets() {
    let client = create_client().await;
    let result = list(&client).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_put_object() {
    let client = create_client().await;
    let bucket = "test-bucket".to_string();
    let key = "test-object".to_string();
    let body = ByteStream::from_static(b"hello from rustcloud");

    let result = put_object(
        &client,
        bucket,
        key,
        body,
        Some("text/plain".to_string()),
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_object() {
    let client = create_client().await;
    let bucket = "test-bucket".to_string();
    let key = "test-object".to_string();

    let result = get_object(&client, bucket, key, None, None, None).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_objects_v2() {
    let client = create_client().await;
    let bucket = "test-bucket".to_string();

    let result = list_objects_v2(
        &client,
        bucket,
        None,
        None,
        Some(100),
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_head_object() {
    let client = create_client().await;
    let bucket = "test-bucket".to_string();
    let key = "test-object".to_string();

    let result = head_object(&client, bucket, key, None, None, None, None, None).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_copy_object() {
    let client = create_client().await;
    let bucket = "test-bucket".to_string();
    let destination_key = "test-object-copy".to_string();
    let copy_source = "test-bucket/test-object".to_string();

    let result = copy_object(
        &client,
        bucket,
        destination_key,
        copy_source,
        Some(MetadataDirective::Copy),
        Some(ObjectCannedAcl::Private),
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok());
}
