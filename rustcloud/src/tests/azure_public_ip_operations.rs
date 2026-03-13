use crate::azure::azure_apis::network::azure_public_ip::AzurePublicIPClient;

#[tokio::test]
async fn test_create_public_ip() {
    let client = AzurePublicIPClient::new();

    let result = client
        .create_public_ip("test-rg", "test-ip", "eastasia")
        .await;

    match &result {
        Ok(_) => println!("Public IP created successfully"),
        Err(e) => println!("Azure Public IP creation failed with error: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}
