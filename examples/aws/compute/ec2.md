


# rustcloud - AWS

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
use aws_sdk_ec2::Client;

#[tokio::main]
async fn main() {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Now you're ready to use the client to interact with AWS services.
}
```

### Create an Instance

```rust
use rustcloud::create_instance;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let ami_id = "ami-ccf405a5"; // Replace with your AMI ID
    create_instance(&client, ami_id).await.unwrap();
}
```

### Stop an Instance

```rust
use rustcloud::stop_instance;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let instance_id = "i-06d518ba15b68685c"; // Replace with your instance ID
    stop_instance(&client, instance_id).await.unwrap();
}
```

### Start an Instance

```rust
use rustcloud::start_instance;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let instance_id = "i-0174bd6f54178e89b"; // Replace with your instance ID
    start_instance(&client, instance_id).await.unwrap();
}
```

### Reboot an Instance

```rust
use rustcloud::reboot_instance;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let instance_id = "i-037a9fae81c33ac30"; // Replace with your instance ID
    reboot_instance(&client, instance_id).await.unwrap();
}
```

### Enable Monitoring

```rust
use rustcloud::enable_monitoring;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let instance_id = "i-037a9fae81c33ac30"; // Replace with your instance ID
    enable_monitoring(&client, instance_id).await.unwrap();
}
```

### Show Instance State

```rust
use rustcloud::show_state;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let instance_ids = vec![String::from("i-037a9fae81c33ac30")]; // Replace with your instance ID
    show_state(&client, Some(instance_ids)).await.unwrap();
}
```

### Show All Events Across Regions

```rust
use rustcloud::show_all_events;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    show_all_events(&client).await.unwrap();
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
