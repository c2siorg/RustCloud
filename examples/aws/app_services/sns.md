# rustcloud - AWS SNS

AWS Simple Notification Service (SNS) is a managed pub/sub messaging service. RustCloud exposes topic management, message publishing, and subscription management through the `aws_sdk_sns` client.

## Configure AWS Credentials

Ensure your AWS credentials are available via environment variables or `~/.aws/credentials`:

```sh
export AWS_ACCESS_KEY_ID="your-access-key"
export AWS_SECRET_ACCESS_KEY="your-secret-key"
export AWS_DEFAULT_REGION="us-east-1"
```

Or configure the shared credentials file:

```
[default]
aws_access_key_id = your-access-key
aws_secret_access_key = your-secret-key
```

## Initialize the Client

```rust
use aws_sdk_sns::Client;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    // Ready to use the client.
}
```

## Operations

### Create a Topic

```rust
use rustcloud::aws::aws_apis::app_services::aws_sns::create_topic;
use aws_sdk_sns::Client;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    create_topic(&client, "my-topic").await.unwrap();
}
```

### Delete a Topic

```rust
use rustcloud::aws::aws_apis::app_services::aws_sns::delete_topic;
use aws_sdk_sns::Client;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let topic_arn = "arn:aws:sns:us-east-1:123456789012:my-topic";
    delete_topic(&client, topic_arn).await.unwrap();
}
```

### List Topics

```rust
use rustcloud::aws::aws_apis::app_services::aws_sns::list_topics;
use aws_sdk_sns::Client;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    list_topics(&client).await.unwrap();
}
```

### Publish a Message

```rust
use rustcloud::aws::aws_apis::app_services::aws_sns::publish;
use aws_sdk_sns::Client;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let topic_arn = "arn:aws:sns:us-east-1:123456789012:my-topic";
    publish(
        &client,
        topic_arn,
        "Hello from RustCloud",
        Some("Alert".to_string()),
    )
    .await
    .unwrap();
}
```

### Subscribe an Endpoint

```rust
use rustcloud::aws::aws_apis::app_services::aws_sns::subscribe;
use aws_sdk_sns::Client;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let topic_arn = "arn:aws:sns:us-east-1:123456789012:my-topic";
    // protocol can be: "email", "sqs", "lambda", "https", "http", "sms"
    subscribe(&client, topic_arn, "email", "you@example.com").await.unwrap();
}
```

### Unsubscribe

```rust
use rustcloud::aws::aws_apis::app_services::aws_sns::unsubscribe;
use aws_sdk_sns::Client;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let subscription_arn = "arn:aws:sns:us-east-1:123456789012:my-topic:abc123";
    unsubscribe(&client, subscription_arn).await.unwrap();
}
```

## Contributing

If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request. Contributions are welcome!
