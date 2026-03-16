# rustcloud — GCP Cloud Storage (GCS)

Google Cloud Storage is GCP's object storage service for blobs, files, and
static assets. RustCloud wraps the GCS JSON API v1 for bucket and object operations.

> **Note:** The existing `gcp_storage.rs` file implements Compute Engine
> persistent disk operations. This module (`gcp_object_storage`) covers GCS
> bucket and object management.

## Configure Credentials

```sh
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/service-account.json"
export GCP_PROJECT_ID="my-project"
# or: gcloud auth application-default login
```

## Initialize the Client

```rust
use rustcloud::gcp::gcp_apis::storage::gcp_object_storage::GcsClient;

let client = GcsClient::new("my-gcp-project");
```

## Operations

### Create a Bucket

```rust
let result = client.create_bucket("my-app-assets", "US-EAST1").await?;
println!("Status: {}", result["status"]);
```

Common locations: `US`, `EU`, `ASIA`, `US-EAST1`, `US-CENTRAL1`, `EUROPE-WEST1`

### List Buckets

```rust
let result = client.list_buckets().await?;
if let Some(items) = result["body"]["items"].as_array() {
    for bucket in items {
        println!("{}", bucket["name"]);
    }
}
```

### Upload an Object

```rust
let content = std::fs::read("logo.png")?;
let result = client
    .upload_object("my-app-assets", "images/logo.png", "image/png", content)
    .await?;
println!("Uploaded: {:?}", result["body"]["selfLink"]);
```

### Download an Object

```rust
let bytes = client.download_object("my-app-assets", "images/logo.png").await?;
std::fs::write("logo-downloaded.png", bytes)?;
```

### List Objects

```rust
// All objects
let result = client.list_objects("my-app-assets", None).await?;

// With prefix filter
let result = client.list_objects("my-app-assets", Some("images/")).await?;
if let Some(items) = result["body"]["items"].as_array() {
    for obj in items {
        println!("{} — {} bytes", obj["name"], obj["size"]);
    }
}
```

### Delete an Object

```rust
client.delete_object("my-app-assets", "images/logo.png").await?;
```

### Delete a Bucket

Bucket must be empty before deletion:

```rust
client.delete_bucket("my-app-assets").await?;
```

## Full example — upload and serve static assets

```rust
use rustcloud::gcp::gcp_apis::storage::gcp_object_storage::GcsClient;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let client = GcsClient::new("my-project");
    let bucket = "my-static-assets";

    client.create_bucket(bucket, "US").await?;

    for file in ["index.html", "style.css", "app.js"] {
        let bytes = std::fs::read(format!("dist/{}", file))?;
        let mime = match file.rsplit('.').next() {
            Some("html") => "text/html",
            Some("css")  => "text/css",
            Some("js")   => "application/javascript",
            _            => "application/octet-stream",
        };
        client.upload_object(bucket, file, mime, bytes).await?;
        println!("Uploaded {}", file);
    }

    let objects = client.list_objects(bucket, None).await?;
    println!("Objects in bucket: {:?}", objects["body"]["items"]);

    Ok(())
}
```

## Supported operations

| Function          | Description                                     |
|-------------------|-------------------------------------------------|
| `create_bucket`   | Create a bucket in a given location             |
| `delete_bucket`   | Delete an empty bucket                          |
| `list_buckets`    | List all buckets in the project                 |
| `upload_object`   | Upload bytes with content type                  |
| `download_object` | Download object content as raw bytes            |
| `delete_object`   | Delete a named object                           |
| `list_objects`    | List objects with optional prefix filter        |

## References

- [GCS JSON API](https://cloud.google.com/storage/docs/json_api/v1)
- [GCS Buckets](https://cloud.google.com/storage/docs/creating-buckets)
- [GCS Objects](https://cloud.google.com/storage/docs/uploading-objects)
