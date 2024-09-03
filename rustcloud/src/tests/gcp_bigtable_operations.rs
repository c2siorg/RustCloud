use std::collections::HashMap;

use crate::gcp::gcp_apis::database::gcp_bigtable::*;
use crate::gcp::types::database::gcp_bigtable_types::*;
use serde_json::json;
use tokio::test;

async fn create_client() -> Bigtable {
    Bigtable::new("your_project_id")
}

#[tokio::test]
async fn test_list_tables() {
    let client = create_client().await;

    let parent = "projects/rare-daylight-403814/instances/rustcloudtest";
    let result = client.list_tables(parent, None, None).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}

#[tokio::test]
async fn test_delete_tables() {
    let client = create_client().await;

    let name = "projects/rare-daylight-403814/instances/rustcloudtest/tables/your_table_id1";
    let result = client.delete_tables(name).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}

#[tokio::test]
async fn test_describe_tables() {
    let client = create_client().await;

    let name = "projects/rare-daylight-403814/instances/rustcloudtest/tables/test";
    let mut table = Table {
        name: Some("Test2".to_string()),
        cluster_states: Some(HashMap::new()),
        column_families: Some(HashMap::new()),
        granularity: None,
        restore_info: None,
        change_stream_config: None,
        deletion_protection: None,
        stats: None,
        automated_backup_policy: None,
    };
    let result = client.describe_tables(name, "changeStreamConfig", table).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}

#[tokio::test]
async fn test_create_tables() {
    let client = create_client().await;

    let parent = "projects/rare-daylight-403814/instances/rustcloudtest";
    let table_id = "your_table_id1";
    let mut table = Table {
        name: None,
        cluster_states: Some(HashMap::new()),
        column_families: Some(HashMap::new()),
        granularity: None,
        restore_info: None,
        change_stream_config: None,
        deletion_protection: None,
        stats: None,
        automated_backup_policy: None,
    };


    let initial_splits: Option<Vec<InitialSplits>> = None;

    let result = client
        .create_table(parent, table_id, table, initial_splits)
        .await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], "200".to_string());
}
