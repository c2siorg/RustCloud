# AWS Lambda — RustCloud Examples

Rust wrapper for AWS Lambda serverless compute via `aws-sdk-lambda`.

## Setup

```rust
use aws_config;
use aws_sdk_lambda::{types::Runtime, Client};
use rustcloud::aws::aws_apis::compute::aws_lambda::*;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
}
```

**Required environment variables:**
```
AWS_ACCESS_KEY_ID=<key>
AWS_SECRET_ACCESS_KEY=<secret>
AWS_DEFAULT_REGION=us-east-1
```

---

## List functions

```rust
let names = list_functions(&client).await?;
println!("{:?}", names);
```

## Create a function

```rust
use std::fs;

let zip_bytes = fs::read("function.zip")?;

let arn = create_function(
    &client,
    "my-function",
    Runtime::Nodejs18x,                               // or Python3_11, Java17, etc.
    "arn:aws:iam::123456789012:role/lambda-exec-role",
    "index.handler",
    zip_bytes,
)
.await?;

println!("ARN: {}", arn);
```

## Get function info

```rust
get_function(&client, "my-function").await?;
// prints runtime, handler, state, last modified
```

## Invoke a function

```rust
let payload = serde_json::json!({"name": "RustCloud"})
    .to_string()
    .into_bytes();

let response_bytes = invoke_function(&client, "my-function", Some(payload)).await?;

if let Some(bytes) = response_bytes {
    println!("Response: {}", String::from_utf8_lossy(&bytes));
}
```

## Update function code

```rust
use std::fs;

let new_zip = fs::read("function_v2.zip")?;
update_function_code(&client, "my-function", new_zip).await?;
```

## Delete a function

```rust
delete_function(&client, "my-function").await?;
```

---

## Supported runtimes

| Constant | Runtime |
|---|---|
| `Runtime::Nodejs18x` | Node.js 18 |
| `Runtime::Python3_11` | Python 3.11 |
| `Runtime::Java17` | Java 17 |
| `Runtime::Dotnet8` | .NET 8 |
| `Runtime::ProvidedAl2023` | Custom runtime (AL2023) |
