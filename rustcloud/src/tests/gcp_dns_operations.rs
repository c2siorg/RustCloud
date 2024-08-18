use crate::gcp::gcp_apis::network::gcp_dns::*;
use std::collections::HashMap;
use tokio::test;

async fn create_client() -> GoogleDns {
    GoogleDns::new("your_project_id")
}

#[tokio::test]
async fn test_list_resource_dns_record_sets() {
    let client = create_client().await;

    let mut options = HashMap::new();
    options.insert("managedZone", "your_managed_zone");
    options.insert("maxResults", "10");

    let result = client.list_resource_dns_record_sets(&options).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_create_dns() {
    let client = create_client().await;

    let mut params = HashMap::new();
    params.insert("Project", "your_project_id");
    params.insert("Description", "Test DNS Description");
    params.insert("DnsName", "test.dns.name.");
    params.insert("nameServers", "ns-cloud-a1.googledomains.com,ns-cloud-a2.googledomains.com");
    params.insert("Id", "12345");
    params.insert("Kind", "dns#managedZone");
    params.insert("Name", "test-dns");
    params.insert("nameServerSet", "test-name-server-set");

    let result = client.create_dns(&params).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_list_dns() {
    let client = create_client().await;

    let mut options = HashMap::new();
    options.insert("maxResults", "10");

    let result = client.list_dns(&options).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.status().is_success());
}

#[tokio::test]
async fn test_delete_dns() {
    let client = create_client().await;

    let mut options = HashMap::new();
    options.insert("managedZone", "your_managed_zone");

    let result = client.delete_dns(&options).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert!(response.status().is_success());
}
