# DigitalOcean Kubernetes (DOKS) — RustCloud Examples

Managed Kubernetes cluster operations via `DigiOceanKubernetes`.
This is the first Kubernetes provider for DigitalOcean in RustCloud.

## Setup

```rust
use rustcloud::digiocean::digiocean_apis::kubernetes::digiocean_kubernetes::DigiOceanKubernetes;

let token = std::env::var("DIGITALOCEAN_TOKEN").unwrap();
let client = DigiOceanKubernetes::new(token);
```

---

## List clusters

```rust
let result = client.list_clusters().await?;
println!("{}", result["body"]);
```

## Create a cluster

```rust
use serde_json::json;
use std::collections::HashMap;

let mut request = HashMap::new();
request.insert("name".to_string(), json!("production"));
request.insert("region".to_string(), json!("nyc1"));
request.insert("version".to_string(), json!("1.29.1-do.0"));
request.insert("node_pools".to_string(), json!([{
    "size": "s-2vcpu-4gb",
    "name": "worker-pool",
    "count": 3
}]));

let result = client.create_cluster(request).await?;
println!("{}", result["body"]); // contains cluster.id for future calls
```

## Get cluster details

```rust
let result = client.get_cluster("bd5f5959-5e1e-4205-a714-a914373942af").await?;
println!("{}", result["body"]);
```

## Get kubeconfig

Downloads the kubeconfig YAML for use with `kubectl`:

```rust
let result = client.get_kubeconfig("bd5f5959-5e1e-4205-a714-a914373942af").await?;
let kubeconfig_yaml = result["body"].as_str().unwrap_or("");
std::fs::write("~/.kube/config", kubeconfig_yaml)?;
```

## Add a node pool

```rust
let mut pool = HashMap::new();
pool.insert("name".to_string(), json!("gpu-pool"));
pool.insert("size".to_string(), json!("g-8vcpu-32gb"));
pool.insert("count".to_string(), json!(2));

client.add_node_pool("bd5f5959-5e1e-4205-a714-a914373942af", pool).await?;
```

## Delete a node pool

```rust
client.delete_node_pool(
    "bd5f5959-5e1e-4205-a714-a914373942af",
    "pool-id-here",
).await?;
```

## Delete a cluster

```rust
client.delete_cluster("bd5f5959-5e1e-4205-a714-a914373942af").await?;
```

---

## Available regions

| Slug | Region |
|---|---|
| `nyc1` | New York 1 |
| `sfo3` | San Francisco 3 |
| `ams3` | Amsterdam 3 |
| `sgp1` | Singapore 1 |
| `lon1` | London 1 |
| `fra1` | Frankfurt 1 |
| `tor1` | Toronto 1 |
| `blr1` | Bangalore 1 |
