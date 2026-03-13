use crate::azure::azure_apis::network::azure_nat_gateway::AzureNatGatewayClient;

#[tokio::test]
async fn test_create_nat_gateway() {
    let client = AzureNatGatewayClient::new();

    let result = client
        .create_nat_gateway("test-rg", "rustcloud-nat", "eastasia")
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Create NAT Gateway failed: {:?}", result);
}

#[tokio::test]
async fn test_list_nat_gateways_rg() {
    let client = AzureNatGatewayClient::new();

    let result = client.list_nat_gateways_rg("test-rg").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "List NAT RG failed: {:?}", result);
}

#[tokio::test]
async fn test_list_nat_gateways_subscription() {
    let client = AzureNatGatewayClient::new();

    let result = client.list_nat_gateways_subscription().await;

    println!("{:?}", result);
    assert!(result.is_ok(), "List NAT subscription failed: {:?}", result);
}

#[tokio::test]
async fn test_get_nat_gateway() {
    let client = AzureNatGatewayClient::new();

    let result = client.get_nat_gateway("test-rg", "rustcloud-nat").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Get NAT Gateway failed: {:?}", result);
}

#[tokio::test]
async fn test_associate_public_ip_with_nat() {
    let client = AzureNatGatewayClient::new();

    let public_ip_id = "/subscriptions/<SUBSCRIPTION_ID>/resourceGroups/test-rg/providers/Microsoft.Network/publicIPAddresses/test-ip";

    let result = client
        .associate_public_ip("test-rg", "rustcloud-nat", public_ip_id, "eastasia")
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Associate public IP failed: {:?}", result);
}

#[tokio::test]
async fn test_remove_public_ips_from_nat() {
    let client = AzureNatGatewayClient::new();

    let result = client
        .remove_public_ips("test-rg", "rustcloud-nat", "eastasia")
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Remove public IPs failed: {:?}", result);
}

#[tokio::test]
async fn test_delete_nat_gateway() {
    let client = AzureNatGatewayClient::new();

    let result = client.delete_nat_gateway("test-rg", "rustcloud-nat").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Delete NAT Gateway failed: {:?}", result);
}
