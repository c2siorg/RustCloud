# rustcloud - Google DNS Client

## Overview

The `GoogleDns` struct provides methods to interact with Google Cloud DNS, including creating, listing, and deleting DNS records and managed zones. This client uses Google Cloud's DNS REST API.

## Prerequisites

Ensure that your GCP credentials are set up by configuring the `GOOGLE_APPLICATION_CREDENTIALS` environment variable to point to your service account key file:

```sh
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/your/service-account-file.json"
```

## Initialize the GoogleDns Client

```rust
use rustcloud::GoogleDns;

#[tokio::main]
async fn main() {
    let dns_client = GoogleDns::new("your-gcp-project-id");

    // Now you're ready to use the client to interact with Google DNS.
}
```

## List Resource DNS Record Sets

```rust
use rustcloud::GoogleDns;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let dns_client = GoogleDns::new("your-gcp-project-id");

    let mut options = HashMap::new();
    options.insert("managedZone", "your-managed-zone-id");
    options.insert("maxResults", "10");
    options.insert("pageToken", "your-page-token");
    options.insert("sortBy", "name");
    options.insert("sortOrder", "ascending");

    let response = dns_client.list_resource_dns_record_sets(&options).await.unwrap();
    println!("{:?}", response);
}
```

### Parameters:
- `managedZone`: The unique name of the managed zone, e.g., `your-managed-zone-id`.
- `maxResults`: (Optional) Maximum number of results to return.
- `pageToken`: (Optional) A token identifying a page of results the server should return.
- `sortBy`: (Optional) Sort results by a specified field.
- `sortOrder`: (Optional) Order of results (ascending/descending).

## Create a DNS Managed Zone

```rust
use rustcloud::GoogleDns;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let dns_client = GoogleDns::new("your-gcp-project-id");

    let mut params = HashMap::new();
    params.insert("Project", "your-gcp-project-id");
    params.insert("Description", "Description of your DNS zone");
    params.insert("DnsName", "example.com.");
    params.insert("nameServers", "ns1.example.com.,ns2.example.com.");
    params.insert("Id", "your-id");
    params.insert("Kind", "dns#managedZone");
    params.insert("Name", "your-zone-name");
    params.insert("nameServerSet", "default");

    let response = dns_client.create_dns(&params).await.unwrap();
    println!("{:?}", response);
}
```

### Parameters:
- `Project`: The GCP project ID.
- `Description`: A description of the managed zone.
- `DnsName`: The DNS name of the managed zone.
- `nameServers`: A comma-separated list of name servers.
- `Id`: The unique identifier for the managed zone.
- `Kind`: The type of resource.
- `Name`: The name of the managed zone.
- `nameServerSet`: The set of name servers to use.

## List DNS Managed Zones

```rust
use rustcloud::GoogleDns;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let dns_client = GoogleDns::new("your-gcp-project-id");

    let mut options = HashMap::new();
    options.insert("maxResults", "10");
    options.insert("pageToken", "your-page-token");

    let response = dns_client.list_dns(&options).await.unwrap();
    println!("{:?}", response);
}
```

### Parameters:
- `maxResults`: (Optional) Maximum number of results to return.
- `pageToken`: (Optional) A token identifying a page of results the server should return.

## Delete a DNS Managed Zone

```rust
use rustcloud::GoogleDns;
use std::collections::HashMap;

#[tokio::main]
async fn main() {
    let dns_client = GoogleDns::new("your-gcp-project-id");

    let mut options = HashMap::new();
    options.insert("managedZone", "your-managed-zone-id");

    let response = dns_client.delete_dns(&options).await.unwrap();
    println!("{:?}", response);
}
```

### Parameters:
- `managedZone`: The unique name of the managed zone to delete, e.g., `your-managed-zone-id`.

## Contributing

If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request. Contributions are welcome!

