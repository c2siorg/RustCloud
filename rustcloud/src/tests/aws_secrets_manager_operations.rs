use crate::aws::aws_apis::security::aws_secrets_manager;
use aws_sdk_secretsmanager::Client;

async fn get_client() -> Client {
    let config = aws_config::load_from_env().await;
    Client::new(&config)
}

#[tokio::test]
async fn test_create_secret() {
    let client = get_client().await;
    let result = aws_secrets_manager::create_secret(
        &client,
        "rustcloud/test/db-password",
        r#"{"username":"admin","password":"s3cr3t"}"#,
    )
    .await;
    assert!(result.is_ok());
    let arn = result.unwrap();
    assert!(arn.contains("rustcloud"));
    println!("Created secret ARN: {}", arn);
}

#[tokio::test]
async fn test_get_secret() {
    let client = get_client().await;
    let result =
        aws_secrets_manager::get_secret(&client, "rustcloud/test/db-password").await;
    assert!(result.is_ok());
    let value = result.unwrap();
    assert!(!value.is_empty());
    println!("Secret value length: {}", value.len());
}

#[tokio::test]
async fn test_update_secret() {
    let client = get_client().await;
    let result = aws_secrets_manager::update_secret(
        &client,
        "rustcloud/test/db-password",
        r#"{"username":"admin","password":"n3wpassword"}"#,
    )
    .await;
    assert!(result.is_ok());
    println!("Updated secret: {:?}", result.unwrap());
}

#[tokio::test]
async fn test_list_secrets() {
    let client = get_client().await;
    let result = aws_secrets_manager::list_secrets(&client).await;
    assert!(result.is_ok());
    println!("Secrets: {:?}", result.unwrap());
}

#[tokio::test]
async fn test_delete_secret() {
    let client = get_client().await;
    // Use minimum 7-day recovery window
    let result = aws_secrets_manager::delete_secret(
        &client,
        "rustcloud/test/db-password",
        Some(7),
    )
    .await;
    assert!(result.is_ok());
    println!("Scheduled deletion: {:?}", result.unwrap());
}
