use crate::azure::azure_apis::network::azure_nic::AzureNICClient;

#[tokio::test]
async fn test_create_nic() {
    let client = AzureNICClient::new();

    let subnet_id = "/subscriptions/<SUBSCRIPTION_ID>/resourceGroups/test-rg/providers/Microsoft.Network/virtualNetworks/test-vnet/subnets/test-subnet";
    let public_ip_id = "/subscriptions/<SUBSCRIPTION_ID>/resourceGroups/test-rg/providers/Microsoft.Network/publicIPAddresses/test-ip";

    let result = client
        .create_nic("test-rg", "test-nic", "eastasia", subnet_id, public_ip_id)
        .await;

    match &result {
        Ok(_) => println!("NIC created successfully"),
        Err(e) => println!("Azure NIC creation failed with error: {:?}", e),
    }

    assert!(result.is_ok(), "Detailed error: {:?}", result);
}
