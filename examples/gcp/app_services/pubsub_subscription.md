# GCP Pub/Sub Subscriptions — RustCloud Examples

Manage Pub/Sub subscriptions via `GcpPubSubSubscription`.
The existing `gcp_notification_service` handles **topics**; this module handles **subscriptions**.

## Setup

```rust
use rustcloud::gcp::gcp_apis::app_services::gcp_pubsub_subscription::GcpPubSubSubscription;
use std::collections::HashMap;

let client = GcpPubSubSubscription::new();
```

**Required:** `GOOGLE_APPLICATION_CREDENTIALS=/path/to/service-account.json`

---

## Create a subscription

```rust
let mut req = HashMap::new();
req.insert("Project".to_string(), "my-project".to_string());
req.insert("Topic".to_string(), "my-topic".to_string());
req.insert("Subscription".to_string(), "my-sub".to_string());
req.insert("AckDeadlineSeconds".to_string(), "30".to_string());

client.create_subscription(req).await?;
```

## List subscriptions

```rust
let mut req = HashMap::new();
req.insert("Project".to_string(), "my-project".to_string());
req.insert("PageSize".to_string(), "20".to_string());

let result = client.list_subscriptions(req).await?;
println!("{}", result["body"]);
```

## Pull messages (synchronous pull)

```rust
let mut req = HashMap::new();
req.insert("Project".to_string(), "my-project".to_string());
req.insert("Subscription".to_string(), "my-sub".to_string());
req.insert("MaxMessages".to_string(), "10".to_string());

let result = client.pull_messages(req).await?;
println!("{}", result["body"]); // JSON with receivedMessages[] array
```

## Acknowledge messages

After processing, ack the messages to remove them from the queue:

```rust
let ack_ids = vec![
    "projects/my-project/subscriptions/my-sub:1".to_string(),
    "projects/my-project/subscriptions/my-sub:2".to_string(),
];

client.acknowledge_messages("my-project", "my-sub", ack_ids).await?;
```

## Switch to push delivery

```rust
client.modify_push_config(
    "my-project",
    "my-sub",
    Some("https://my-service.example.com/push"),
).await?;
```

## Revert to pull delivery (clear push endpoint)

```rust
client.modify_push_config("my-project", "my-sub", None).await?;
```

## Delete a subscription

```rust
let mut req = HashMap::new();
req.insert("Project".to_string(), "my-project".to_string());
req.insert("Subscription".to_string(), "my-sub".to_string());

client.delete_subscription(req).await?;
```
