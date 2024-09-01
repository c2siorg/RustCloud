# rustcloud - AWS IAM

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
use aws_sdk_iam::Client;

#[tokio::main]
async fn main() {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Now you're ready to use the client to interact with AWS IAM.
}
```

### Attach Group Policy

```rust
use rustcloud::attach_group_policy;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let group_name = String::from("your-group-name");
    let policy_arn = String::from("your-policy-arn");
    
    attach_group_policy(&client, group_name, policy_arn).await.unwrap();
}
```

### Create IAM Group

```rust
use rustcloud::create_group;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let path = String::from("/your/path/");
    let group_name = String::from("your-group-name");

    create_group(&client, path, group_name).await.unwrap();
}
```

### Delete IAM Group

```rust
use rustcloud::delete_group;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let group_name = String::from("your-group-name");
    
    delete_group(&client, group_name).await.unwrap();
}
```

### Detach Group Policy

```rust
use rustcloud::detach_group_policy;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let group_name = String::from("your-group-name");
    let policy_arn = String::from("your-policy-arn");
    
    detach_group_policy(&client, group_name, policy_arn).await.unwrap();
}
```

### Describe IAM Group

```rust
use rustcloud::describe;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let group_name = String::from("your-group-name");
    let marker = None; // Optionally, specify a marker for pagination
    let max_items = Some(10);

    describe(&client, group_name, marker, max_items).await.unwrap();
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
