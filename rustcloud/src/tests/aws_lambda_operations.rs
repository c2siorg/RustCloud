use crate::aws::aws_apis::compute::aws_lambda::*;
use aws_sdk_lambda::{types::Runtime, Client};

async fn create_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}

#[tokio::test]
async fn test_list_functions() {
    let client = create_client().await;
    let result = list_functions(&client).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_function() {
    let client = create_client().await;
    let result = create_function(
        &client,
        "rustcloud-test-fn",
        Runtime::Nodejs18x,
        "arn:aws:iam::123456789012:role/lambda-role",
        "index.handler",
        vec![80, 75, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    )
    .await;
    println!("create_function result: {:?}", result);
}

#[tokio::test]
async fn test_get_function() {
    let client = create_client().await;
    let result = get_function(&client, "rustcloud-test-fn").await;
    println!("get_function result: {:?}", result);
}

#[tokio::test]
async fn test_invoke_function() {
    let client = create_client().await;
    let payload = serde_json::json!({"key": "value"})
        .to_string()
        .into_bytes();
    let result = invoke_function(&client, "rustcloud-test-fn", Some(payload)).await;
    println!("invoke_function result: {:?}", result);
}

#[tokio::test]
async fn test_update_function_code() {
    let client = create_client().await;
    let result = update_function_code(
        &client,
        "rustcloud-test-fn",
        vec![80, 75, 5, 6, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
    )
    .await;
    println!("update_function_code result: {:?}", result);
}

#[tokio::test]
async fn test_delete_function() {
    let client = create_client().await;
    let result = delete_function(&client, "rustcloud-test-fn").await;
    println!("delete_function result: {:?}", result);
}
