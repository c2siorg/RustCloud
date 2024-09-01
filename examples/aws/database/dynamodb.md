
# rustcloud - AWS DynamoDB

## Configure AWS credentials

First, ensure that your AWS credentials are set up. You can do this by setting environment variables:

```sh
export AWS_ACCESS_KEY_ID= "xxxxxxxxxxxx"
export AWS_SECRET_ACCESS_KEY= "xxxxxxxxxxxx"
```

Alternatively, you can use the AWS credentials file located in your `<HOME>/.aws/credentials`.

## Initialize the library

```rust
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_dynamodb::Client;

#[tokio::main]
async fn main() {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Now you're ready to use the client to interact with AWS DynamoDB.
}
```

### Create a DynamoDB Table

```rust
use rustcloud::create_table;
use aws_sdk_dynamodb::types::{AttributeDefinition, KeySchemaElement, BillingMode, ProvisionedThroughput, TableClass};

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let attribute_definitions = AttributeDefinition::builder().attribute_name("id").attribute_type("S").build();
    let key_schema = KeySchemaElement::builder().attribute_name("id").key_type("HASH").build();
    let billing_mode = BillingMode::Provisioned;
    let provisioned_throughput = ProvisionedThroughput::builder().read_capacity_units(5).write_capacity_units(5).build();
    let table_class = TableClass::Standard;

    create_table(
        &client,
        attribute_definitions,
        String::from("my-table"),
        key_schema,
        None,
        None,
        billing_mode,
        provisioned_throughput,
        None,
        None,
        None,
        table_class,
        None,
        None,
        None,
    ).await.unwrap();
}
```

### Delete an Item from a DynamoDB Table

```rust
use rustcloud::delete_item;
use aws_sdk_dynamodb::types::{AttributeValue, ReturnValue};

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let mut key = HashMap::new();
    key.insert(String::from("id"), AttributeValue::S(String::from("123")));

    delete_item(
        &client,
        String::from("my-table"),
        Some(key),
        None,
        None,
        Some(ReturnValue::AllOld),
        None,
        None,
        None,
        None,
        None
    ).await.unwrap();
}
```

### Delete a DynamoDB Table

```rust
use rustcloud::delete_table;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    delete_table(&client, String::from("my-table")).await.unwrap();
}
```

### Query a DynamoDB Table

```rust
use rustcloud::query;
use aws_sdk_dynamodb::types::Condition;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let mut key_conditions = HashMap::new();
    key_conditions.insert(
        String::from("id"),
        Condition::builder().attribute_value_list(AttributeValue::S(String::from("123"))).comparison_operator("EQ").build()
    );

    query(
        &client,
        String::from("my-table"),
        None,
        None,
        None,
        None,
        None,
        Some(key_conditions),
        None,
        None,
        None,
        None,
        None,
        None,
        None,
        None,
    ).await.unwrap();
}
```

### Utility: Setup Client

You can add this utility function to set up the AWS client:

```rust
async fn setup_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&config)
}
```
## Contributing

If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request. Contributions are welcome!
