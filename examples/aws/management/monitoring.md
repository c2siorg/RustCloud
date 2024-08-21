# rustcloud - AWS CloudWatch

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
use aws_sdk_cloudwatch::Client;

#[tokio::main]
async fn main() {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Now you're ready to use the client to interact with AWS CloudWatch.
}
```

### Delete a CloudWatch Alarm

```rust
use rustcloud::delete_alarm;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let alarm_name = String::from("my-alarm");
    delete_alarm(&client, &alarm_name).await.unwrap();
}
```

### Get Metric Data from CloudWatch

```rust
use rustcloud::get_metric_data;
use aws_sdk_cloudwatch::types::MetricDataQuery;
use aws_sdk_ec2::primitives::DateTime;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let metric_data_queries = Some(vec![MetricDataQuery::builder().id("m1").metric_stat(/* configure your metric stat */).build()]);
    let start_time = Some(DateTime::from_millis(1622499200000));  // Example: Timestamp in milliseconds
    let end_time = Some(DateTime::from_millis(1622585600000));    // Example: Timestamp in milliseconds

    get_metric_data(&client, metric_data_queries, start_time, end_time, None, None, None, None).await.unwrap();
}
```

### List CloudWatch Alarms

```rust
use rustcloud::list_alarms;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    list_alarms(&client).await.unwrap();
}
```

### Create or Update a CloudWatch Alarm

```rust
use rustcloud::put_metric_alarm;
use aws_sdk_cloudwatch::types::{ComparisonOperator, Statistic};

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let alarm_name = String::from("my-alarm");
    let alarm_description = Some(String::from("This is a test alarm"));
    let alarm_actions = Some(vec![String::from("arn:aws:sns:us-east-1:123456789012:my-sns-topic")]);
    let comparison_operator = Some(ComparisonOperator::GreaterThanOrEqualToThreshold);
    let evaluation_periods = Some(1);
    let metric_name = Some(String::from("CPUUtilization"));
    let namespace = Some(String::from("AWS/EC2"));
    let period = Some(60);
    let statistic = Some(Statistic::Average);
    let threshold = Some(80.0);
    let treat_missing_data = Some(String::from("breaching"));

    put_metric_alarm(
        &client,
        &alarm_name,
        alarm_description,
        alarm_actions,
        comparison_operator,
        evaluation_periods,
        metric_name,
        namespace,
        period,
        statistic,
        threshold,
        treat_missing_data
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
