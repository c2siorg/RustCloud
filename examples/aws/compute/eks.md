# rustcloud - AWS EKS

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
use aws_sdk_eks::Client;

#[tokio::main]
async fn main() {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Now you're ready to use the client to interact with AWS EKS.
}
```

### Create a Cluster

```rust
use rustcloud::create_cluster;
use aws_sdk_eks::types::{VpcConfigRequest, KubernetesNetworkConfigRequest};

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let cluster_name = String::from("my-cluster");
    let version = Some(String::from("1.21"));
    let role_arn = Some(String::from("arn:aws:iam::123456789012:role/EKS-Role"));
    let resources_vpc_config = Some(VpcConfigRequest::builder().subnet_ids("subnet-12345678").build());
    let kubernetes_network_config = Some(KubernetesNetworkConfigRequest::builder().service_ipv4_cidr("10.0.0.0/16").build());

    create_cluster(&client, cluster_name, version, role_arn, resources_vpc_config, kubernetes_network_config).await.unwrap();
}
```

### Create a Node Group

```rust
use rustcloud::create_node_group;
use aws_sdk_eks::types::{NodegroupScalingConfig, AmiTypes, RemoteAccessConfig};

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let cluster_name = String::from("my-cluster");
    let nodegroup_name = String::from("my-nodegroup");
    let scaling_config = Some(NodegroupScalingConfig::builder().min_size(1).max_size(3).desired_size(2).build());
    let subnets = Some(vec![String::from("subnet-12345678")]);
    let instance_types = Some(vec![String::from("t3.medium")]);
    let ami_type = Some(AmiTypes::Al2X8664);
    let remote_access = Some(RemoteAccessConfig::builder().ec2_ssh_key("my-key").build());
    let node_role = Some(String::from("arn:aws:iam::123456789012:role/EKS-NodeRole"));

    create_node_group(
        &client, 
        cluster_name, 
        nodegroup_name, 
        Some(20), 
        scaling_config, 
        subnets, 
        instance_types, 
        ami_type, 
        remote_access, 
        node_role, 
        None, 
        None, 
        None, 
        None, 
        None, 
        None
    ).await.unwrap();
}
```

### Delete a Node Group

```rust
use rustcloud::delete_nodegroup;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let cluster_name = String::from("my-cluster");
    let nodegroup_name = String::from("my-nodegroup");

    delete_nodegroup(&client, cluster_name, nodegroup_name).await.unwrap();
}
```

### Describe a Cluster

```rust
use rustcloud::describe_cluster;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let cluster_name = String::from("my-cluster");

    describe_cluster(&client, cluster_name).await.unwrap();
}
```

### Describe a Node Group

```rust
use rustcloud::describe_nodegroup;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let cluster_name = String::from("my-cluster");
    let nodegroup_name = String::from("my-nodegroup");

    describe_nodegroup(&client, cluster_name, nodegroup_name).await.unwrap();
}
```

### Delete a Cluster

```rust
use rustcloud::delete_cluster;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let cluster_name = String::from("my-cluster");

    delete_cluster(&client, &cluster_name).await.unwrap();
}
```

### List Clusters

```rust
use rustcloud::list_clusters;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    list_clusters(&client, Some(10), None).await.unwrap();
}
```

### List Node Groups

```rust
use rustcloud::list_nodegroups;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let cluster_name = String::from("my-cluster");

    list_nodegroups(&client, cluster_name, Some(10)).await.unwrap();
}
```

### Update Tags

```rust
use rustcloud::update_tags;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let resource_arn = String::from("arn:aws:eks:us-east-1:123456789012:cluster/my-cluster");
    let tags = Some(HashMap::from([
        (String::from("Environment"), String::from("Production")),
        (String::from("Team"), String::from("DevOps"))
    ]));

    update_tags(&client, resource_arn, tags).await.unwrap();
}
```

### Update Cluster Configuration

```rust
use rustcloud::update_config;
use aws_sdk_eks::types::{VpcConfigRequest, Logging};

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let cluster_name = String::from("my-cluster");
    let resources_vpc_config = Some(VpcConfigRequest::builder().subnet_ids("subnet-12345678").build());
    let logging = Some(Logging::builder().cluster_logging(vec![]).build());

    update_config(&client, cluster_name, resources_vpc_config, logging, None, None).await.unwrap();
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
