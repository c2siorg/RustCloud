use crate::gcp::gcp_apis::network::gcp_dns::*;
use std::collections::HashMap;
use tokio::test;
use serde_json::json;

async fn create_client() -> GoogleDns {
    GoogleDns::new()
}

#[tokio::test]
async fn test_list_resource_dns_record_sets() {
    let client = create_client().await;
    let project_id = "rare-daylight-403814".to_string();
    let mut options = HashMap::new();
    options.insert("managedZone", "rustcloudtest");
    options.insert("maxResults", "10");

    let result = client.list_resource_dns_record_sets(project_id,&options).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}

#[tokio::test]
async fn test_create_dns() {
    let client = create_client().await;
    let project_id = "rare-daylight-403814".to_string();

    let mut params = HashMap::new();
    params.insert("Project".to_string(), json!("rare-daylight-403814"));
    params.insert("Description".to_string(), json!("Test DNS Description"));
    params.insert("DnsName".to_string(), json!("test.dns1.name."));
    // params.insert(
    //     "nameServers",
    //     "ns-cloud-a1.googledomains.com,ns-cloud-a2.googledomains.com",
    // );
    // params.insert("Id", "12345");
    params.insert("Kind".to_string(), json!("dns#managedZone"));
    params.insert("Name".to_string(), json!("test-dns1"));
    // params.insert("nameServerSet", "test-name-server-set");

    let result = client.create_dns(project_id, params).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string())
}

#[tokio::test]
async fn test_list_gcp_dns() {
    let client = create_client().await;
    let project_id = "rare-daylight-403814".to_string();

    let mut options = HashMap::new();
    options.insert("maxResults", "10");

    let result = client.list_dns(project_id, &options).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}

#[tokio::test]
async fn test_delete_dns() {
    let client = create_client().await;
    let project_id = "rare-daylight-403814".to_string();

    let mut options = HashMap::new();
    options.insert("managedZone", "rustcloudtest");

    let result = client.delete_dns(project_id, &options).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}
