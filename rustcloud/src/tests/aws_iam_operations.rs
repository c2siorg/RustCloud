use crate::aws::aws_apis::security::aws_iam::*;
use aws_sdk_iam::Client;

async fn create_client() -> Client {
    let config = aws_config::load_defaults(aws_config::BehaviorVersion::latest()).await;
    Client::new(&config)
}

#[tokio::test]
async fn test_attach_group_policy() {
    let client = create_client().await;

    let group_name = "TestGroup".to_string();
    let policy_arn = "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess".to_string();

    let result = attach_group_policy(&client, group_name, policy_arn).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_group() {
    let client = create_client().await;

    let path = "/".to_string();
    let group_name = "TestGroup".to_string();

    let result = create_group(&client, path, group_name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_group() {
    let client = create_client().await;

    let group_name = "TestGroup".to_string();

    let result = delete_group(&client, group_name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_detach_group_policy() {
    let client = create_client().await;

    let group_name = "TestGroup".to_string();
    let policy_arn = "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess".to_string();

    let result = detach_group_policy(&client, group_name, policy_arn).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_describe_group() {
    let client = create_client().await;

    let group_name = "TestGroup".to_string();
    let marker = None;
    let max_items = Some(100);

    let result = describe(&client, group_name, marker, max_items).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_user() {
    let client = create_client().await;

    let user_name = "TestUser".to_string();
    let path = Some("/".to_string());

    let result = create_user(&client, user_name, path, None).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_user() {
    let client = create_client().await;

    let user_name = "TestUser".to_string();

    let result = delete_user(&client, user_name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_users() {
    let client = create_client().await;

    let result = list_users(&client, None, None, Some(50)).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_attach_user_policy() {
    let client = create_client().await;

    let user_name = "TestUser".to_string();
    let policy_arn = "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess".to_string();

    let result = attach_user_policy(&client, user_name, policy_arn).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_detach_user_policy() {
    let client = create_client().await;

    let user_name = "TestUser".to_string();
    let policy_arn = "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess".to_string();

    let result = detach_user_policy(&client, user_name, policy_arn).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_create_role() {
    let client = create_client().await;

    let role_name = "TestRole".to_string();
    // Minimal trust policy that allows EC2 to assume this role
    let assume_role_policy_document = r#"{
        "Version": "2012-10-17",
        "Statement": [{
            "Effect": "Allow",
            "Principal": { "Service": "ec2.amazonaws.com" },
            "Action": "sts:AssumeRole"
        }]
    }"#
    .to_string();

    let result = create_role(
        &client,
        role_name,
        assume_role_policy_document,
        Some("/".to_string()),
        Some("Test role created by RustCloud".to_string()),
    )
    .await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_role() {
    let client = create_client().await;

    let role_name = "TestRole".to_string();

    let result = delete_role(&client, role_name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_roles() {
    let client = create_client().await;

    let result = list_roles(&client, None, None, Some(50)).await;
    assert!(result.is_ok());
}
