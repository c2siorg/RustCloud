use std::collections::HashMap;
use std::sync::Arc;

use wiremock::matchers::{header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::gcp::gcp_apis::app_services::gcp_notification_service::Googlenotification;
use crate::gcp::gcp_apis::auth::gcp_auth::MockTokenProvider;

async fn mock_client(server: &MockServer) -> Googlenotification {
    Googlenotification::with_http_client(
        reqwest::Client::new(),
        server.uri(),
        Arc::new(MockTokenProvider::new("fake-token")),
    )
}

#[tokio::test]
async fn test_list_topic() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/test-project/topics"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut request = HashMap::new();
    request.insert("Project".to_string(), "test-project".to_string());
    request.insert("PageSize".to_string(), "10".to_string());
    request.insert("PageToken".to_string(), "token".to_string());

    let result = client.list_topic(request).await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_get_topic() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path("/v1/projects/test-project/topics/test-topic"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut request = HashMap::new();
    request.insert("Project".to_string(), "test-project".to_string());
    request.insert("Topic".to_string(), "test-topic".to_string());

    let result = client.get_topic(request).await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_delete_topic() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path("/v1/projects/test-project/topics/test-topic"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut request = HashMap::new();
    request.insert("Project".to_string(), "test-project".to_string());
    request.insert("Topic".to_string(), "test-topic".to_string());

    let result = client.delete_topic(request).await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
async fn test_create_topic() {
    let server = MockServer::start().await;
    Mock::given(method("PUT"))
        .and(path("/v1/projects/test-project/topics/test-topic"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let mut request = HashMap::new();
    request.insert("Project".to_string(), "test-project".to_string());
    request.insert("Topic".to_string(), "test-topic".to_string());

    let result = client.create_topic(request).await;
    assert!(result.is_ok(), "{:?}", result.err());
}

#[tokio::test]
#[ignore]
async fn test_list_topic_live() {
    let client = Googlenotification::new();
    let mut request = HashMap::new();
    request.insert("Project".to_string(), "your_project_id".to_string());

    let result = client.list_topic(request).await;
    assert!(result.is_ok(), "{:?}", result.err());
}
