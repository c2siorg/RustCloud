# Azure Cosmos DB — RustCloud Examples

NoSQL database operations via `AzureCosmosDb`. This is the first Azure database
provider in RustCloud, covering Cosmos DB's core document model.

## Setup

```rust
use rustcloud::azure::azure_apis::database::azure_cosmos_db::AzureCosmosDb;

// From environment variables
let client = AzureCosmosDb::new();

// Or explicit config
let client = AzureCosmosDb::with_config(
    "https://myaccount.documents.azure.com",
    "my-bearer-token",
);
```

**Required environment variables:**
```
AZURE_COSMOS_HOST=https://myaccount.documents.azure.com
AZURE_COSMOS_TOKEN=<azure-ad-bearer-token>
```

---

## Databases

```rust
// List databases
let result = client.list_databases().await?;
println!("{}", result["body"]);

// Create a database
client.create_database("my-database").await?;

// Delete a database
client.delete_database("my-database").await?;
```

## Containers (Collections)

```rust
// Create a container with partition key /userId
client.create_container("my-database", "users", "/userId").await?;

// Delete a container
client.delete_container("my-database", "users").await?;
```

## Documents

```rust
use serde_json::json;

// Upsert (create or replace) a document
let doc = json!({
    "id": "user123",
    "userId": "user123",
    "name": "Alice",
    "email": "alice@example.com"
});
client.upsert_document("my-database", "users", &doc).await?;

// Get a document by id and partition key
let result = client.get_document("my-database", "users", "user123", "user123").await?;
println!("{}", result["body"]);

// Query with Cosmos SQL syntax
let result = client.query_documents(
    "my-database",
    "users",
    "SELECT * FROM c WHERE c.name = 'Alice'",
).await?;
println!("{}", result["body"]);
```

---

## Response format

All methods return `Result<serde_json::Value, Box<dyn Error>>` with shape:
```json
{ "status": 200, "body": { ... } }
```
