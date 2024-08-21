# rustcloud - AWS Elastic Load Balancing

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
use aws_sdk_elasticloadbalancing::Client;

#[tokio::main]
async fn main() {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Now you're ready to use the client to interact with AWS Elastic Load Balancing.
}
```

### Add Tags to Load Balancer

```rust
use rustcloud::add_tags;
use aws_sdk_elasticloadbalancing::types::Tag;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let load_balancer_name = String::from("your-load-balancer-name");
    let tags = Tag::builder()
        // Configure your Tag here
        .build();
        
    add_tags(&client, load_balancer_name, tags).await.unwrap();
}
```

### Create Load Balancer

```rust
use rustcloud::create;
use aws_sdk_elasticloadbalancing::types::{Listener, Tag};

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let load_balancer_name = String::from("your-load-balancer-name");
    let listeners = Some(vec![
        // Configure your Listeners here
    ]);
    let availability_zones = Some(vec![
        // Configure your Availability Zones here
    ]);
    let subnets = Some(vec![
        // Configure your Subnets here
    ]);
    let security_groups = Some(vec![
        // Configure your Security Groups here
    ]);
    let scheme = Some(String::from("internet-facing"));
    let tags = Some(vec![
        // Configure your Tags here
    ]);

    create(
        &client,
        load_balancer_name,
        listeners,
        availability_zones,
        subnets,
        security_groups,
        scheme,
        tags
    ).await.unwrap();
}
```

### Delete Load Balancer

```rust
use rustcloud::delete;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let load_balancer_name = String::from("your-load-balancer-name");
    delete(&client, load_balancer_name).await.unwrap();
}
```

### Describe Load Balancer

```rust
use rustcloud::describe;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let load_balancer_name = String::from("your-load-balancer-name");
    describe(&client, load_balancer_name).await.unwrap();
}
```

### List Load Balancers

```rust
use rustcloud::list_load_balancers;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let load_balancer_names = None; // Optionally, specify load balancer names
    let marker = None; // Optionally, specify a marker for pagination
    let page_size = Some(10);

    list_load_balancers(
        &client,
        load_balancer_names,
        marker,
        page_size
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
