use crate::aws::aws_apis::compute::aws_ecs::*;

use aws_sdk_ecs::{Client, Config};
use aws_sdk_ecs::config::Region;
use aws_sdk_ecs::types::{ClusterConfiguration, ClusterSetting, Tag, ClusterSettingName};


#[tokio::test]
async fn test_create_cluster() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let name = "test-cluster".to_string();
    let tags = Some(vec![
        Tag::builder().key("Environment").value("Test").build(),
        Tag::builder().key("Name").value("Test Cluster").build(),
    ]);
    let settings = ClusterSetting::builder()
    .name(ClusterSettingName::from("my-cluster-setting"))
    .value("some-value")
    .build(); 
    let configuration = ClusterConfiguration::builder().build(); 
    let capacity_providers = None; 

    let result = create_cluster(&client, &name, tags, settings, configuration, capacity_providers).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_cluster() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let name = "test-cluster".to_string(); 

    let result = delete_cluster(&client, &name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_describe_cluster() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let clusters = None; 
    let include = None; 

    let result = describe_cluster(&client, clusters, include).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_show_clusters() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let max_results = Some(10); 

    let result = show_clusters(&client, max_results).await;
    assert!(result.is_ok());
}

