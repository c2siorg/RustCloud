use crate::azure::azure_apis::network::azure_vnet::AzureVNetClient;

#[tokio::test]
async fn test_create_vnet() {
    let client = AzureVNetClient::new();

    let result = client.create_vnet("test-rg", "test-vnet", "eastasia").await;

    match &result {
        Ok(_) => println!("VNet created successfully"),
        Err(e) => println!("Azure VNet creation failed with error: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}
