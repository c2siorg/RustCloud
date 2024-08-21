use crate::aws::aws_apis::compute::aws_ecs::*;

use aws_config;
use aws_sdk_ecs::config::Region;
use aws_sdk_ecs::types::{ClusterConfiguration, ClusterSetting, ClusterSettingName, Tag};
use aws_sdk_ecs::{Client, Config};

async fn create_client() -> Client {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    return client;
}
#[tokio::test]
async fn test_create_ecs_cluster() {
    let client = create_client().await;

    let name = "test-cluster".to_string();
    let tags = Some(vec![
        Tag::builder().key("Environment").value("Test").build(),
        Tag::builder().key("Name").value("Test Cluster").build(),
    ]);
    // let settings = ClusterSetting::builder()
    // .value("some-value")
    // .build(); // Replace with your desired settings
    let settings = None;
    let configuration = ClusterConfiguration::builder().build(); // Replace with your desired configuration
    let capacity_providers = None; // Replace with your desired capacity providers if any

    let result = create_cluster(
        &client,
        &name,
        tags,
        settings,
        configuration,
        capacity_providers,
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_ecs_cluster() {
    let client = create_client().await;

    let name = "test-cluster".to_string();

    let result = delete_cluster(&client, &name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_describe_ecs_cluster() {
    let client = create_client().await;

    let clusters = Some(vec!["test-cluster".to_string()]); // Optionally specify cluster names to describe
    let include = None; // Optionally specify fields to include in the description

    let result = describe_cluster(&client, clusters, include).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_show_ecs_clusters() {
    let client = create_client().await;

    let max_results = Some(10);

    let result = show_clusters(&client, max_results).await;
    assert!(result.is_ok());
}
