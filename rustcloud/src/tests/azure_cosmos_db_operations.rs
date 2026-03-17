use crate::azure::azure_apis::database::azure_cosmos_db::AzureCosmosDb;
use serde_json::json;

fn create_client() -> AzureCosmosDb {
    AzureCosmosDb::with_config(
        "https://rustcloud-test.documents.azure.com",
        "test-token",
    )
}

#[tokio::test]
async fn test_list_databases() {
    let client = create_client();
    let result = client.list_databases().await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_database() {
    let client = create_client();
    let result = client.create_database("rustcloud-testdb").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_container() {
    let client = create_client();
    let result = client
        .create_container("rustcloud-testdb", "items", "/id")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_upsert_document() {
    let client = create_client();
    let doc = json!({
        "id": "doc1",
        "name": "RustCloud test",
        "provider": "azure"
    });
    let result = client.upsert_document("rustcloud-testdb", "items", &doc).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_document() {
    let client = create_client();
    let result = client
        .get_document("rustcloud-testdb", "items", "doc1", "doc1")
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_query_documents() {
    let client = create_client();
    let result = client
        .query_documents(
            "rustcloud-testdb",
            "items",
            "SELECT * FROM c WHERE c.provider = 'azure'",
        )
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_container() {
    let client = create_client();
    let result = client.delete_container("rustcloud-testdb", "items").await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_database() {
    let client = create_client();
    let result = client.delete_database("rustcloud-testdb").await;
    assert!(result.is_ok());
}
