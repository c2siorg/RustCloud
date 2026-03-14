use crate::azure::azure_apis::storage::azure_storage_account::AzureStorageAccountClient;

#[tokio::test]
async fn test_check_storage_account_name() {
    let client = AzureStorageAccountClient::new();

    let result = client
        .check_storage_account_name("rustcloudstorage12345")
        .await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "Check name availability failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_create_storage_account() {
    let client = AzureStorageAccountClient::new();

    let result = client
        .create_storage_account("test-rg", "rustcloudstorage12345", "eastasia")
        .await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "Create storage account failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_list_storage_accounts_rg() {
    let client = AzureStorageAccountClient::new();

    let result = client.list_storage_accounts_resource_group("test-rg").await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "List storage accounts (RG) failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_list_storage_accounts_subscription() {
    let client = AzureStorageAccountClient::new();

    let result = client.list_storage_accounts_subscription().await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "List storage accounts (subscription) failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_get_storage_account() {
    let client = AzureStorageAccountClient::new();

    let result = client
        .get_storage_account("test-rg", "rustcloudstorage12345")
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Get storage account failed: {:?}", result);
}

#[tokio::test]
async fn test_list_storage_account_keys() {
    let client = AzureStorageAccountClient::new();

    let result = client
        .list_storage_account_keys("test-rg", "rustcloudstorage12345")
        .await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "List storage account keys failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_delete_storage_account() {
    let client = AzureStorageAccountClient::new();

    let result = client
        .delete_storage_account("test-rg", "rustcloudstorage12345")
        .await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "Delete storage account failed: {:?}",
        result
    );
}
