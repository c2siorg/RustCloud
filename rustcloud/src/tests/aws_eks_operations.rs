use aws_sdk_eks::config::{BehaviorVersion, Credentials, Region, SharedCredentialsProvider};
use aws_sdk_eks::types::{
    AmiTypes, KubernetesNetworkConfigRequest, Logging, NodegroupScalingConfig,
    UpdateAccessConfigRequest, VpcConfigRequest,
};
use aws_sdk_eks::{Client, Config};
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;
use std::collections::HashMap;

use crate::aws::aws_apis::compute::aws_eks::*;

fn mock_client(response_body: &str) -> Client {
    let request = http::Request::builder()
        .method("POST")
        .uri("https://eks.us-east-1.amazonaws.com/clusters")
        .body(SdkBody::empty())
        .unwrap();
    let response = http::Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(SdkBody::from(response_body))
        .unwrap();
    let http_client = StaticReplayClient::new(vec![ReplayEvent::new(request, response)]);

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
async fn test_create_eks_cluster() {
    let client = mock_client(r#"{"cluster":{"name":"test-cluster","status":"CREATING"}}"#);

    let result = create_cluster(
        &client,
        "test-cluster".to_string(),
        Some("1.21".to_string()),
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_create_node_group() {
    let client = mock_client(
        r#"{"nodegroup":{"nodegroupName":"test-nodegroup","clusterName":"test-cluster","status":"CREATING"}}"#,
    );

    let scaling_config = Some(
        NodegroupScalingConfig::builder()
            .desired_size(2)
            .min_size(1)
            .max_size(3)
            .build(),
    );

    let result = create_node_group(
        &client,
        "test-cluster".to_string(),
        "test-nodegroup".to_string(),
        None,
        scaling_config,
        None,
        Some(vec!["t3.medium".to_string()]),
        Some(AmiTypes::Al2X8664),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_nodegroup() {
    let client = mock_client(
        r#"{"nodegroup":{"nodegroupName":"test-nodegroup","clusterName":"test-cluster","status":"DELETING"}}"#,
    );

    let result = delete_nodegroup(
        &client,
        "test-cluster".to_string(),
        "test-nodegroup".to_string(),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_describe_eks_cluster() {
    let client = mock_client(r#"{"cluster":{"name":"test-cluster","status":"ACTIVE"}}"#);

    let result = describe_cluster(&client, "test-cluster".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_describe_nodegroup() {
    let client = mock_client(
        r#"{"nodegroup":{"nodegroupName":"test-nodegroup","clusterName":"test-cluster","status":"ACTIVE"}}"#,
    );

    let result = describe_nodegroup(
        &client,
        "test-cluster".to_string(),
        "test-nodegroup".to_string(),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_eks_cluster() {
    let client = mock_client(r#"{"cluster":{"name":"test-cluster","status":"DELETING"}}"#);

    let result = delete_cluster(&client, "test-cluster").await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_list_eks_clusters() {
    let client = mock_client(r#"{"clusters":["test-cluster"]}"#);

    let result = list_clusters(&client, Some(10), None).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_list_nodegroups() {
    let client = mock_client(r#"{"nodegroups":["test-nodegroup"]}"#);

    let result = list_nodegroups(&client, "test-cluster".to_string(), Some(10)).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_update_tags() {
    let client = mock_client("{}");

    let mut tags = HashMap::new();
    tags.insert("key1".to_string(), "value1".to_string());

    let result = update_tags(
        &client,
        "arn:aws:eks:us-east-1:123456789012:cluster/test-cluster".to_string(),
        Some(tags),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_update_config() {
    let client = mock_client(r#"{"update":{"id":"11111111-2222-3333-4444-555555555555","status":"InProgress","type":"ConfigUpdate"}}"#);

    let result = update_config(
        &client,
        "test-cluster".to_string(),
        Some(VpcConfigRequest::builder().build()),
        Some(Logging::builder().build()),
        None,
        Some(UpdateAccessConfigRequest::builder().build()),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_describe_eks_cluster_live() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let result = describe_cluster(&client, "test-cluster".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}
