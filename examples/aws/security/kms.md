# rustcloud - AWS KMS

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
use aws_sdk_kms::Client;

#[tokio::main]
async fn main() {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Now you're ready to use the client to interact with AWS KMS.
}
```

### Create Key

```rust
use rustcloud::create_key;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let policy = String::from("your-key-policy");
    let description = Some(String::from("your-description"));
    let key_usage = Some(KeyUsageType::ENCRYPT_DECRYPT);
    let key_spec = Some(KeySpec::SYMMETRIC_DEFAULT);
    let origin = Some(OriginType::AWS_KMS);
    let custom_key_store_id = Some(String::from("your-custom-key-store-id"));
    let bypass_policy_lockout_safety_check = Some(true);
    let tags = Some(vec![Tag {
        key: String::from("tag-key"),
        value: String::from("tag-value"),
    }]);
    let multi_region = Some(true);
    let xks_key_id = Some(String::from("your-xks-key-id"));

    create_key(
        &client, 
        policy,
        description,
        key_usage,
        key_spec,
        origin,
        custom_key_store_id,
        bypass_policy_lockout_safety_check,
        tags,
        multi_region,
        xks_key_id
    ).await.unwrap();
}
```

### Delete Key

```rust
use rustcloud::delete_key;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let custom_key_store_id = String::from("your-custom-key-store-id");
    
    delete_key(&client, custom_key_store_id).await.unwrap();
}
```

### Describe Key

```rust
use rustcloud::describe_key;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let key_id = String::from("your-key-id");
    let grant_tokens = Some(vec![String::from("your-grant-token")]);

    describe_key(&client, key_id, grant_tokens).await.unwrap();
}
```

### Put Key Policy

```rust
use rustcloud::put_key_policy;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let key_id = String::from("your-key-id");
    let policy_name = String::from("your-policy-name");
    let policy = String::from("your-key-policy");
    let bypass_policy_lockout_safety_check = Some(true);

    put_key_policy(&client, key_id, policy_name, policy, bypass_policy_lockout_safety_check).await.unwrap();
}
```

### Update Key Description

```rust
use rustcloud::update;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let key_id = String::from("your-key-id");
    let description = Some(String::from("your-new-description"));

    update(&client, key_id, description).await.unwrap();
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
