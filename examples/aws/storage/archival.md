# rustcloud - AWS Glacier

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
use aws_sdk_glacier::Client;

#[tokio::main]
async fn main() {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    let client = Client::new(&config);

    // Now you're ready to use the client to interact with AWS Glacier.
}
```

### Create Vault

```rust
use rustcloud::create_vault;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let vault_name = String::from("your-vault-name");
    let account_id = String::from("your-account-id");

    create_vault(&client, vault_name, account_id).await.unwrap();
}
```

### Delete Archive

```rust
use rustcloud::delete_archive;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let account_id = String::from("your-account-id");
    let vault_name = String::from("your-vault-name");
    let archive_id = String::from("your-archive-id");

    delete_archive(&client, account_id, vault_name, archive_id).await.unwrap();
}
```

### Delete Vault

```rust
use rustcloud::delete_vault;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let account_id = String::from("your-account-id");
    let vault_name = String::from("your-vault-name");

    delete_vault(&client, account_id, vault_name).await.unwrap();
}
```

### Upload

```rust
use rustcloud::upload;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let account_id = String::from("your-account-id");
    let vault_name = String::from("your-vault-name");
    let archive_description = Some(String::from("your-archive-description"));
    let part_size = Some(String::from("your-part-size"));

    upload(&client, account_id, vault_name, archive_description, part_size).await.unwrap();
}
```

### List Vaults

```rust
use rustcloud::list;

#[tokio::main]
async fn main() {
    let client = setup_client().await;

    let account_id = String::from("your-account-id");
    let marker = Some(String::from("your-marker"));
    let limit = Some(10);

    list(&client, account_id, marker, limit).await.unwrap();
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
