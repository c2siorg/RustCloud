# rustcloud - AWS ECS

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
use aws_sdk_ecs::Client;

#[tokio::main]
async fn main() {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Now you're ready to use the client to interact with AWS ECS.
}
```

### Create a Cluster

```rust
use rustcloud::create_cluster;
use aws_sdk_ecs::types::{Tag, ClusterConfiguration, ClusterSetting};

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let name = String::from("my-cluster");
    let tags = Some(vec![Tag::builder().key("Environment").value("Production").build()]);
    let settings = None; // Add settings if necessary
    let configuration = ClusterConfiguration::default(); // Modify as per your needs
    let capacity_providers = None; // Add capacity providers if necessary

    create_cluster(&client, &name, tags, settings, configuration, capacity_providers).await.unwrap();
}
```

### Delete a Cluster

```rust
use rustcloud::delete_cluster;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let name = String::from("my-cluster");

    delete_cluster(&client, &name).await.unwrap();
}
```

### Describe a Cluster

```rust
use rustcloud::describe_cluster;
use aws_sdk_ecs::types::ClusterField;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let clusters = Some(vec![String::from("my-cluster")]);
    let include = Some(vec![ClusterField::Tags]); // Modify as per your needs

    describe_cluster(&client, clusters, include).await.unwrap();
}
```

### List All Clusters

```rust
use rustcloud::show_clusters;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let max_results = Some(10); // Adjust the max results as needed

    show_clusters(&client, max_results).await.unwrap();
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
