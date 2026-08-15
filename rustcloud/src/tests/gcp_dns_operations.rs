use std::collections::HashMap;
use std::sync::Arc;

use wiremock::matchers::{header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::gcp::gcp_apis::auth::gcp_auth::MockTokenProvider;
use crate::gcp::gcp_apis::network::gcp_dns::GoogleDns;

async fn mock_client(server: &MockServer) -> GoogleDns {
    GoogleDns::with_http_client(
        reqwest::Client::new(),
        "test-project",
        server.uri(),
        Arc::new(MockTokenProvider::new("fake-token")),
    )
}

#[tokio::test]
async fn test_list_resource_dns_record_sets() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/dns/v1/projects/test-project/managedZones/test-zone/rrsets",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut options = HashMap::new();
    options.insert("managedZone", "test-zone");
    options.insert("maxResults", "10");

    let result = client.list_resource_dns_record_sets(&options).await;
    assert!(result.is_ok(), "{:?}", result.err());
    assert!(result.unwrap().status().is_success());
}

#[tokio::test]
async fn test_create_dns() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path("/dns/v1/projects/test-project/managedZones"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut params = HashMap::new();
    params.insert("Project", "test-project");
    params.insert("Description", "Test DNS Description");
    params.insert("DnsName", "test.dns.name.");
    params.insert(
        "nameServers",
        "ns-cloud-a1.googledomains.com,ns-cloud-a2.googledomains.com",
    );
    params.insert("Id", "12345");
    params.insert("Kind", "dns#managedZone");
    params.insert("Name", "test-dns");
    params.insert("nameServerSet", "test-name-server-set");

    let result = client.create_dns(&params).await;
    assert!(result.is_ok(), "{:?}", result.err());
    assert!(result.unwrap().status().is_success());
}

#[tokio::test]
async fn test_list_dns() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/dns/v1/projects/test-project/managedZones/"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut options = HashMap::new();
    options.insert("maxResults", "10");

    let result = client.list_dns(&options).await;
    assert!(result.is_ok(), "{:?}", result.err());
    assert!(result.unwrap().status().is_success());
}

#[tokio::test]
async fn test_delete_dns() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/dns/v1/projects/test-project/managedZones/test-zone"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut options = HashMap::new();
    options.insert("managedZone", "test-zone");

    let result = client.delete_dns(&options).await;
    assert!(result.is_ok(), "{:?}", result.err());
    assert!(result.unwrap().status().is_success());
}

#[tokio::test]
#[ignore]
async fn test_list_dns_live() {
    let client = GoogleDns::new("your_project_id");
    let options = HashMap::new();

    let result = client.list_dns(&options).await;
    assert!(result.is_ok(), "{:?}", result.err());
}
