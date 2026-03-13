use crate::azure::azure_apis::compute::azure_container_instance::AzureContainerInstanceClient;

#[tokio::test]
async fn test_create_container_group() {
    let client = AzureContainerInstanceClient::new();

    let result = client
        .create_container_group(
            "test-rg",
            "test-container",
            "eastasia",
            "mcr.microsoft.com/azuredocs/aci-helloworld",
        )
        .await;

    match &result {
        Ok(_) => println!("Container created successfully"),
        Err(e) => println!("Azure container creation failed: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}

#[tokio::test]
async fn test_list_container_groups() {
    let client = AzureContainerInstanceClient::new();

    let result = client.list_container_groups("test-rg").await;

    match &result {
        Ok(_) => println!("Containers listed successfully"),
        Err(e) => println!("Azure list containers failed: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_get_container_group() {
    let client = AzureContainerInstanceClient::new();

    let result = client
        .get_container_group("test-rg", "test-container")
        .await;

    match &result {
        Ok(_) => println!("Container fetched successfully"),
        Err(e) => println!("Azure get container failed: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_restart_container_group() {
    let client = AzureContainerInstanceClient::new();

    let result = client
        .restart_container_group("test-rg", "test-container")
        .await;

    match &result {
        Ok(_) => println!("Container restarted successfully"),
        Err(e) => println!("Azure restart container failed: {:?}", e),
    }

    assert!(result.is_ok(), "Azure error: {:?}", result);
}

#[tokio::test]
async fn test_delete_container_group() {
    let client = AzureContainerInstanceClient::new();

    let result = client
        .delete_container_group("test-rg", "test-container")
        .await;

    match &result {
        Ok(_) => println!("Container deleted successfully"),
        Err(e) => println!("Azure container deletion failed: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}
