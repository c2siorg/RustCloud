use aws_sdk_dynamodb::config::{BehaviorVersion, Credentials, Region, SharedCredentialsProvider};
use aws_sdk_dynamodb::types::{
    AttributeDefinition, AttributeValue, BillingMode, ComparisonOperator, Condition,
    KeySchemaElement, KeyType, ProvisionedThroughput, PutRequest, ReturnConsumedCapacity,
    ReturnValue, ScalarAttributeType, Select, TableClass, WriteRequest,
};
use aws_sdk_dynamodb::{Client, Config};
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;
use std::collections::HashMap;

use crate::aws::aws_apis::database::aws_dynamodb::*;

fn mock_client(response_body: &str) -> Client {
    let request = http::Request::builder()
        .method("POST")
        .uri("https://dynamodb.us-east-1.amazonaws.com/")
        .body(SdkBody::empty())
        .unwrap();
    let response = http::Response::builder()
        .status(200)
        .header("content-type", "application/x-amz-json-1.0")
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
async fn test_create_table() {
    let client = mock_client(
        r#"{"TableDescription":{"TableName":"test-table","TableStatus":"CREATING"}}"#,
    );

    let attribute_definitions = AttributeDefinition::builder()
        .attribute_name("id")
        .attribute_type(ScalarAttributeType::S)
        .build()
        .unwrap();
    let key_schema = KeySchemaElement::builder()
        .attribute_name("id")
        .key_type(KeyType::Hash)
        .build()
        .unwrap();
    let provisioned_throughput = ProvisionedThroughput::builder()
        .read_capacity_units(5)
        .write_capacity_units(5)
        .build()
        .unwrap();

    let result = create_table(
        &client,
        attribute_definitions,
        "test-table".to_string(),
        key_schema,
        None,
        None,
        BillingMode::Provisioned,
        provisioned_throughput,
        None,
        None,
        None,
        TableClass::Standard,
        Some(false),
        None,
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_item() {
    let client = mock_client("{}");

    let mut key = HashMap::new();
    key.insert("id".to_string(), AttributeValue::S("test-id".to_string()));

    let result = delete_item(
        &client,
        "test-table".to_string(),
        Some(key),
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
async fn test_delete_table() {
    let client = mock_client(
        r#"{"TableDescription":{"TableName":"test-table","TableStatus":"DELETING"}}"#,
    );

    let result = delete_table(&client, "test-table".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_query() {
    let client = mock_client(r#"{"Items":[],"Count":0,"ScannedCount":0}"#);

    let mut key_conditions = HashMap::new();
    let condition = Condition::builder()
        .comparison_operator(ComparisonOperator::Eq)
        .attribute_value_list(AttributeValue::S("test-id".to_string()))
        .build()
        .expect("Failed to build condition");
    key_conditions.insert("id".to_string(), condition);

    let result = query(
        &client,
        "test-table".to_string(),
        None,
        Some(Select::AllAttributes),
        None,
        Some(10),
        Some(false),
        Some(key_conditions),
        None,
        None,
        Some(true),
        None,
        Some(ReturnConsumedCapacity::Total),
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
async fn test_get_item() {
    let client = mock_client(
        r#"{"Item":{"id":{"S":"test-id"}}}"#,
    );

    let mut key = HashMap::new();
    key.insert("id".to_string(), AttributeValue::S("test-id".to_string()));

    let result = get_item(
        &client,
        "test-table".to_string(),
        key,
        None,
        Some(false),
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_put_item() {
    let client = mock_client("{}");

    let mut item = HashMap::new();
    item.insert("id".to_string(), AttributeValue::S("test-id".to_string()));
    item.insert(
        "name".to_string(),
        AttributeValue::S("test-name".to_string()),
    );

    let result = put_item(
        &client,
        "test-table".to_string(),
        item,
        None,
        None,
        Some(ReturnValue::None),
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
async fn test_update_item() {
    let client = mock_client("{}");

    let mut key = HashMap::new();
    key.insert("id".to_string(), AttributeValue::S("test-id".to_string()));

    let result = update_item(
        &client,
        "test-table".to_string(),
        key,
        None,
        None,
        None,
        Some(ReturnValue::AllNew),
        None,
        None,
        Some("SET #n = :val".to_string()),
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_scan() {
    let client = mock_client(r#"{"Items":[],"Count":0,"ScannedCount":0}"#);

    let result = scan(
        &client,
        "test-table".to_string(),
        None,
        None,
        Some(100),
        Some(false),
        None,
        None,
        Some(ReturnConsumedCapacity::Total),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        Some(Select::AllAttributes),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_batch_write_item() {
    let client = mock_client(r#"{"UnprocessedItems":{}}"#);

    let put_request = PutRequest::builder()
        .item("id", AttributeValue::S("batch-id-1".to_string()))
        .item("name", AttributeValue::S("batch-name-1".to_string()))
        .build()
        .unwrap();
    let write_request = WriteRequest::builder().put_request(put_request).build();

    let mut request_items = HashMap::new();
    request_items.insert("test-table".to_string(), vec![write_request]);

    let result = batch_write_item(&client, request_items, None, None).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_delete_table_live() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let result = delete_table(&client, "test-table".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}
