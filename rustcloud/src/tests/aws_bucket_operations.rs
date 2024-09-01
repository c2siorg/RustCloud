use crate::aws::aws_apis::storage::aws_storage_bucket::*;
use aws_sdk_s3::config::Region;
use aws_sdk_s3::{
    types::{BucketCannedAcl, CreateBucketConfiguration, ObjectOwnership, RequestPayer},
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
