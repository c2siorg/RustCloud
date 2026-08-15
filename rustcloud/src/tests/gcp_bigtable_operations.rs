use std::sync::Arc;

use wiremock::matchers::{header_exists, method, path};
use wiremock::{Mock, MockServer, ResponseTemplate};

use crate::gcp::gcp_apis::auth::gcp_auth::MockTokenProvider;
use crate::gcp::gcp_apis::database::gcp_bigtable::Bigtable;
use crate::gcp::types::database::gcp_bigtable_types::*;

async fn mock_client(server: &MockServer) -> Bigtable {
    Bigtable::with_http_client(
        reqwest::Client::new(),
        "test-project",
        server.uri(),
        Arc::new(MockTokenProvider::new("fake-token")),
    )
}

#[tokio::test]
async fn test_list_tables() {
    let server = MockServer::start().await;
    Mock::given(method("GET"))
        .and(path(
            "/v2/projects/test-project/instances/test-instance/tables",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let parent = "projects/test-project/instances/test-instance";
    let result = client.list_tables(parent, None, None).await;

    assert!(result.is_ok(), "{:?}", result.as_ref().err());
    assert_eq!(result.unwrap()["status"], 200);
}

#[tokio::test]
async fn test_delete_tables() {
    let server = MockServer::start().await;
    Mock::given(method("DELETE"))
        .and(path(
            "/v2/projects/test-project/instances/test-instance/tables/test-table",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let name = "projects/test-project/instances/test-instance/tables/test-table";
    let result = client.delete_tables(name).await;

    assert!(result.is_ok(), "{:?}", result.as_ref().err());
    assert_eq!(result.unwrap()["status"], 200);
}

#[tokio::test]
async fn test_describe_tables() {
    let server = MockServer::start().await;
    Mock::given(method("PATCH"))
        .and(path(
            "/v2/projects/test-project/instances/test-instance/tables/test-table",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let name = "projects/test-project/instances/test-instance/tables/test-table";
    let result = client.describe_tables(name).await;

    assert!(result.is_ok(), "{:?}", result.as_ref().err());
    assert_eq!(result.unwrap()["status"], 200);
}

#[tokio::test]
async fn test_create_tables() {
    let server = MockServer::start().await;
    Mock::given(method("POST"))
        .and(path(
            "/v2/projects/test-project/instances/test-instance/tables",
        ))
        .and(header_exists("authorization"))
        .respond_with(ResponseTemplate::new(200))
        .mount(&server)
        .await;

    let client = mock_client(&server).await;
    let parent = "projects/test-project/instances/test-instance";
    let table_id = "test-table";
    let table = Table {
        name: "test_table".to_string(),
        granularity: "MILLIS".to_string(),
    };
    let initial_splits = vec![InitialSplits {
        key: "test_key".to_string(),
    }];
    let cluster_states = ClusterStates {
        replication_state: "READY".to_string(),
    };

    let result = client
        .create_tables(parent, table_id, table, initial_splits, cluster_states)
        .await;

    assert!(result.is_ok(), "{:?}", result.as_ref().err());
    assert_eq!(result.unwrap()["status"], 200);
}

#[tokio::test]
#[ignore]
async fn test_list_tables_live() {
    let client = Bigtable::new("your_project_id");
    let parent = "projects/your_project_id/instances/your_instance_id";
    let result = client.list_tables(parent, None, None).await;
    assert!(result.is_ok(), "{:?}", result.err());
}
