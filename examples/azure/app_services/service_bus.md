# Azure Service Bus — RustCloud Examples

Enterprise messaging operations via `AzureServiceBus`. This is the first Azure
app_services provider in RustCloud, adding queue-based messaging equivalent to AWS SQS.

## Setup

```rust
use rustcloud::azure::azure_apis::app_services::azure_service_bus::AzureServiceBus;

// From environment variables
let client = AzureServiceBus::new();

// Or explicit config
let client = AzureServiceBus::with_config("my-namespace", "my-bearer-token");
```

**Required environment variables:**
```
AZURE_SERVICEBUS_NAMESPACE=my-namespace
AZURE_SERVICEBUS_TOKEN=<azure-ad-bearer-token>
```

---

## Queue management

```rust
// Create a queue (1 GB capacity, 1-minute lock duration)
client.create_queue("orders").await?;

// List all queues in the namespace
let result = client.list_queues().await?;
println!("{}", result["body"]);

// Delete a queue
client.delete_queue("orders").await?;
```

## Send a message

```rust
use serde_json::json;

let payload = json!({
    "orderId": "ORD-001",
    "amount": 99.99,
    "currency": "USD"
})
.to_string();

client.send_message("orders", &payload).await?;
```

## Receive and delete (at-most-once)

```rust
let result = client.receive_message("orders").await?;
println!("Body: {}", result["body"]);
println!("Broker props: {}", result["broker_properties"]);
```

## Peek-lock then complete (at-least-once)

```rust
// 1. Lock the message for up to 30 seconds
let locked = client.peek_lock_message("orders", 30).await?;

// Parse lock token and sequence number from broker properties
// BrokerProperties header contains: {"SequenceNumber":1,"LockToken":"abc-..."}

// 2. Process the message...

// 3. Complete (delete) the message after successful processing
client.complete_message("orders", "1", "abc-lock-token").await?;
```

---

## Response format

All methods return `Result<serde_json::Value, Box<dyn Error>>` with shape:
```json
{ "status": 201, "body": "..." }
```
Messaging responses include `"broker_properties"` with sequence number and lock token.
