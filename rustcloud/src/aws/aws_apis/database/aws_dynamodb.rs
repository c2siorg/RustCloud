#![allow(clippy::result_large_err)]

use std::collections::HashMap;
use tokio;
use aws_sdk_dynamodb::{operation::{create_table::CreateTableInput, delete_item::DeleteItemInput, delete_table::DeleteTableInput, query::QueryInput, update_item::UpdateItemInput}, types::{AttributeDefinition, AttributeValue, BillingMode, Condition, ConditionalOperator, ExpectedAttributeValue, GlobalSecondaryIndex, KeySchemaElement, LocalSecondaryIndex, OnDemandThroughput, ProvisionedThroughput, ReturnConsumedCapacity, ReturnItemCollectionMetrics, ReturnValue, ReturnValuesOnConditionCheckFailure, Select, SseSpecification, StreamSpecification, TableClass, Tag}, Client, Error};


#[tokio::main]
#[allow(clippy::result_large_err)]
async fn create_table(client : &Client, attribute_definitions:AttributeDefinition,table_name: String,key_schema:KeySchemaElement,local_secondary_indexes: LocalSecondaryIndex,global_secondary_index: GlobalSecondaryIndex,billing_mode: BillingMode,provisioned_throughput :ProvisionedThroughput,stream_specification :StreamSpecification,sse_specification :SseSpecification,tags :Option<Vec<Tag>>,
 table_class: TableClass,deletion_protection_enabled: Option<bool>,resource_policy: Option<String>,on_demand_throughput: Option<OnDemandThroughput>) -> Result<(), Error> {
    let table = client.create_table().attribute_definitions(attribute_definitions).table_name(table_name).key_schema(key_schema).local_secondary_indexes(local_secondary_indexes).global_secondary_indexes(global_secondary_index).billing_mode(billing_mode).provisioned_throughput(provisioned_throughput).stream_specification(stream_specification).sse_specification(sse_specification).set_tags(tags).table_class(table_class).set_deletion_protection_enabled(deletion_protection_enabled).set_resource_policy(resource_policy).set_on_demand_throughput(on_demand_throughput).send().await?;
        

    println!("table created: {:?}", table);

    Ok(())
}


async fn delete_item(client: &Client,   table_name: String, key: Option<HashMap<String, AttributeValue>>, expected: Option<HashMap<String, ExpectedAttributeValue>>, conditional_operator: Option<ConditionalOperator>,  return_values: Option<ReturnValue>, return_consumed_capacity: Option<ReturnConsumedCapacity>, return_item_collection_metrics: Option<ReturnItemCollectionMetrics>, condition_expression: Option<String>, expression_attribute_names: Option<HashMap<String, String>>, expression_attribute_values: Option<HashMap<String, AttributeValue>>,return_values_on_condition_check_failure: Option<ReturnValuesOnConditionCheckFailure>) -> Result<(), Error> {
    let delete_item = client.delete_item().table_name(table_name).set_key(key).set_expected(expected).set_conditional_operator(conditional_operator).set_return_values(return_values).set_return_consumed_capacity(return_consumed_capacity).set_return_item_collection_metrics(return_item_collection_metrics).set_condition_expression(condition_expression).set_expression_attribute_names(expression_attribute_names).set_return_values_on_condition_check_failure(return_values_on_condition_check_failure).set_expression_attribute_values(expression_attribute_values).send().await?;

    println!("item deleted: {:?}", delete_item);


    Ok(())
    
}


async fn delete_table(client: &Client,   table_name: String) -> Result<(), Error> {
    let delete_table = client.delete_table().table_name(table_name).send().await?;

    println!("table deleted: {:?}", delete_table);

    Ok(())
    
}


async fn query(client: &Client, table_name: String, index_name: Option<String>, select: Option<Select>, attributes_to_get: Option<Vec<String>>, limit: Option<i32>, consistent_read: Option<bool>, key_conditions: Option<HashMap<String, Condition>>, query_filter: Option<HashMap<String, Condition>>, conditional_operator: Option<ConditionalOperator>, scan_index_forward: Option<bool>,exclusive_start_key: Option<HashMap<String, AttributeValue>>,return_consumed_capacity: Option<ReturnConsumedCapacity>, projection_expression: Option<String>, filter_expression: Option<String>,key_condition_expression: Option<String>,expression_attribute_names: Option<HashMap<String, String>>,expression_attribute_values: Option<HashMap<String, AttributeValue>>) -> Result<(), Error> {
    let query = client.query().table_name(table_name).set_index_name(index_name).set_select(select).set_attributes_to_get(attributes_to_get).set_limit(limit).set_consistent_read(consistent_read).set_key_conditions(key_conditions).set_query_filter(query_filter).set_conditional_operator(conditional_operator).set_scan_index_forward(scan_index_forward).set_exclusive_start_key(exclusive_start_key).set_return_consumed_capacity(return_consumed_capacity).set_projection_expression(projection_expression).set_filter_expression(filter_expression).set_key_condition_expression(key_condition_expression).set_expression_attribute_names(expression_attribute_names).set_expression_attribute_values(expression_attribute_values).send().await?;
    

    println!("table queried: {:?}", query);

    Ok(())
}


pub struct UpdateItemInput {
    
}

async fn update_item(client: &Client, table_name: String, index_name: Option<String>, select: Option<Select>, attributes_to_get: Option<Vec<String>>, limit: Option<i32>, consistent_read: Option<bool>, key_conditions: Option<HashMap<String, Condition>>, query_filter: Option<HashMap<String, Condition>>, conditional_operator: Option<ConditionalOperator>, scan_index_forward: Option<bool>,exclusive_start_key: Option<HashMap<String, AttributeValue>>,return_consumed_capacity: Option<ReturnConsumedCapacity>, projection_expression: Option<String>, filter_expression: Option<String>,key_condition_expression: Option<String>,expression_attribute_names: Option<HashMap<String, String>>,expression_attribute_values: Option<HashMap<String, AttributeValue>>) -> Result<(), Error> {
    
    let query = client.update_item().table_name(table_name).set_index_name(index_name).set_select(select).set_attributes_to_get(attributes_to_get).set_limit(limit).set_consistent_read(consistent_read).set_key_conditions(key_conditions).set_query_filter(query_filter).set_conditional_operator(conditional_operator).set_scan_index_forward(scan_index_forward).set_exclusive_start_key(exclusive_start_key).set_return_consumed_capacity(return_consumed_capacity).set_projection_expression(projection_expression).set_filter_expression(filter_expression).set_key_condition_expression(key_condition_expression).set_expression_attribute_names(expression_attribute_names).set_expression_attribute_values(expression_attribute_values).send().await?;
    

    println!("table queried: {:?}", query);

    Ok(())
}



