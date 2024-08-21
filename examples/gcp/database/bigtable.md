# rustcloud - GCP Bigtable Client

## Overview

The `Bigtable` struct provides methods to interact with Google Cloud Bigtable, including creating, listing, describing, and deleting tables. This client uses Google Cloud's REST API.

## Prerequisites

Ensure that your GCP credentials are set up by configuring the `GOOGLE_APPLICATION_CREDENTIALS` environment variable to point to your service account key file:

```sh
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/your/service-account-file.json"
```

## Initialize the Bigtable Client

```rust
use rustcloud::Bigtable;

#[tokio::main]
async fn main() {
    let bigtable_client = Bigtable::new("your-gcp-project-id");

    // Now you're ready to use the client to interact with Bigtable.
}
```

## List Tables

```rust
use rustcloud::Bigtable;

#[tokio::main]
async fn main() {
    let bigtable_client = Bigtable::new("your-gcp-project-id");

    let parent = "projects/your-project-id/instances/your-instance-id";
    let response = bigtable_client.list_tables(parent, None, None).await.unwrap();
    println!("{:?}", response);
}
```

### Parameters:
- `parent`: The unique name of the instance for which tables should be listed, e.g., `projects/{project_id}/instances/{instance_id}`.
- `page_token`: (Optional) A token identifying a page of results the server should return.
- `view`: (Optional) The view to return. Defaults to `NAME_ONLY`.

## Create a Table

```rust
use rustcloud::Bigtable;
use rustcloud::gcp::types::database::gcp_bigtable_types::{Table, InitialSplits, ClusterStates};

#[tokio::main]
async fn main() {
    let bigtable_client = Bigtable::new("your-gcp-project-id");

    let parent = "projects/your-project-id/instances/your-instance-id";
    let table_id = "your-table-id";
    let table = Table {
        // Populate table fields
    };
    let initial_splits = vec![
        InitialSplits {
            // Populate initial splits fields
        },
    ];
    let cluster_states = ClusterStates {
        // Populate cluster states fields
    };

    let response = bigtable_client.create_tables(parent, table_id, table, initial_splits, cluster_states).await.unwrap();
    println!("{:?}", response);
}
```

### Parameters:
- `parent`: The unique name of the instance in which to create the table, e.g., `projects/{project_id}/instances/{instance_id}`.
- `table_id`: The ID to be used when referring to the table within its instance.
- `table`: The `Table` object containing the table configuration.
- `initial_splits`: A list of initial row key splits to be created in the table.
- `cluster_states`: The cluster configuration for the table.

## Describe a Table

```rust
use rustcloud::Bigtable;

#[tokio::main]
async fn main() {
    let bigtable_client = Bigtable::new("your-gcp-project-id");

    let name = "projects/your-project-id/instances/your-instance-id/tables/your-table-id";
    let response = bigtable_client.describe_tables(name).await.unwrap();
    println!("{:?}", response);
}
```

### Parameters:
- `name`: The unique name of the table to be described, e.g., `projects/{project_id}/instances/{instance_id}/tables/{table_id}`.

## Delete a Table

```rust
use rustcloud::Bigtable;

#[tokio::main]
async fn main() {
    let bigtable_client = Bigtable::new("your-gcp-project-id");

    let name = "projects/your-project-id/instances/your-instance-id/tables/your-table-id";
    let response = bigtable_client.delete_tables(name).await.unwrap();
    println!("{:?}", response);
}
```

### Parameters:
- `name`: The unique name of the table to be deleted, e.g., `projects/{project_id}/instances/{instance_id}/tables/{table_id}`.

## Contributing

If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request. Contributions are welcome!

