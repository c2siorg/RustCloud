use crate::gcp::gcp_apis::artificial_intelligence::gcp_automl::*;
use std::collections::HashMap;
use tokio::test;

async fn create_client() -> AutoML {
    AutoML::new("your_project_id")
}

#[tokio::test]
async fn test_create_dataset() {
    let client = create_client().await;

    let location = "your_location";
    let name = "your_dataset_name";

    let result = client.create_dataset(location, name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_dataset() {
    let client = create_client().await;

    let location = "your_location";
    let dataset_id = "your_dataset_id";

    let result = client.get_dataset(location, dataset_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_import_data_set() {
    let client = create_client().await;

    let location = "your_location";
    let dataset_id = "your_dataset_id";
    let uris = vec!["gs://your_bucket/your_file.csv".to_string()];

    let result = client.import_data_set(location, dataset_id, uris).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_models() {
    let client = create_client().await;

    let location = "your_location";

    let result = client.list_models(location).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_model() {
    let client = create_client().await;

    let location = "your_location";
    let dataset_id = "your_dataset_id";
    let model_name = "your_model_name";
    let column_id = "your_column_id";
    let train_budget = 1000;

    let result = client
        .create_model(location, dataset_id, model_name, column_id, train_budget)
        .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_deploy_model() {
    let client = create_client().await;

    let location = "your_location";
    let model_id = "your_model_id";

    let result = client.deploy_model(location, model_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_undeploy_model() {
    let client = create_client().await;

    let location = "your_location";
    let model_id = "your_model_id";

    let result = client.undeploy_model(location, model_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_model() {
    let client = create_client().await;

    let location = "your_location";
    let model_id = "your_model_id";

    let result = client.get_model(location, model_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_export_dataset() {
    let client = create_client().await;

    let location = "your_location";
    let dataset_id = "your_dataset_id";
    let gcs_uri = "gs://your_bucket/your_export_path/";

    let result = client.export_dataset(location, dataset_id, gcs_uri).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_model() {
    let client = create_client().await;

    let location = "your_location";
    let model_id = "your_model_id";

    let result = client.delete_model(location, model_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_dataset() {
    let client = create_client().await;

    let location = "your_location";
    let dataset_id = "your_dataset_id";

    let result = client.delete_dataset(location, dataset_id).await;
    assert!(result.is_ok());
}
