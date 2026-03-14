use crate::azure::azure_apis::storage::azure_file_storage::AzureFileClient;

async fn create_client() -> AzureFileClient {
    let account = std::env::var("AZURE_STORAGE_ACCOUNT").expect("AZURE_STORAGE_ACCOUNT not set");

    AzureFileClient::new(account)
}

#[tokio::test]
async fn test_list_shares_storage() {
    let client = create_client().await;

    let result = client.list_shares().await;

    println!("{:?}", result);

    assert!(result.is_ok(), "List shares failed: {:?}", result);
}

#[tokio::test]
async fn test_create_share_storage() {
    let client = create_client().await;

    let share = "test-share";

    let result = client.create_share(share).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Create share failed: {:?}", result);
}

#[tokio::test]
async fn test_create_directory() {
    let client = create_client().await;

    let share = "test-share";
    let directory = "test-dir";

    let result = client.create_directory(share, directory).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Create directory failed: {:?}", result);
}

#[tokio::test]
async fn test_list_files() {
    let client = create_client().await;

    let share = "test-share";
    let directory = "test-dir";

    let result = client.list_files(share, directory).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "List files failed: {:?}", result);
}

#[tokio::test]
async fn test_create_file() {
    let client = create_client().await;

    let share = "test-share";
    let directory = "test-dir";
    let file = "test-file.txt";

    let result = client.create_file(share, directory, file, 1024).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Create file failed: {:?}", result);
}

#[tokio::test]
async fn test_upload_file_range() {
    let client = create_client().await;

    let share = "test-share";
    let directory = "test-dir";
    let file = "test-file.txt";

    let content = b"Hello from RustCloud File Storage!".to_vec();

    let result = client
        .upload_file_range(share, directory, file, content)
        .await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Upload file range failed: {:?}", result);
}

#[tokio::test]
async fn test_download_file() {
    let client = create_client().await;

    let share = "test-share";
    let directory = "test-dir";
    let file = "test-file.txt";

    let result = client.download_file(share, directory, file).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Download file failed: {:?}", result);
}

#[tokio::test]
async fn test_get_file_properties() {
    let client = create_client().await;

    let share = "test-share";
    let directory = "test-dir";
    let file = "test-file.txt";

    let result = client.get_file_properties(share, directory, file).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Get file properties failed: {:?}", result);
}

#[tokio::test]
async fn test_delete_file() {
    let client = create_client().await;

    let share = "test-share";
    let directory = "test-dir";
    let file = "test-file.txt";

    let result = client.delete_file(share, directory, file).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Delete file failed: {:?}", result);
}

#[tokio::test]
async fn test_delete_directory() {
    let client = create_client().await;

    let share = "test-share";
    let directory = "test-dir";

    let result = client.delete_directory(share, directory).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Delete directory failed: {:?}", result);
}

#[tokio::test]
async fn test_delete_share_storage() {
    let client = create_client().await;

    let share = "test-share";

    let result = client.delete_share(share).await;

    println!("{:?}", result);

    assert!(result.is_ok(), "Delete share failed: {:?}", result);
}
