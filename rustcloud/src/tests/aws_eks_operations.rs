use crate::aws::aws_apis::compute::aws_eks::*;
use aws_sdk_eks::error::SdkError;
use aws_sdk_eks::{Client, Config};
use aws_sdk_eks::config::Region;
use aws_sdk_eks::types::{VpcConfigRequest, KubernetesNetworkConfigRequest, NodegroupScalingConfig, AmiTypes, Logging, UpdateAccessConfigRequest};
use std::collections::HashMap;


async fn create_client() -> Client {
    let config =  aws_config::load_from_env().await;
    let client =  Client::new(&config);
    return client;
}
#[tokio::test]
async fn test_create_eks_cluster() {
    let client = create_client().await;
    
    let cluster_name = "test-cluster".to_string();
    let version = Some("1.21".to_string());
    let role_arn = None;
    let resources_vpc_config = None;
    let kubernetes_network_config = None;

    let result = create_cluster(
        &client,
        cluster_name,
        version,
        role_arn,
        resources_vpc_config,
        kubernetes_network_config
    ).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_node_group() {
    let client = create_client().await;
    
    let cluster_name = "test-cluster".to_string();
    let nodegroup_name = "test-nodegroup".to_string();
    let scaling_config = Some(NodegroupScalingConfig::builder().desired_size(2).min_size(1).max_size(3).build());
    let subnets =None;
    let instance_types = Some(vec!["t3.medium".to_string()]);
    let ami_type = Some(AmiTypes::Al2X8664);
    let node_role =None;

    let result = create_node_group(&client, cluster_name, nodegroup_name, None, scaling_config, subnets, instance_types, ami_type, None, node_role, None, None, None, None, None, None, None, None, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_nodegroup() {
    let client = create_client().await;
    
    let cluster_name = "test-cluster".to_string();
    let nodegroup_name = "test-nodegroup".to_string();

    let result = delete_nodegroup(&client, cluster_name, nodegroup_name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_describe_eks_cluster() {
    let client = create_client().await;
    
    let cluster_name = "test-cluster".to_string();

    let result = describe_cluster(&client, cluster_name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_describe_nodegroup() {
    let client = create_client().await;
    
    let cluster_name = "test-cluster".to_string();
    let nodegroup_name = "test-nodegroup".to_string();

    let result = describe_nodegroup(&client, cluster_name, nodegroup_name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_eks_cluster() {
    let client = create_client().await;
    
    let cluster_name = "test-cluster";

    let result = delete_cluster(&client, cluster_name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_eks_clusters() {
    let client = create_client().await;
    
    let max_results = Some(10);
    let include = None;

    let result = list_clusters(&client, max_results, include).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_nodegroups() {
    let client = create_client().await;
    
    let cluster_name = "test-cluster".to_string();
    let max_results = Some(10);

    let result = list_nodegroups(&client, cluster_name, max_results).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_tags() {
    let client = create_client().await;
    
    let resource_arn = "arn:aws:eks:us-east-1:123456789012:cluster/test-cluster".to_string();
    let mut tags = HashMap::new();
    tags.insert("key1".to_string(), "value1".to_string());

    let result = update_tags(&client, resource_arn, Some(tags)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_update_config() {
    let client = create_client().await;
    
    let cluster_name = "test-cluster".to_string();
    let resources_vpc_config = Some(VpcConfigRequest::builder().build());
    let logging = Some(Logging::builder().build());
    let access_config = Some(UpdateAccessConfigRequest::builder().build());

    let result = update_config(&client, cluster_name, resources_vpc_config, logging, None, access_config).await;
    assert!(result.is_ok());
}