use aws_sdk_ecs::config::{BehaviorVersion, Credentials, Region, SharedCredentialsProvider};
use aws_sdk_ecs::types::{ClusterConfiguration, Tag};
use aws_sdk_ecs::{Client, Config};
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;

use crate::aws::aws_apis::compute::aws_ecs::*;

fn mock_client(response_bodies: &[&str]) -> Client {
    let events = response_bodies
        .iter()
        .map(|body| {
            let request = http::Request::builder()
                .method("POST")
                .uri("https://ecs.us-east-1.amazonaws.com/")
                .body(SdkBody::empty())
                .unwrap();
            let response = http::Response::builder()
                .status(200)
                .header("content-type", "application/x-amz-json-1.1")
                .body(SdkBody::from(*body))
                .unwrap();
            ReplayEvent::new(request, response)
        })
        .collect();
    let http_client = StaticReplayClient::new(events);

    let config = Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new("us-east-1"))
        .credentials_provider(SharedCredentialsProvider::new(Credentials::new(
            "fake-access-key",
            "fake-secret-key",
            None,
            None,
            "test",
        )))
        .http_client(http_client)
        .build();

    Client::from_conf(config)
}

#[tokio::test]
async fn test_create_ecs_cluster() {
    let client = mock_client(&[r#"{"cluster":{"clusterName":"test-cluster","status":"ACTIVE"}}"#]);

    let tags = Some(vec![
        Tag::builder().key("Environment").value("Test").build(),
        Tag::builder().key("Name").value("Test Cluster").build(),
    ]);
    let configuration = ClusterConfiguration::builder().build();

    let result = create_cluster(&client, &"test-cluster".to_string(), tags, None, configuration, None)
        .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_ecs_cluster() {
    let client = mock_client(&[r#"{"cluster":{"clusterName":"test-cluster","status":"INACTIVE"}}"#]);

    let result = delete_cluster(&client, &"test-cluster".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_describe_ecs_cluster() {
    let client = mock_client(&[r#"{"clusters":[{"clusterName":"test-cluster","status":"ACTIVE"}]}"#]);

    let result = describe_cluster(&client, Some(vec!["test-cluster".to_string()]), None).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_show_ecs_clusters() {
    // show_clusters chains ListClusters -> DescribeClusters, hence two queued events.
    let client = mock_client(&[
        r#"{"clusterArns":["arn:aws:ecs:us-east-1:123456789012:cluster/test-cluster"]}"#,
        r#"{"clusters":[{"clusterName":"test-cluster","clusterArn":"arn:aws:ecs:us-east-1:123456789012:cluster/test-cluster","status":"ACTIVE"}]}"#,
    ]);

    let result = show_clusters(&client, Some(10)).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_describe_ecs_cluster_live() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let result = describe_cluster(&client, Some(vec!["test-cluster".to_string()]), None).await;

    assert!(result.is_ok(), "{:?}", result);
}
