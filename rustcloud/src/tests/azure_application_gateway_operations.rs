use crate::azure::azure_apis::network::azure_application_gateway::AzureApplicationGatewayClient;

#[tokio::test]
async fn test_create_application_gateway() {
    let client = AzureApplicationGatewayClient::new();

    let subnet_id = "/subscriptions/<SUBSCRIPTION_ID>/resourceGroups/test-rg/providers/Microsoft.Network/virtualNetworks/test-vnet/subnets/test-subnet";
    let public_ip_id = "/subscriptions/<SUBSCRIPTION_ID>/resourceGroups/test-rg/providers/Microsoft.Network/publicIPAddresses/test-ip";

    let result = client
        .create_application_gateway(
            "test-rg",
            "rustcloud-appgw",
            "eastasia",
            subnet_id,
            public_ip_id,
        )
        .await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "Create Application Gateway failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_list_application_gateways_rg() {
    let client = AzureApplicationGatewayClient::new();

    let result = client.list_application_gateways_rg("test-rg").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "List App Gateways RG failed: {:?}", result);
}

#[tokio::test]
async fn test_list_application_gateways_subscription() {
    let client = AzureApplicationGatewayClient::new();

    let result = client.list_application_gateways_subscription().await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "List App Gateways Subscription failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_get_application_gateway() {
    let client = AzureApplicationGatewayClient::new();

    let result = client
        .get_application_gateway("test-rg", "rustcloud-appgw")
        .await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "Get Application Gateway failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_stop_application_gateway() {
    let client = AzureApplicationGatewayClient::new();

    let result = client
        .stop_application_gateway("test-rg", "rustcloud-appgw")
        .await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "Stop Application Gateway failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_start_application_gateway() {
    let client = AzureApplicationGatewayClient::new();

    let result = client
        .start_application_gateway("test-rg", "rustcloud-appgw")
        .await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "Start Application Gateway failed: {:?}",
        result
    );
}

#[tokio::test]
async fn test_delete_application_gateway() {
    let client = AzureApplicationGatewayClient::new();

    let result = client
        .delete_application_gateway("test-rg", "rustcloud-appgw")
        .await;

    println!("{:?}", result);
    assert!(
        result.is_ok(),
        "Delete Application Gateway failed: {:?}",
        result
    );
}
