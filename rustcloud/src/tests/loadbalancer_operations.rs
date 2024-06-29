use crate::aws::aws_apis::network::aws_loadbalancer::*;
use aws_sdk_elasticloadbalancing::{Client, Config};
use aws_sdk_elasticloadbalancing::types::{Listener, Tag};

use aws_sdk_elasticloadbalancing::config::Region;
use aws_sdk_elasticloadbalancing::Error;
use std::collections::HashMap;
use tokio::runtime::Runtime;

fn create_client() -> Client {
    // Replace with your actual AWS credentials and region for real scenarios
    let config = Config::builder()
        .region(Region::new("us-east-1")) // Specify your desired AWS region
        .build();
    Client::from_conf(config)
}

#[tokio::test]
async fn test_add_tags() {
    let client = create_client();
    let tag = Tag::builder().key("Environment".to_string()).value("Production".to_string()).build().unwrap();
    let result = add_tags(&client, "my-load-balancer".to_string(), tag ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create() {
    let client = create_client();
    let result = create(&client, "my-load-balancer".to_string(), None, None, None, None, None, None).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete() {
    let client = create_client();
    let result = delete(&client, "my-load-balancer".to_string()).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_describe() {
    let client = create_client();
    let result = describe(&client, "my-load-balancer".to_string()).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_load_balancers() {
    let client = create_client();
    let result = list_load_balancers(&client, "my-load-balancer".to_string(), "marker".to_string(), 10).await;

    assert!(result.is_ok());
}