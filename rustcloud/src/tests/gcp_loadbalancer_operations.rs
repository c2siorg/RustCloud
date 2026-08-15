use std::collections::HashMap;
use std::sync::Arc;

use wiremock::matchers::{header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::gcp::gcp_apis::auth::gcp_auth::MockTokenProvider;
use crate::gcp::gcp_apis::network::gcp_loadbalancer::GoogleLoadBalancer;

async fn mock_client(server: &MockServer) -> GoogleLoadBalancer {
    GoogleLoadBalancer::with_http_client(
        reqwest::Client::new(),
        "test-project",
        server.uri(),
        Arc::new(MockTokenProvider::new("fake-token")),
    )
}

#[tokio::test]
async fn test_create_load_balancer() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/compute/beta/projects/test-project/regions/us-central1/targetPools",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut params = HashMap::new();
    params.insert("Project", "test-project");
    params.insert("Name", "test-lb");
    params.insert("Region", "us-central1");
    params.insert("healthChecks", "healthCheck1,healthCheck2");
    params.insert("Instances", "instance1,instance2");

    let result = client.create_load_balancer(&params).await;
    assert!(result.is_ok(), "{:?}", result.err());
    assert!(result.unwrap().status().is_success());
}

#[tokio::test]
async fn test_delete_load_balancer() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(
            "/compute/beta/projects/test-project/regions/us-central1/targetPools/test-lb",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut options = HashMap::new();
    options.insert("Project", "test-project");
    options.insert("Region", "us-central1");
    options.insert("TargetPool", "test-lb");

    let result = client.delete_load_balancer(&options).await;
    assert!(result.is_ok(), "{:?}", result.err());
    assert!(result.unwrap().status().is_success());
}

#[tokio::test]
async fn test_list_load_balancer() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/compute/beta/projects/test-project/regions/us-central1/targetPools",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut options = HashMap::new();
    options.insert("Project", "test-project");
    options.insert("Region", "us-central1");

    let result = client.list_load_balancer(&options).await;
    assert!(result.is_ok(), "{:?}", result.err());
    assert!(result.unwrap().status().is_success());
}

#[tokio::test]
async fn test_attach_node_with_load_balancer() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/compute/beta/projects/test-project/regions/us-central1/targetPools/test-lb/addInstance",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut params = HashMap::new();
    params.insert("Project", "test-project");
    params.insert("TargetPool", "test-lb");
    params.insert("Region", "us-central1");
    params.insert("Instances", "instance1,instance2");

    let result = client.attach_node_with_load_balancer(&params).await;
    assert!(result.is_ok(), "{:?}", result.err());
    assert!(result.unwrap().status().is_success());
}

#[tokio::test]
async fn test_detach_node_with_load_balancer() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/compute/beta/projects/test-project/regions/us-central1/targetPools/test-lb/removeInstance",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut params = HashMap::new();
    params.insert("Project", "test-project");
    params.insert("TargetPool", "test-lb");
    params.insert("Region", "us-central1");
    params.insert("Instances", "instance1,instance2");

    let result = client.detach_node_with_load_balancer(&params).await;
    assert!(result.is_ok(), "{:?}", result.err());
    assert!(result.unwrap().status().is_success());
}

#[tokio::test]
#[ignore]
async fn test_list_load_balancer_live() {
    let client = GoogleLoadBalancer::new("your_project_id");
    let mut options = HashMap::new();
    options.insert("Project", "your_project_id");
    options.insert("Region", "us-central1");

    let result = client.list_load_balancer(&options).await;
    assert!(result.is_ok(), "{:?}", result.err());
}
