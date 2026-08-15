use aws_sdk_iam::config::{BehaviorVersion, Credentials, Region, SharedCredentialsProvider};
use aws_sdk_iam::{Client, Config};
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;

use crate::aws::aws_apis::security::aws_iam::*;

fn mock_client(response_body: &str) -> Client {
    let request = http::Request::builder()
        .method("POST")
        .uri("https://iam.amazonaws.com/")
        .body(SdkBody::empty())
        .unwrap();
    let response = http::Response::builder()
        .status(200)
        .header("content-type", "text/xml")
        .body(SdkBody::from(response_body))
        .unwrap();
    let http_client = StaticReplayClient::new(vec![ReplayEvent::new(request, response)]);

    let config = Config::builder()
        .behavior_version(BehaviorVersion::latest())
        .region(Region::new("us-east-1"))
        .credentials_provider(SharedCredentialsProvider::new(Credentials::new(
            "fake-access-key",
            "fake-secret-key",
            None,
            None,
            "test",
        )))
        .http_client(http_client)
        .build();

    Client::from_conf(config)
}

const REQUEST_ID: &str = "<ResponseMetadata><RequestId>e1a2b3c4-d5e6-7f89-0123-456789abcdef</RequestId></ResponseMetadata>";

#[tokio::test]
async fn test_attach_group_policy() {
    let body = format!(
        r#"<?xml version="1.0"?><AttachGroupPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">{REQUEST_ID}</AttachGroupPolicyResponse>"#
    );
    let client = mock_client(&body);

    let result = attach_group_policy(
        &client,
        "TestGroup".to_string(),
        "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess".to_string(),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_create_group() {
    let body = format!(
        r#"<?xml version="1.0"?><CreateGroupResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
            <CreateGroupResult>
                <Group>
                    <Path>/</Path>
                    <GroupName>TestGroup</GroupName>
                    <GroupId>AGPA1234567890EXAMPLE</GroupId>
                    <Arn>arn:aws:iam::123456789012:group/TestGroup</Arn>
                    <CreateDate>2026-01-01T00:00:00Z</CreateDate>
                </Group>
            </CreateGroupResult>
            {REQUEST_ID}
        </CreateGroupResponse>"#
    );
    let client = mock_client(&body);

    let result = create_group(&client, "/".to_string(), "TestGroup".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_group() {
    let body = format!(
        r#"<?xml version="1.0"?><DeleteGroupResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">{REQUEST_ID}</DeleteGroupResponse>"#
    );
    let client = mock_client(&body);

    let result = delete_group(&client, "TestGroup".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_detach_group_policy() {
    let body = format!(
        r#"<?xml version="1.0"?><DetachGroupPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">{REQUEST_ID}</DetachGroupPolicyResponse>"#
    );
    let client = mock_client(&body);

    let result = detach_group_policy(
        &client,
        "TestGroup".to_string(),
        "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess".to_string(),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_describe_group() {
    let body = format!(
        r#"<?xml version="1.0"?><GetGroupResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
            <GetGroupResult>
                <Group>
                    <Path>/</Path>
                    <GroupName>TestGroup</GroupName>
                    <GroupId>AGPA1234567890EXAMPLE</GroupId>
                    <Arn>arn:aws:iam::123456789012:group/TestGroup</Arn>
                    <CreateDate>2026-01-01T00:00:00Z</CreateDate>
                </Group>
                <Users/>
                <IsTruncated>false</IsTruncated>
            </GetGroupResult>
            {REQUEST_ID}
        </GetGroupResponse>"#
    );
    let client = mock_client(&body);

    let result = describe(&client, "TestGroup".to_string(), None, Some(100)).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_create_user() {
    let body = format!(
        r#"<?xml version="1.0"?><CreateUserResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
            <CreateUserResult>
                <User>
                    <Path>/</Path>
                    <UserName>TestUser</UserName>
                    <UserId>AIDA1234567890EXAMPLE</UserId>
                    <Arn>arn:aws:iam::123456789012:user/TestUser</Arn>
                    <CreateDate>2026-01-01T00:00:00Z</CreateDate>
                </User>
            </CreateUserResult>
            {REQUEST_ID}
        </CreateUserResponse>"#
    );
    let client = mock_client(&body);

    let result = create_user(&client, "TestUser".to_string(), Some("/".to_string()), None).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_user() {
    let body = format!(
        r#"<?xml version="1.0"?><DeleteUserResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">{REQUEST_ID}</DeleteUserResponse>"#
    );
    let client = mock_client(&body);

    let result = delete_user(&client, "TestUser".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_list_users() {
    let body = format!(
        r#"<?xml version="1.0"?><ListUsersResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
            <ListUsersResult>
                <Users>
                    <member>
                        <Path>/</Path>
                        <UserName>TestUser</UserName>
                        <UserId>AIDA1234567890EXAMPLE</UserId>
                        <Arn>arn:aws:iam::123456789012:user/TestUser</Arn>
                        <CreateDate>2026-01-01T00:00:00Z</CreateDate>
                    </member>
                </Users>
                <IsTruncated>false</IsTruncated>
            </ListUsersResult>
            {REQUEST_ID}
        </ListUsersResponse>"#
    );
    let client = mock_client(&body);

    let result = list_users(&client, None, None, Some(50)).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_attach_user_policy() {
    let body = format!(
        r#"<?xml version="1.0"?><AttachUserPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">{REQUEST_ID}</AttachUserPolicyResponse>"#
    );
    let client = mock_client(&body);

    let result = attach_user_policy(
        &client,
        "TestUser".to_string(),
        "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess".to_string(),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_detach_user_policy() {
    let body = format!(
        r#"<?xml version="1.0"?><DetachUserPolicyResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">{REQUEST_ID}</DetachUserPolicyResponse>"#
    );
    let client = mock_client(&body);

    let result = detach_user_policy(
        &client,
        "TestUser".to_string(),
        "arn:aws:iam::aws:policy/AmazonS3ReadOnlyAccess".to_string(),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_create_role() {
    let body = format!(
        r#"<?xml version="1.0"?><CreateRoleResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
            <CreateRoleResult>
                <Role>
                    <Path>/</Path>
                    <RoleName>TestRole</RoleName>
                    <RoleId>AROA1234567890EXAMPLE</RoleId>
                    <Arn>arn:aws:iam::123456789012:role/TestRole</Arn>
                    <CreateDate>2026-01-01T00:00:00Z</CreateDate>
                </Role>
            </CreateRoleResult>
            {REQUEST_ID}
        </CreateRoleResponse>"#
    );
    let client = mock_client(&body);

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
        "TestRole".to_string(),
        assume_role_policy_document,
        Some("/".to_string()),
        Some("Test role created by RustCloud".to_string()),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_role() {
    let body = format!(
        r#"<?xml version="1.0"?><DeleteRoleResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">{REQUEST_ID}</DeleteRoleResponse>"#
    );
    let client = mock_client(&body);

    let result = delete_role(&client, "TestRole".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_list_roles() {
    let body = format!(
        r#"<?xml version="1.0"?><ListRolesResponse xmlns="https://iam.amazonaws.com/doc/2010-05-08/">
            <ListRolesResult>
                <Roles>
                    <member>
                        <Path>/</Path>
                        <RoleName>TestRole</RoleName>
                        <RoleId>AROA1234567890EXAMPLE</RoleId>
                        <Arn>arn:aws:iam::123456789012:role/TestRole</Arn>
                        <CreateDate>2026-01-01T00:00:00Z</CreateDate>
                    </member>
                </Roles>
                <IsTruncated>false</IsTruncated>
            </ListRolesResult>
            {REQUEST_ID}
        </ListRolesResponse>"#
    );
    let client = mock_client(&body);

    let result = list_roles(&client, None, None, Some(50)).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_list_users_live() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let result = list_users(&client, None, None, Some(50)).await;

    assert!(result.is_ok(), "{:?}", result);
}
