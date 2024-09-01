# rustcloud - GCP AutoML

## Configure GCP Credentials

First, ensure that your GCP credentials are set up. You can do this by setting the `GOOGLE_APPLICATION_CREDENTIALS` environment variable to point to your service account key file:

```sh
export GOOGLE_APPLICATION_CREDENTIALS="/path/to/your/service-account-file.json"
```

Alternatively, you can use the application default credentials (ADC) setup, which will use your gcloud credentials if authenticated.

## Initialize the Library

```rust
use rustcloud::AutoML;

#[tokio::main]
async fn main() {
    let project_id = "my-gcp-project";
    let automl_client = AutoML::new(project_id);

    // Now you're ready to use the client to interact with GCP AutoML.
}
```

### Create a Dataset

```rust
use rustcloud::AutoML;

#[tokio::main]
async fn main() {
    let project_id = "my-gcp-project";
    let automl_client = AutoML::new(project_id);

    let location = "us-central1";
    let dataset_name = "my-dataset";

    let response = automl_client.create_dataset(location, dataset_name).await.unwrap();
    println!("{:?}", response);
}
```

### Get a Dataset

```rust
use rustcloud::AutoML;

#[tokio::main]
async fn main() {
    let project_id = "my-gcp-project";
    let automl_client = AutoML::new(project_id);

    let location = "us-central1";
    let dataset_id = "my-dataset-id";

    let response = automl_client.get_dataset(location, dataset_id).await.unwrap();
    println!("{:?}", response);
}
```

### Import Data into Dataset

```rust
use rustcloud::AutoML;

#[tokio::main]
async fn main() {
    let project_id = "my-gcp-project";
    let automl_client = AutoML::new(project_id);

    let location = "us-central1";
    let dataset_id = "my-dataset-id";
    let uris = vec!["gs://my-bucket/my-data.csv".to_string()];

    let response = automl_client.import_data_set(location, dataset_id, uris).await.unwrap();
    println!("{:?}", response);
}
```

### List All Models

```rust
use rustcloud::AutoML;

#[tokio::main]
async fn main() {
    let project_id = "my-gcp-project";
    let automl_client = AutoML::new(project_id);

    let location = "us-central1";

    let response = automl_client.list_models(location).await.unwrap();
    println!("{:?}", response);
}
```

### Create a Model

```rust
use rustcloud::AutoML;

#[tokio::main]
async fn main() {
    let project_id = "my-gcp-project";
    let automl_client = AutoML::new(project_id);

    let location = "us-central1";
    let dataset_id = "my-dataset-id";
    let model_name = "my-model";
    let target_column_id = "target-column";
    let train_budget = 1000; // Training budget in milli node hours

    let response = automl_client.create_model(location, dataset_id, model_name, target_column_id, train_budget).await.unwrap();
    println!("{:?}", response);
}
```

### Deploy a Model

```rust
use rustcloud::AutoML;

#[tokio::main]
async fn main() {
    let project_id = "my-gcp-project";
    let automl_client = AutoML::new(project_id);

    let location = "us-central1";
    let model_id = "my-model-id";

    let response = automl_client.deploy_model(location, model_id).await.unwrap();
    println!("{:?}", response);
}
```

### Undeploy a Model

```rust
use rustcloud::AutoML;

#[tokio::main]
async fn main() {
    let project_id = "my-gcp-project";
    let automl_client = AutoML::new(project_id);

    let location = "us-central1";
    let model_id = "my-model-id";

    let response = automl_client.undeploy_model(location, model_id).await.unwrap();
    println!("{:?}", response);
}
```

### Get a Model

```rust
use rustcloud::AutoML;

#[tokio::main]
async fn main() {
    let project_id = "my-gcp-project";
    let automl_client = AutoML::new(project_id);

    let location = "us-central1";
    let model_id = "my-model-id";

    let response = automl_client.get_model(location, model_id).await.unwrap();
    println!("{:?}", response);
}
```

### Export a Dataset

```rust
use rustcloud::AutoML;

#[tokio::main]
async fn main() {
    let project_id = "my-gcp-project";
    let automl_client = AutoML::new(project_id);

    let location = "us-central1";
    let dataset_id = "my-dataset-id";
    let gcs_uri = "gs://my-bucket/output/";

    let response = automl_client.export_dataset(location, dataset_id, gcs_uri).await.unwrap();
    println!("{:?}", response);
}
```

### Delete a Model

```rust
use rustcloud::AutoML;

#[tokio::main]
async fn main() {
    let project_id = "my-gcp-project";
    let automl_client = AutoML::new(project_id);

    let location = "us-central1";
    let model_id = "my-model-id";

    let response = automl_client.delete_model(location, model_id).await.unwrap();
    println!("{:?}", response);
}
```

### Delete a Dataset

```rust
use rustcloud::AutoML;

#[tokio::main]
async fn main() {
    let project_id = "my-gcp-project";
    let automl_client = AutoML::new(project_id);

    let location = "us-central1";
    let dataset_id = "my-dataset-id";

    let response = automl_client.delete_dataset(location, dataset_id).await.unwrap();
    println!("{:?}", response);
}
```

## Contributing

If you encounter any issues or have suggestions for improvements, please feel free to open an issue or submit a pull request. Contributions are welcome!
