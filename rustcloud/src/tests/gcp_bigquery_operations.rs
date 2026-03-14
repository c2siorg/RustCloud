use crate::gcp::gcp_apis::database::gcp_bigquery::*;
use crate::gcp::types::database::gcp_bigquery_types::*;

fn project_id() -> String {
    std::env::var("GCP_PROJECT_ID").unwrap_or_else(|_| "your_project_id".to_string())
}

async fn create_client() -> BigQuery {
    BigQuery::new(&project_id())
}

#[tokio::test]
async fn test_create_dataset() {
    let client = create_client().await;
    client.delete_dataset("test_create_ds", true).await.ok();
    let result = client.create_dataset("test_create_ds").await;
    client.delete_dataset("test_create_ds", true).await.ok();
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], 200);
}

#[tokio::test]
async fn test_delete_dataset() {
    let client = create_client().await;
    client.create_dataset("test_delete_ds").await.ok();
    let result = client.delete_dataset("test_delete_ds", true).await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], 204);
}

#[tokio::test]
async fn test_list_datasets() {
    let client = create_client().await;
    let result = client.list_datasets().await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], 200);
}

#[tokio::test]
async fn test_create_table() {
    let client = create_client().await;
    client.create_dataset("test_create_table_ds").await.ok();
    let fields = vec![
        TableField {
            name: "id".to_string(),
            field_type: "INTEGER".to_string(),
        },
        TableField {
            name: "name".to_string(),
            field_type: "STRING".to_string(),
        },
    ];
    let result = client
        .create_table("test_create_table_ds", "test_tbl", fields)
        .await;
    client
        .delete_dataset("test_create_table_ds", true)
        .await
        .ok();
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], 200);
}

#[tokio::test]
async fn test_delete_table() {
    let client = create_client().await;
    client.create_dataset("test_delete_table_ds").await.ok();
    let fields = vec![TableField {
        name: "id".to_string(),
        field_type: "INTEGER".to_string(),
    }];
    client
        .create_table("test_delete_table_ds", "test_tbl", fields)
        .await
        .ok();
    let result = client
        .delete_table("test_delete_table_ds", "test_tbl")
        .await;
    client
        .delete_dataset("test_delete_table_ds", true)
        .await
        .ok();
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], 204);
}

#[tokio::test]
async fn test_list_tables() {
    let client = create_client().await;
    client.create_dataset("test_list_tables_ds").await.ok();
    let result = client.list_tables("test_list_tables_ds").await;
    client
        .delete_dataset("test_list_tables_ds", true)
        .await
        .ok();
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], 200);
}

#[tokio::test]
async fn test_run_query() {
    let client = create_client().await;
    let result = client.run_query("SELECT 1").await;
    assert!(result.is_ok());
    let response = result.unwrap();
    assert_eq!(response["status"], 200);
}
