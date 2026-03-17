# rustcloud — AWS SQS

AWS Simple Queue Service (SQS) is a fully managed message queue for
decoupling distributed systems. RustCloud exposes queue lifecycle management
and message operations through the `aws_sdk_sqs` client.

## Configure AWS Credentials

```sh
export AWS_ACCESS_KEY_ID="your-access-key"
export AWS_SECRET_ACCESS_KEY="your-secret-key"
export AWS_DEFAULT_REGION="us-east-1"
```

## Initialize the Client

```rust
use aws_sdk_sqs::Client;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
}
```

## Operations

### Create a Queue

```rust
use rustcloud::aws::aws_apis::app_services::aws_sqs;

let queue_url = aws_sqs::create_queue(&client, "my-queue").await?;
println!("Queue URL: {}", queue_url);
```

### List Queues

```rust
// List all queues
let urls = aws_sqs::list_queues(&client, None).await?;

// Filter by prefix
let urls = aws_sqs::list_queues(&client, Some("my-".to_string())).await?;
```

### Get Queue URL

```rust
let url = aws_sqs::get_queue_url(&client, "my-queue").await?;
```

### Send a Message

```rust
// Send immediately
let msg_id = aws_sqs::send_message(&client, &queue_url, "hello world", None).await?;

// Send with a 30-second visibility delay
let msg_id = aws_sqs::send_message(&client, &queue_url, "delayed message", Some(30)).await?;
```

### Receive Messages

```rust
// Receive up to 5 messages with 20-second long polling
let messages = aws_sqs::receive_messages(&client, &queue_url, Some(5), Some(20)).await?;

for msg in &messages {
    println!("Body: {}", msg.body().unwrap_or_default());
    println!("Handle: {}", msg.receipt_handle().unwrap_or_default());
}
```

### Delete a Message (Acknowledge)

After processing a message, delete it to prevent redelivery:

```rust
for msg in messages {
    // process msg ...
    if let Some(handle) = msg.receipt_handle() {
        aws_sqs::delete_message(&client, &queue_url, handle).await?;
    }
}
```

### Delete a Queue

```rust
aws_sqs::delete_queue(&client, &queue_url).await?;
```

## Full example — producer / consumer

```rust
use aws_sdk_sqs::Client;
use rustcloud::aws::aws_apis::app_services::aws_sqs;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    // Create queue
    let url = aws_sqs::create_queue(&client, "orders").await?;

    // Producer: enqueue three orders
    for i in 1..=3 {
        aws_sqs::send_message(&client, &url, &format!("order-{}", i), None).await?;
    }

    // Consumer: poll and process
    let messages = aws_sqs::receive_messages(&client, &url, Some(10), Some(5)).await?;
    for msg in messages {
        println!("Processing: {}", msg.body().unwrap_or_default());
        if let Some(handle) = msg.receipt_handle() {
            aws_sqs::delete_message(&client, &url, handle).await?;
        }
    }

    // Cleanup
    aws_sqs::delete_queue(&client, &url).await?;
    Ok(())
}
```

## SNS + SQS fan-out pattern

Combine SNS (pub/sub broadcast) with SQS (reliable queue consumption):

```
                  ┌─────────────────┐
  Publisher ───▶  │   SNS Topic     │
                  └────────┬────────┘
                           │ fan-out
              ┌────────────┼────────────┐
              ▼            ▼            ▼
         SQS Queue    SQS Queue    SQS Queue
         (service A)  (service B)  (service C)
```

## Supported operations

| Function           | Description                              |
|--------------------|------------------------------------------|
| `create_queue`     | Create queue, returns URL                |
| `delete_queue`     | Delete queue and all its messages        |
| `list_queues`      | List URLs with optional prefix filter    |
| `get_queue_url`    | Look up URL by queue name                |
| `send_message`     | Enqueue message with optional delay      |
| `receive_messages` | Long-poll receive, up to 10 per call     |
| `delete_message`   | Acknowledge message by receipt handle    |

## References

- [AWS SQS Developer Guide](https://docs.aws.amazon.com/AWSSimpleQueueService/latest/SQSDeveloperGuide/)
- [aws-sdk-sqs crate](https://docs.rs/aws-sdk-sqs/latest/aws_sdk_sqs/)
- [SNS example](sns.md) — pub/sub broadcasting
