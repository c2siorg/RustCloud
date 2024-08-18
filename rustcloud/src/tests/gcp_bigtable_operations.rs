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

    let parent = "projects/your_project_id/instances/your_instance_id";
    let result = client.list_tables(parent, None, None).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], 200);
}

#[tokio::test]
async fn test_delete_tables() {
    let client = create_client().await;

    let name = "projects/your_project_id/instances/your_instance_id/tables/your_table_id";
    let result = client.delete_tables(name).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], 200);
}

#[tokio::test]
async fn test_describe_tables() {
    let client = create_client().await;

    let name = "projects/your_project_id/instances/your_instance_id/tables/your_table_id";
    let result = client.describe_tables(name).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], 200);
}

#[tokio::test]
async fn test_create_tables() {
    let client = create_client().await;

    let parent = "projects/your_project_id/instances/your_instance_id";
    let table_id = "your_table_id";
    let table = Table {
        // Populate Table struct fields
    };
    let initial_splits = vec![
        InitialSplits {
            // Populate InitialSplits struct fields
        },
    ];
    let cluster_states = ClusterStates {
        // Populate ClusterStates struct fields
    };

    let result = client.create_tables(parent, table_id, table, initial_splits, cluster_states).await;

    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], 200);
}
