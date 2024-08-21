# rustcloud - GCP Compute Engine (GCE)

## Configure GCP Credentials

First, ensure that your GCP credentials are set up. You can do this by setting the `GOOGLE_APPLICATION_CREDENTIALS` environment variable to point to your service account key file:

```sh
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/your/service-account-file.json"
```

Alternatively, you can use the application default credentials (ADC) setup, which will use your gcloud credentials if authenticated.

## Initialize the Library

```rust
use rustcloud::GCE;

#[tokio::main]
async fn main() {
    let gce_client = GCE::new();

    // Now you're ready to use the client to interact with GCP Compute Engine.
}
```

### Create a Compute Engine Instance

```rust
use rustcloud::GCE;
use std::collections::HashMap;
use serde_json::json;

#[tokio::main]
async fn main() {
    let gce_client = GCE::new();

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), json!("my-gcp-project"));
    request.insert("Zone".to_string(), json!("us-central1-a"));
    request.insert("Name".to_string(), json!("my-instance"));
    request.insert("MachineType".to_string(), json!("n1-standard-1"));
    // Add other parameters as needed

    let response = gce_client.create_node(request).await.unwrap();
    println!("{:?}", response);
}
```

### Start a Compute Engine Instance

```rust
use rustcloud::GCE;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let gce_client = GCE::new();

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "my-gcp-project".to_string());
    request.insert("Zone".to_string(), "us-central1-a".to_string());
    request.insert("instance".to_string(), "my-instance".to_string());

    let response = gce_client.start_node(request).await.unwrap();
    println!("{:?}", response);
}
```

### Stop a Compute Engine Instance

```rust
use rustcloud::GCE;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let gce_client = GCE::new();

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "my-gcp-project".to_string());
    request.insert("Zone".to_string(), "us-central1-a".to_string());
    request.insert("instance".to_string(), "my-instance".to_string());

    let response = gce_client.stop_node(request).await.unwrap();
    println!("{:?}", response);
}
```

### Delete a Compute Engine Instance

```rust
use rustcloud::GCE;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let gce_client = GCE::new();

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "my-gcp-project".to_string());
    request.insert("Zone".to_string(), "us-central1-a".to_string());
    request.insert("instance".to_string(), "my-instance".to_string());

    let response = gce_client.delete_node(request).await.unwrap();
    println!("{:?}", response);
}
```

### Reboot a Compute Engine Instance

```rust
use rustcloud::GCE;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let gce_client = GCE::new();

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "my-gcp-project".to_string());
    request.insert("Zone".to_string(), "us-central1-a".to_string());
    request.insert("instance".to_string(), "my-instance".to_string());

    let response = gce_client.reboot_node(request).await.unwrap();
    println!("{:?}", response);
}
```

### List Compute Engine Instances in a Zone

```rust
use rustcloud::GCE;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let gce_client = GCE::new();

    let mut request = HashMap::new();
    request.insert("projectid".to_string(), "my-gcp-project".to_string());
    request.insert("Zone".to_string(), "us-central1-a".to_string());

    let response = gce_client.list_node(request).await.unwrap();
    println!("{:?}", response);
}
```

## Contributing

If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request. Contributions are welcome!

