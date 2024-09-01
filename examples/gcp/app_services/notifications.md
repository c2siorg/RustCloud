# rustcloud - GCP Pub/Sub

## Configure GCP Credentials

First, ensure that your GCP credentials are set up. You can do this by setting the `GOOGLE_APPLICATION_CREDENTIALS` environment variable to point to your service account key file:

```sh
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/your/service-account-file.json"
```

Alternatively, you can use the application default credentials (ADC) setup, which will use your gcloud credentials if authenticated.

## Initialize the Library

```rust
use rustcloud::Googlenotification;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let notification_client = Googlenotification::new();

    // Now you're ready to use the client to interact with GCP Pub/Sub.
}
```

### Create a Topic

```rust
use rustcloud::Googlenotification;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let client = Googlenotification::new();

    let mut request = HashMap::new();
    request.insert("Project".to_string(), "my-gcp-project".to_string());
    request.insert("Topic".to_string(), "my-topic".to_string());

    let response = client.create_topic(request).await.unwrap();
    println!("{:?}", response);
}
```

### Delete a Topic

```rust
use rustcloud::Googlenotification;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let client = Googlenotification::new();

    let mut request = HashMap::new();
    request.insert("Project".to_string(), "my-gcp-project".to_string());
    request.insert("Topic".to_string(), "my-topic".to_string());

    let response = client.delete_topic(request).await.unwrap();
    println!("{:?}", response);
}
```

### Get a Topic

```rust
use rustcloud::Googlenotification;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let client = Googlenotification::new();

    let mut request = HashMap::new();
    request.insert("Project".to_string(), "my-gcp-project".to_string());
    request.insert("Topic".to_string(), "my-topic".to_string());

    let response = client.get_topic(request).await.unwrap();
    println!("{:?}", response);
}
```

### List All Topics

```rust
use rustcloud::Googlenotification;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let client = Googlenotification::new();

    let mut request = HashMap::new();
    request.insert("Project".to_string(), "my-gcp-project".to_string());
    request.insert("PageSize".to_string(), "10".to_string());

    let response = client.list_topic(request).await.unwrap();
    println!("{:?}", response);
}
```

## Utility: Retrieve Token

Ensure you have a method for retrieving the GCP access token for authenticating API requests:

```rust
use reqwest::Client;

async fn retrieve_token() -> Result<String, Box<dyn std::error::Error>> {
    let client = Client::new();
    let response = client.get("http://metadata.google.internal/computeMetadata/v1/instance/service-accounts/default/token")
        .header("Metadata-Flavor", "Google")
        .send()
        .await?
        .json::<serde_json::Value>()
        .await?;
    
    Ok(response["access_token"].as_str().unwrap().to_string())
}
```

## Contributing

If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request. Contributions are welcome!
