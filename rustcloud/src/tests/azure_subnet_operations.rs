use crate::azure::azure_apis::network::azure_subnet::AzureSubnetClient;

#[tokio::test]
async fn test_create_subnet() {
    let client = AzureSubnetClient::new();

    let result = client
        .create_subnet("test-rg", "test-vnet", "test-subnet")
        .await;

    match &result {
        Ok(_) => println!("Subnet created successfully"),
        Err(e) => println!("Azure Subnet creation failed with error: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}
