use crate::azure::azure_apis::security::azure_key_vault::AzureKeyVault;

fn get_client() -> AzureKeyVault {
    AzureKeyVault::new()
}

#[tokio::test]
async fn test_set_secret() {
    let client = get_client();
    let result = client.set_secret("rustcloud-test-secret", "hello-rustcloud").await;
    assert!(result.is_ok());
    let resp = result.unwrap();
    println!("Set secret: {:?}", resp);
}

#[tokio::test]
async fn test_get_secret() {
    let client = get_client();
    let result = client.get_secret("rustcloud-test-secret").await;
    assert!(result.is_ok());
    let value = result.unwrap();
    assert_eq!(value, "hello-rustcloud");
    println!("Got secret value: {}", value);
}

#[tokio::test]
async fn test_list_secrets() {
    let client = get_client();
    let result = client.list_secrets().await;
    assert!(result.is_ok());
    let names = result.unwrap();
    println!("Secrets in vault: {:?}", names);
}

#[tokio::test]
async fn test_get_secret_versions() {
    let client = get_client();
    let result = client.get_secret_versions("rustcloud-test-secret").await;
    assert!(result.is_ok());
    println!("Versions: {:?}", result.unwrap());
}

#[tokio::test]
async fn test_delete_secret() {
    let client = get_client();
    let result = client.delete_secret("rustcloud-test-secret").await;
    assert!(result.is_ok());
    println!("Deleted: {:?}", result.unwrap());
}
