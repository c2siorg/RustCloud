use crate::azure::azure_apis::network::azure_load_balancer::AzureLoadBalancerClient;

#[tokio::test]
async fn test_create_load_balancer() {
    let client = AzureLoadBalancerClient::new();

    let public_ip_id = "/subscriptions/<SUBSCRIPTION_ID>/resourceGroups/test-rg/providers/Microsoft.Network/publicIPAddresses/test-ip";

    let result = client
        .create_load_balancer("test-rg", "test-loadbalancer", "eastasia", public_ip_id)
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Create LB failed: {:?}", result);
}

#[tokio::test]
async fn test_list_load_balancers_rg() {
    let client = AzureLoadBalancerClient::new();

    let result = client.list_load_balancers_rg("test-rg").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "List RG failed: {:?}", result);
}

#[tokio::test]
async fn test_list_load_balancers_subscription() {
    let client = AzureLoadBalancerClient::new();

    let result = client.list_load_balancers_subscription().await;

    println!("{:?}", result);
    assert!(result.is_ok(), "List subscription failed: {:?}", result);
}

#[tokio::test]
async fn test_get_load_balancer() {
    let client = AzureLoadBalancerClient::new();

    let result = client
        .get_load_balancer("test-rg", "test-loadbalancer")
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Get LB failed: {:?}", result);
}

#[tokio::test]
async fn test_delete_load_balancer() {
    let client = AzureLoadBalancerClient::new();

    let result = client
        .delete_load_balancer("test-rg", "test-loadbalancer")
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Delete LB failed: {:?}", result);
}
