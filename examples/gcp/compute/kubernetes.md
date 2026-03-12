# rustcloud - GCP Kubernetes Client

## Overview

The `GCPKubernetesClient` allows you to interact with Google Kubernetes Engine (GKE) to manage Kubernetes clusters and node pools on Google Cloud Platform (GCP).

## Prerequisites

Ensure that your GCP credentials are set up. You can do this by setting the `GOOGLE_APPLICATION_CREDENTIALS` environment variable to point to your service account key file:

```sh
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/your/service-account-file.json"
```

## Initialize the GCP Kubernetes Client

```rust
use rustcloud::GCPKubernetesClient;

#[tokio::main]
async fn main() {
    let kubernetes_client = GCPKubernetesClient::new();

    // Now you're ready to use the client to interact with GKE.
}
```

## Create a Kubernetes Cluster

```rust
use rustcloud::GCPKubernetesClient;
use rustcloud::gcp::types::compute::gcp_kubernetes_types::CreateClusterRequest;

#[tokio::main]
async fn main() {
    let kubernetes_client = GCPKubernetesClient::new();

    let request = CreateClusterRequest {
        projectId: "my-gcp-project".to_string(),
        zone: "us-central1-a".to_string(),
        // Add other necessary fields
    };

    let response = kubernetes_client.create_cluster(request).await.unwrap();
    println!("{:?}", response);
}
```

## Delete a Kubernetes Cluster

```rust
use rustcloud::GCPKubernetesClient;
use rustcloud::gcp::types::compute::gcp_kubernetes_types::DeleteClusterRequest;

#[tokio::main]
async fn main() {
    let kubernetes_client = GCPKubernetesClient::new();

    let request = DeleteClusterRequest {
        project_id: "my-gcp-project".to_string(),
        zone: "us-central1-a".to_string(),
        cluster_id: "my-cluster".to_string(),
    };

    let response = kubernetes_client.delete_cluster(request).await.unwrap();
    println!("{:?}", response);
}
```

## List Kubernetes Clusters

```rust
use rustcloud::GCPKubernetesClient;
use rustcloud::gcp::types::compute::gcp_kubernetes_types::ListClustersRequest;

#[tokio::main]
async fn main() {
    let kubernetes_client = GCPKubernetesClient::new();

    let request = ListClustersRequest {
        project_id: "my-gcp-project".to_string(),
        zone: "us-central1-a".to_string(),
    };

    let response = kubernetes_client.list_clusters(request).await.unwrap();
    println!("{:?}", response);
}
```

## Get a Kubernetes Cluster

```rust
use rustcloud::GCPKubernetesClient;
use rustcloud::gcp::types::compute::gcp_kubernetes_types::GetClusterRequest;

#[tokio::main]
async fn main() {
    let kubernetes_client = GCPKubernetesClient::new();

    let request = GetClusterRequest {
        project_id: "my-gcp-project".to_string(),
        zone: "us-central1-a".to_string(),
        cluster_id: "my-cluster".to_string(),
    };

    let response = kubernetes_client.get_cluster(request).await.unwrap();
    println!("{:?}", response);
}
```

## Create a Node Pool

```rust
use rustcloud::GCPKubernetesClient;
use rustcloud::gcp::types::compute::gcp_kubernetes_types::CreateNodePoolRequest;

#[tokio::main]
async fn main() {
    let kubernetes_client = GCPKubernetesClient::new();

    let request = CreateNodePoolRequest {
        projectId: "my-gcp-project".to_string(),
        zone: "us-central1-a".to_string(),
        clusterId: "my-cluster".to_string(),
        // Add other necessary fields
    };

    let response = kubernetes_client.create_node_pool(request).await.unwrap();
    println!("{:?}", response);
}
```

## Delete a Node Pool

```rust
use rustcloud::GCPKubernetesClient;
use rustcloud::gcp::types::compute::gcp_kubernetes_types::DeleteNodePoolRequest;

#[tokio::main]
async fn main() {
    let kubernetes_client = GCPKubernetesClient::new();

    let request = DeleteNodePoolRequest {
        project_id: "my-gcp-project".to_string(),
        zone: "us-central1-a".to_string(),
        cluster_id: "my-cluster".to_string(),
        node_pool_id: "my-node-pool".to_string(),
    };

    let response = kubernetes_client.delete_node_pool(request).await.unwrap();
    println!("{:?}", response);
}
```

## Get a Node Pool

```rust
use rustcloud::GCPKubernetesClient;
use rustcloud::gcp::types::compute::gcp_kubernetes_types::GetNodePoolRequest;

#[tokio::main]
async fn main() {
    let kubernetes_client = GCPKubernetesClient::new();

    let request = GetNodePoolRequest {
        project_id: "my-gcp-project".to_string(),
        zone: "us-central1-a".to_string(),
        cluster_id: "my-cluster".to_string(),
        node_pool_id: "my-node-pool".to_string(),
    };

    let response = kubernetes_client.get_node_pool(request).await.unwrap();
    println!("{:?}", response);
}
```

## List Node Pools

```rust
use rustcloud::GCPKubernetesClient;
use rustcloud::gcp::types::compute::gcp_kubernetes_types::ListNodePoolsRequest;

#[tokio::main]
async fn main() {
    let kubernetes_client = GCPKubernetesClient::new();

    let request = ListNodePoolsRequest {
        project_id: "my-gcp-project".to_string(),
        zone: "us-central1-a".to_string(),
        cluster_id: "my-cluster".to_string(),
    };

    let response = kubernetes_client.list_node_pools(request).await.unwrap();
    println!("{:?}", response);
}
```

## Set Addons Config

```rust
use rustcloud::GCPKubernetesClient;
use rustcloud::gcp::types::compute::gcp_kubernetes_types::SetAddonsConfigRequest;

#[tokio::main]
async fn main() {
    let kubernetes_client = GCPKubernetesClient::new();

    let request = SetAddonsConfigRequest {
        projectId: "my-gcp-project".to_string(),
        zone: "us-central1-a".to_string(),
        clusterId: "my-cluster".to_string(),
        // Add other necessary fields
    };

    let response = kubernetes_client.set_addons_config(request).await.unwrap();
    println!("{:?}", response);
}
```

## Contributing

If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request. Contributions are welcome!
