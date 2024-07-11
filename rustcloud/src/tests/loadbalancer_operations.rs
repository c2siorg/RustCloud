use crate::aws::aws_apis::network::aws_loadbalancer::*;
use aws_sdk_elasticloadbalancing::{Client, Config};
use aws_sdk_elasticloadbalancing::types::{Tag};
use aws_sdk_elasticloadbalancing::config::Region;


async fn create_client() -> Client {
    let config =  aws_config::load_from_env().await;
    let client =  Client::new(&config);
    return client;
}

#[tokio::test]
async fn test_add_tags_to_loadbalancer() {
    let client = create_client().await;
    let tag = Tag::builder().key("Environment".to_string()).value("Production".to_string()).build().unwrap();
    let result = add_tags(&client, "my-load-balancer".to_string(), tag ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_loadbalancer() {
    let client = create_client().await;
    let result = create(&client, "my-load-balancer".to_string(), None, None, None, None, None, None).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_loadbalancer() {
    let client = create_client().await;
    let result = delete(&client, "my-load-balancer".to_string()).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_describe_loadbalancer() {
    let client = create_client().await;
    let result = describe(&client, "my-load-balancer".to_string()).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_load_balancers() {
    let client = create_client().await;
    let result = list_load_balancers(&client, "my-load-balancer".to_string(), "marker".to_string(), 10).await;

    assert!(result.is_ok());
}