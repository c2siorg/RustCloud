use crate::azure::azure_apis::management::azure_resource_group::AzureResourceGroupClient;

#[tokio::test]
async fn test_create_resource_group() {
    let client = AzureResourceGroupClient::new();

    let result = client
        .create_resource_group("rustcloud-rg", "eastasia")
        .await;

    match &result {
        Ok(_) => println!("Resource Group created successfully"),
        Err(e) => println!("Azure Resource Group creation failed with error: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}
