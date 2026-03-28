use crate::gcp::gcp_apis::storage::gcp_object_storage::GcsClient;

fn get_client() -> GcsClient {
    let project_id = std::env::var("GCP_PROJECT_ID").unwrap_or("your-project-id".to_string());
    GcsClient::new(&project_id)
}

#[tokio::test]
async fn test_create_bucket() {
    let client = get_client();
    let result = client
        .create_bucket("rustcloud-test-bucket-01", "US-EAST1")
        .await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    println!("Created bucket: {:?}", resp["status"]);
}

#[tokio::test]
async fn test_list_buckets() {
    let client = get_client();
    let result = client.list_buckets().await;
    assert!(result.is_ok());
    println!("Buckets: {:?}", result.unwrap()["body"]["items"]);
}

#[tokio::test]
async fn test_upload_and_download_object() {
    let client = get_client();
    let bucket = "rustcloud-test-bucket-01";
    let content = b"hello from rustcloud".to_vec();

    let upload = client
        .upload_object(bucket, "test/hello.txt", "text/plain", content.clone())
        .await;
    assert!(upload.is_ok());
    println!("Uploaded: {:?}", upload.unwrap()["status"]);

    let downloaded = client.download_object(bucket, "test/hello.txt").await;
    assert!(downloaded.is_ok());
    let bytes = downloaded.unwrap();
    assert_eq!(bytes, content);
    println!("Downloaded {} bytes", bytes.len());
}

#[tokio::test]
async fn test_list_objects() {
    let client = get_client();
    let result = client
        .list_objects("rustcloud-test-bucket-01", Some("test/"))
        .await;
    assert!(result.is_ok());
    println!("Objects: {:?}", result.unwrap()["body"]["items"]);
}

#[tokio::test]
async fn test_delete_object_and_bucket() {
    let client = get_client();
    let bucket = "rustcloud-test-bucket-01";

    let obj_result = client.delete_object(bucket, "test/hello.txt").await;
    assert!(obj_result.is_ok());

    let bucket_result = client.delete_bucket(bucket).await;
    assert!(bucket_result.is_ok());
    println!("Cleanup complete");
}
