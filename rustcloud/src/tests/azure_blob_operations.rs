use crate::azure::azure_apis::storage::azure_blob::AzureBlobClient;

async fn create_client() -> AzureBlobClient {
    let account = std::env::var("AZURE_STORAGE_ACCOUNT").expect("AZURE_STORAGE_ACCOUNT not set");

    AzureBlobClient::new(account)
}

#[tokio::test]
async fn test_list_containers() {
    let client = create_client().await;

    let result = client.list_containers().await;

    println!("{:?}", result);

    assert!(result.is_ok(), "List containers failed: {:?}", result);
}

#[tokio::test]
async fn test_create_container() {
    let client = create_client().await;

    let container = "test-container";

    let result = client.create_container(container).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Create container failed: {:?}", result);
}

#[tokio::test]
async fn test_delete_container() {
    let client = create_client().await;

    let container = "test-container";

    let result = client.delete_container(container).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Delete container failed: {:?}", result);
}

#[tokio::test]
async fn test_upload_blob() {
    let client = create_client().await;

    let container = "test-container";
    let blob_name = "sample.txt";

    let data = b"Hello from RustCloud!".to_vec();

    let result = client.upload_blob(container, blob_name, data).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Upload blob failed: {:?}", result);
}

#[tokio::test]
async fn test_list_blobs() {
    let client = create_client().await;

    let container = "test-container";

    let result = client.list_blobs(container).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "List blobs failed: {:?}", result);
}

#[tokio::test]
async fn test_delete_blob() {
    let client = create_client().await;

    let container = "test-container";
    let blob_name = "sample.txt";

    let result = client.delete_blob(container, blob_name).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Delete blob failed: {:?}", result);
}
