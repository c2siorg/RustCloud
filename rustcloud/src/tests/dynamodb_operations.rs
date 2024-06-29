use crate::aws::aws_apis::database::aws_dynamodb::*;
use aws_sdk_dynamodb::config::Region;
use aws_sdk_dynamodb::{Client, Config};
use aws_sdk_dynamodb::types::{AttributeDefinition, AttributeValue, BillingMode, ComparisonOperator, Condition, KeySchemaElement, KeyType, ProvisionedThroughput, ReturnConsumedCapacity, ScalarAttributeType, Select, SseSpecification, StreamSpecification, TableClass,LocalSecondaryIndex, GlobalSecondaryIndex
};
use std::collections::HashMap;

#[tokio::test]
async fn test_create_table() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);
    
    let table_name = "test-table".to_string();
    let attribute_definitions = AttributeDefinition::builder()
        .attribute_name("id")
        .attribute_type(ScalarAttributeType::S)
        .build().unwrap();
    let key_type = KeyType::Hash;
    let key_schema = KeySchemaElement::builder()
        .attribute_name("id")
        .key_type(key_type)
        .build().unwrap();
    let provisioned_throughput = ProvisionedThroughput::builder()
        .read_capacity_units(5)
        .write_capacity_units(5)
        .build().unwrap();

    let result = create_table(
        &client,
        attribute_definitions,
        table_name,
        key_schema,
        LocalSecondaryIndex::builder().build().unwrap(),  // LocalSecondaryIndex
        GlobalSecondaryIndex::builder().build().unwrap(),  // GlobalSecondaryIndex
        BillingMode::Provisioned,
        provisioned_throughput,
        StreamSpecification::builder().build().unwrap(),
        SseSpecification::builder().build(),
        None,  // tags
        TableClass::Standard,
        Some(false),  // deletion_protection_enabled
        None,  // resource_policy
        None,  // on_demand_throughput
    );
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_item() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);
    
    let table_name = "test-table".to_string();
    let mut key = HashMap::new();
    key.insert("id".to_string(), AttributeValue::S("test-id".to_string()));

    let result = delete_item(
        &client,
        table_name,
        Some(key),
        None,  // expected
        None,  // conditional_operator
        None,  // return_values
        None,  // return_consumed_capacity
        None,  // return_item_collection_metrics
        None,  // condition_expression
        None,  // expression_attribute_names
        None,  // expression_attribute_values
        None,  // return_values_on_condition_check_failure
    ).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_table() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);
    
    let table_name = "test-table".to_string();

    let result = delete_table(&client, table_name).await;
    
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_query() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);
    
    let table_name = "test-table".to_string();
    let mut key_conditions = HashMap::new();
    
    let condition = Condition::builder()
        .comparison_operator(ComparisonOperator::Eq)
        .attribute_value_list(AttributeValue::S("test-id".to_string()))
        .build()
        .expect("Failed to build condition");

    key_conditions.insert("id".to_string(), condition);

    let result = query(
        &client,
        table_name,
        None,  // index_name
        Some(Select::AllAttributes),
        None,  // attributes_to_get
        Some(10),  // limit
        Some(false),  // consistent_read
        Some(key_conditions),
        None,  // query_filter
        None,  // conditional_operator
        Some(true),  // scan_index_forward
        None,  // exclusive_start_key
        Some(ReturnConsumedCapacity::Total),
        None,  // projection_expression
        None,  // filter_expression
        None,  // key_condition_expression
        None,  // expression_attribute_names
        None,  // expression_attribute_values
    ).await;
    
    assert!(result.is_ok());
}