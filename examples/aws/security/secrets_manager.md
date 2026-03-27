# rustcloud — AWS Secrets Manager

AWS Secrets Manager stores, rotates, and retrieves application secrets —
database credentials, API keys, OAuth tokens. RustCloud wraps it with the
same flat-function pattern used across all AWS services.

## Configure AWS Credentials

```sh
export AWS_ACCESS_KEY_ID="your-access-key"
export AWS_SECRET_ACCESS_KEY="your-secret-key"
export AWS_DEFAULT_REGION="us-east-1"
```

## Initialize the Client

```rust
use aws_sdk_secretsmanager::Client;

#[tokio::main]
async fn main() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
}
```

## Operations

### Create a Secret

Store a plaintext string or a JSON map:

```rust
use rustcloud::aws::aws_apis::security::aws_secrets_manager;

// Plaintext
let arn = aws_secrets_manager::create_secret(&client, "prod/api-key", "sk-abc123").await?;

// JSON map (recommended for multiple fields)
let arn = aws_secrets_manager::create_secret(
    &client,
    "prod/db-credentials",
    r#"{"host":"db.example.com","username":"app","password":"s3cr3t"}"#,
).await?;

println!("ARN: {}", arn);
```

### Get a Secret

```rust
let value = aws_secrets_manager::get_secret(&client, "prod/db-credentials").await?;
// value is the raw secret string — parse JSON if needed
let parsed: serde_json::Value = serde_json::from_str(&value)?;
println!("Host: {}", parsed["host"]);
```

### Update a Secret

```rust
aws_secrets_manager::update_secret(
    &client,
    "prod/db-credentials",
    r#"{"host":"db.example.com","username":"app","password":"n3wpassword"}"#,
).await?;
```

### List Secrets

```rust
let names = aws_secrets_manager::list_secrets(&client).await?;
for name in names {
    println!("{}", name);
}
```

### Delete a Secret

```rust
// Schedule deletion with a 7-day recovery window (minimum)
aws_secrets_manager::delete_secret(&client, "prod/api-key", Some(7)).await?;

// Use AWS default (30-day recovery window)
aws_secrets_manager::delete_secret(&client, "prod/api-key", None).await?;
```

## Naming conventions

Use path-style names (`service/env/name`) to group and control access via IAM:

```
prod/database/primary-password
prod/api/stripe-key
staging/database/primary-password
```

## IAM + KMS + Secrets Manager

The three AWS security services work together:

| Service              | Purpose                                   |
|----------------------|-------------------------------------------|
| IAM                  | Controls *who* can access secrets         |
| KMS                  | Encrypts secret values at rest            |
| Secrets Manager      | Stores and retrieves the actual values    |

## Supported operations

| Function        | Description                                      |
|-----------------|--------------------------------------------------|
| `create_secret` | Store a new secret, returns ARN                  |
| `get_secret`    | Retrieve current secret string by name or ARN    |
| `update_secret` | Overwrite secret value, returns ARN              |
| `delete_secret` | Schedule deletion with configurable grace period |
| `list_secrets`  | List all secret names in the account             |

## References

- [AWS Secrets Manager Developer Guide](https://docs.aws.amazon.com/secretsmanager/latest/userguide/)
- [aws-sdk-secretsmanager crate](https://docs.rs/aws-sdk-secretsmanager/latest/aws_sdk_secretsmanager/)
- [IAM example](iam.md)
- [KMS example](kms.md)
