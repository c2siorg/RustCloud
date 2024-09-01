# rustcloud - AWS S3 Operations

## Configure AWS credentials

Ensure that your AWS credentials are configured by setting the environment variables:

```sh
export AWS_ACCESS_KEY_ID= "xxxxxxxxxxxx"
export AWS_SECRET_ACCESS_KEY= "xxxxxxxxxxxx"
```

Alternatively, you can use the AWS credentials file located in your `<HOME>/.aws/credentials`.

## Initialize the library

```rust
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_s3::Client;

#[tokio::main]
async fn main() {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Now you're ready to use the client to interact with AWS S3.
}
```

### Create Bucket

```rust
use rustcloud::create_bucket;
use aws_sdk_s3::types::{BucketCannedAcl, CreateBucketConfiguration, ObjectOwnership};

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let acl = BucketCannedAcl::Private;
    let bucket = String::from("my-new-bucket");
    let create_bucket_configuration = CreateBucketConfiguration::builder()
        .location_constraint("us-east-1")
        .build();
    let grant_full_control = Some(String::from("full-control-grant"));
    let grant_read = Some(String::from("read-grant"));
    let grant_read_acp = Some(String::from("read-acp-grant"));
    let grant_write = Some(String::from("write-grant"));
    let grant_write_acp = Some(String::from("write-acp-grant"));
    let object_lock_enabled_for_bucket = Some(true);
    let object_ownership = Some(ObjectOwnership::BucketOwnerPreferred);

    create_bucket(
        &client,
        acl,
        bucket,
        create_bucket_configuration,
        grant_full_control,
        grant_read,
        grant_read_acp,
        grant_write,
        grant_write_acp,
        object_lock_enabled_for_bucket,
        object_ownership
    ).await.unwrap();
}
```

### Delete Bucket

```rust
use rustcloud::delete;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let bucket = String::from("my-bucket-to-delete");
    let expected_bucket_owner = Some(String::from("expected-bucket-owner"));

    delete(&client, bucket, expected_bucket_owner).await.unwrap();
}
```

### Delete Object

```rust
use rustcloud::delete_object;
use aws_sdk_s3::types::RequestPayer;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let bucket = String::from("my-bucket");
    let key = String::from("my-object-key");
    let expected_bucket_owner = Some(String::from("expected-bucket-owner"));
    let mfa = Some(String::from("mfa-token"));
    let version_id = Some(String::from("object-version-id"));
    let request_payer = Some(RequestPayer::Requester);

    delete_object(
        &client,
        bucket,
        key,
        expected_bucket_owner,
        mfa,
        version_id,
        request_payer
    ).await.unwrap();
}
```

### List Buckets

```rust
use rustcloud::list;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    list(&client).await.unwrap();
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
