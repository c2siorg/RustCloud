# rustcloud — Azure Key Vault

Azure Key Vault stores and controls access to secrets, encryption keys, and
certificates. RustCloud wraps the Key Vault REST API v7.4 for secret operations.

## Configure Credentials

```sh
export AZURE_KEYVAULT_URL="https://my-vault.vault.azure.net"
export AZURE_KEYVAULT_TOKEN="<azure-ad-bearer-token>"
```

To obtain a bearer token with the Azure CLI:

```sh
az account get-access-token --resource https://vault.azure.net --query accessToken -o tsv
```

## Initialize the Client

```rust
use rustcloud::azure::azure_apis::security::azure_key_vault::AzureKeyVault;

// From environment variables
let vault = AzureKeyVault::new();

// Explicit configuration
let vault = AzureKeyVault::with_config(
    "https://my-vault.vault.azure.net",
    "eyJ0eXAiOiJKV1QiLCJhbGciOiJSUzI1NiJ9...",
);
```

## Operations

### Set a Secret

```rust
let result = vault.set_secret("db-password", "s3cr3t-v@lue").await?;
println!("{:?}", result);
```

### Get a Secret

```rust
let value = vault.get_secret("db-password").await?;
println!("Value: {}", value);
```

### List Secrets

```rust
let names = vault.list_secrets().await?;
for name in names {
    println!("{}", name);
}
```

### Get Secret Versions

```rust
let versions = vault.get_secret_versions("db-password").await?;
println!("{:?}", versions);
```

### Delete a Secret

Soft-delete — secret is recoverable within the vault retention period (default 90 days):

```rust
vault.delete_secret("db-password").await?;
```

## Full example

```rust
use rustcloud::azure::azure_apis::security::azure_key_vault::AzureKeyVault;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let vault = AzureKeyVault::new();

    // Store a database connection string
    vault.set_secret(
        "prod-db-connection",
        "Server=tcp:myserver.database.windows.net;Database=mydb;",
    ).await?;

    // Retrieve it later
    let conn = vault.get_secret("prod-db-connection").await?;
    println!("Connecting to: {}", &conn[..30]);

    // Audit: see how many versions exist
    let versions = vault.get_secret_versions("prod-db-connection").await?;
    println!("Versions: {:?}", versions["body"]["value"]);

    Ok(())
}
```

## Azure security trio

| Service              | Purpose                                       |
|----------------------|-----------------------------------------------|
| Azure AD / auth      | Identity and access management                |
| Azure Key Vault      | Secret, key, and certificate storage          |
| Azure OpenAI         | AI workloads consuming secrets at runtime     |

## Supported operations

| Function              | Description                                     |
|-----------------------|-------------------------------------------------|
| `set_secret`          | Create or update a secret                       |
| `get_secret`          | Retrieve the latest secret value                |
| `delete_secret`       | Soft-delete (recoverable)                       |
| `list_secrets`        | List all secret names in the vault              |
| `get_secret_versions` | List all versions of a named secret             |

## References

- [Azure Key Vault REST API](https://docs.microsoft.com/en-us/rest/api/keyvault/)
- [Azure Key Vault Secrets](https://docs.microsoft.com/en-us/azure/key-vault/secrets/)
