use crate::azure::azure_apis::storage::azure_blob::AzureBlobClient;

async fn create_client() -> AzureBlobClient {
    let account = std::env::var("AZURE_STORAGE_ACCOUNT").expect("AZURE_STORAGE_ACCOUNT not set");

    AzureBlobClient::new(account)
}

#[tokio::test]
async fn test_list_containers() {
    let client = create_client().await;

    let result = client.list_containers().await;

    match result {
        Ok(containers) => {
            println!("Containers response:\n{}", containers);
        }
        Err(e) => {
            panic!("Azure error: {:?}", e);
        }
    }
}

#[tokio::test]
async fn test_create_container() {
    let client = create_client().await;

    let container = "test-container";

    let result = client.create_container(container).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_container() {
    let client = create_client().await;

    let container = "test-container";

    let result = client.delete_container(container).await;

    assert!(result.is_ok(), "{:?}", result);
}
