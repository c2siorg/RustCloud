# rustcloud - AWS Route 53

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
use aws_sdk_route53::Client;

#[tokio::main]
async fn main() {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Now you're ready to use the client to interact with AWS Route 53.
}
```

### Change Resource Record Sets

```rust
use rustcloud::change_record_sets;
use aws_sdk_route53::types::ChangeBatch;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let hosted_zone_id = String::from("your-hosted-zone-id");
    let change_batch = ChangeBatch::builder()
        // Configure your ChangeBatch here
        .build();
        
    change_record_sets(&client, hosted_zone_id, change_batch).await.unwrap();
}
```

### Create a Hosted Zone

```rust
use rustcloud::create_zone;
use aws_sdk_route53::types::{HostedZoneConfig, Vpc};

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let name = String::from("example.com");
    let vpc = Vpc::builder()
        // Configure your VPC here
        .build();
    let caller_reference = String::from("unique-caller-reference");
    let hosted_zone_config = Some(HostedZoneConfig::builder()
        // Configure your HostedZoneConfig here
        .build());
    let delegation_set_id = Some(String::from("your-delegation-set-id"));

    create_zone(
        &client,
        name,
        vpc,
        caller_reference,
        hosted_zone_config,
        delegation_set_id
    ).await.unwrap();
}
```

### Delete a Hosted Zone

```rust
use rustcloud::delete_zone;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let id = String::from("your-hosted-zone-id");
    delete_zone(&client, id).await.unwrap();
}
```

### List Hosted Zones

```rust
use rustcloud::list_zones;
use aws_sdk_route53::types::HostedZoneType;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let marker = None; // Optionally, specify a marker for pagination
    let max_items = Some(10);
    let delegation_set_id = None; // Optionally, specify a delegation set ID
    let hosted_zone_type = Some(HostedZoneType::PrivateHostedZone);

    list_zones(
        &client,
        marker,
        max_items,
        delegation_set_id,
        hosted_zone_type
    ).await.unwrap();
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
