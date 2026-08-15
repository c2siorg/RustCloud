use wiremock::matchers::{header_exists, method, path, query_param};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::azure::azure_apis::storage::azure_blob::AzureBlobClient;

// AzureAuth::generate_headers reads this directly and is only ever called from
// this module, so a fixed fake value here is safe even under parallel test threads.
fn set_fake_storage_key() {
    std::env::set_var("AZURE_STORAGE_KEY", "ZmFrZWtleWZha2VrZXlmYWtla2V5MTY=");
}

async fn mock_client(server: &MockServer) -> AzureBlobClient {
    set_fake_storage_key();
    AzureBlobClient::with_http_client(
        reqwest::Client::new(),
        "testaccount".to_string(),
        server.uri(),
    )
}

#[tokio::test]
async fn test_list_containers() {
    let server = MockServer::start().await;
    let body = "<EnumerationResults><Containers/></EnumerationResults>";

    Mock::given(method("GET"))
        .and(path("/"))
        .and(query_param("comp", "list"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200).set_body_string(body))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let result = client.list_containers().await;

    assert_eq!(result.unwrap(), body);
}

#[tokio::test]
async fn test_create_container() {
    let server = MockServer::start().await;

    Mock::given(method("PUT"))
        .and(path("/test-container"))
        .and(query_param("restype", "container"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(201))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let result = client.create_container("test-container").await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_container() {
    let server = MockServer::start().await;

    Mock::given(method("DELETE"))
        .and(path("/test-container"))
        .and(query_param("restype", "container"))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(202))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let result = client.delete_container("test-container").await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_list_containers_live() {
    let account = std::env::var("AZURE_STORAGE_ACCOUNT").expect("AZURE_STORAGE_ACCOUNT not set");
    let client = AzureBlobClient::new(account);

    let result = client.list_containers().await;

    assert!(result.is_ok(), "{:?}", result);
}
