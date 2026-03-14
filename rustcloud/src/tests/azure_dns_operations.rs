use crate::azure::azure_apis::network::azure_dns::AzureDnsClient;

#[tokio::test]
async fn test_create_dns_zone() {
    let client = AzureDnsClient::new();

    let result = client
        .create_dns_zone("test-rg", "rustcloud-demo.com", "global")
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Create DNS zone failed: {:?}", result);
}

#[tokio::test]
async fn test_list_dns_zones_rg() {
    let client = AzureDnsClient::new();

    let result = client.list_dns_zones_rg("test-rg").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "List DNS RG failed: {:?}", result);
}

#[tokio::test]
async fn test_list_dns_zones_subscription() {
    let client = AzureDnsClient::new();

    let result = client.list_dns_zones_subscription().await;

    println!("{:?}", result);
    assert!(result.is_ok(), "List DNS subscription failed: {:?}", result);
}

#[tokio::test]
async fn test_get_dns_zone() {
    let client = AzureDnsClient::new();

    let result = client.get_dns_zone("test-rg", "rustcloud-demo.com").await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Get DNS zone failed: {:?}", result);
}

#[tokio::test]
async fn test_create_a_record() {
    let client = AzureDnsClient::new();

    let result = client
        .create_a_record("test-rg", "rustcloud-demo.com", "www", "1.2.3.4")
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Create A record failed: {:?}", result);
}

#[tokio::test]
async fn test_list_record_sets() {
    let client = AzureDnsClient::new();

    let result = client
        .list_record_sets("test-rg", "rustcloud-demo.com")
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "List record sets failed: {:?}", result);
}

#[tokio::test]
async fn test_delete_record() {
    let client = AzureDnsClient::new();

    let result = client
        .delete_record("test-rg", "rustcloud-demo.com", "A", "www")
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Delete record failed: {:?}", result);
}

#[tokio::test]
async fn test_delete_dns_zone() {
    let client = AzureDnsClient::new();

    let result = client
        .delete_dns_zone("test-rg", "rustcloud-demo.com")
        .await;

    println!("{:?}", result);
    assert!(result.is_ok(), "Delete DNS zone failed: {:?}", result);
}
