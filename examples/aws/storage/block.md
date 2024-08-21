# rustcloud - AWS EC2 Volume Operations

## Configure AWS credentials

Ensure that your AWS credentials are configured by setting the environment variables:

```sh
export AWS_ACCESS_KEY_ID= "xxxxxxxxxxxx"
export AWS_SECRET_ACCESS_KEY= "xxxxxxxxxxxx"
```

Alternatively, you can use the AWS credentials file located in your `<HOME>/.aws/credentials`.

## Initialize the library

```rust
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::Client;

#[tokio::main]
async fn main() {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Now you're ready to use the client to interact with AWS EC2.
}
```

### Create Volume

```rust
use rustcloud::create;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let availability_zone = String::from("us-east-1a");
    let size = Some(100);
    let volume_type = Some(aws_sdk_ec2::types::VolumeType::Gp2);
    let iops = Some(1000);
    let encrypted = Some(true);
    let kms_key_id = Some(String::from("your-kms-key-id"));

    create(
        &client,
        availability_zone,
        size,
        volume_type,
        iops,
        encrypted,
        kms_key_id
    ).await.unwrap();
}
```

### Delete Volume

```rust
use rustcloud::delete;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let volume_id = String::from("vol-1234567890abcdef0");

    delete(&client, volume_id).await.unwrap();
}
```

### Describe Volume

```rust
use rustcloud::describe;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let volume_id = String::from("vol-1234567890abcdef0");
    let attribute = aws_sdk_ec2::types::VolumeAttributeName::Size;

    describe(&client, volume_id, attribute).await.unwrap();
}
```

### List Volumes

```rust
use rustcloud::list;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let volume_ids = Some(vec![String::from("vol-1234567890abcdef0")]);
    let filters = Some(vec![
        aws_sdk_ec2::types::Filter {
            name: String::from("volume-type"),
            values: vec![String::from("gp2")],
        }
    ]);
    let max_results = Some(10);
    let next_token = Some(String::from("your-next-token"));

    list(&client, volume_ids, filters, max_results, next_token).await.unwrap();
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
