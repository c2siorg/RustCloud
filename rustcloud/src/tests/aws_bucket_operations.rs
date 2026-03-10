use crate::aws::aws_apis::storage::aws_storage_bucket::*;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::{
    primitives::ByteStream,
    types::{
        BucketCannedAcl, CreateBucketConfiguration, MetadataDirective, ObjectCannedAcl,
        ObjectOwnership, RequestPayer,
    },
    Client, Error,
};
use tokio;

async fn create_client() -> Client {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    return client;
}

#[tokio::test]
async fn test_create_bucket() {
    let client = create_client().await;
    let acl = BucketCannedAcl::PublicRead;
    let bucket = "test-bucket".to_string();
    let create_bucket_configuration = CreateBucketConfiguration::builder()
        .location_constraint(aws_sdk_s3::types::BucketLocationConstraint::UsWest2)
        .build();
    let grant_full_control = None;
    let grant_read = None;
    let grant_read_acp = None;
    let grant_write = None;
    let grant_write_acp = None;
    let object_lock_enabled_for_bucket = None;
    let object_ownership = None;

    let result = create_bucket(
        &client,
        acl,
        bucket,
        create_bucket_configuration,
        grant_full_control,
        grant_read,
        grant_read_acp,
        grant_write,
        grant_write_acp,
        object_lock_enabled_for_bucket,
        object_ownership,
    )
    .await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_bucket() {
    let client = create_client().await;
    let bucket = "test-bucket".to_string();

    let result = delete(&client, bucket.clone(), None).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_object() {
    let client = create_client().await;
    let bucket = "test-bucket".to_string();
    let key = "test-object".to_string();

    let result = delete_object(&client, bucket.clone(), key.clone(), None, None, None, None).await;

    assert!(result.is_ok());

    // Clean up
    delete(&client, bucket, None).await.unwrap();
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

    let result = list_objects_v2(&client, bucket, None, None, Some(100), None, None, None, None)
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
