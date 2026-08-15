use aws_sdk_ec2::config::{BehaviorVersion, Credentials, Region, SharedCredentialsProvider};
use aws_sdk_ec2::{Client, Config};
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;

use crate::aws::aws_apis::compute::aws_ec2::*;

fn mock_client(response_bodies: &[&str]) -> Client {
    let events = response_bodies
        .iter()
        .map(|body| {
            let request = http::Request::builder()
                .method("POST")
                .uri("https://ec2.us-east-1.amazonaws.com/")
                .body(SdkBody::empty())
                .unwrap();
            let response = http::Response::builder()
                .status(200)
                .header("content-type", "text/xml")
                .body(SdkBody::from(*body))
                .unwrap();
            ReplayEvent::new(request, response)
        })
        .collect();
    let http_client = StaticReplayClient::new(events);

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

const REQUEST_ID: &str = "<requestId>e1a2b3c4-d5e6-7f89-0123-456789abcdef</requestId>";
const INSTANCE_ID: &str = "i-1234567890abcdef0";

fn run_instances_response() -> String {
    format!(
        r#"<?xml version="1.0"?><RunInstancesResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">
            {REQUEST_ID}
            <reservationId>r-1234567890abcdef0</reservationId>
            <ownerId>123456789012</ownerId>
            <instancesSet>
                <item>
                    <instanceId>{INSTANCE_ID}</instanceId>
                    <imageId>ami-0aff18ec83b712f05</imageId>
                    <instanceState><code>0</code><name>pending</name></instanceState>
                    <instanceType>t1.micro</instanceType>
                    <launchTime>2026-01-01T00:00:00.000Z</launchTime>
                    <placement><availabilityZone>us-east-1a</availabilityZone></placement>
                </item>
            </instancesSet>
        </RunInstancesResponse>"#
    )
}

fn create_tags_response() -> String {
    format!(
        r#"<?xml version="1.0"?><CreateTagsResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">{REQUEST_ID}<return>true</return></CreateTagsResponse>"#
    )
}

fn terminate_instances_response() -> String {
    format!(
        r#"<?xml version="1.0"?><TerminateInstancesResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">
            {REQUEST_ID}
            <instancesSet>
                <item>
                    <instanceId>{INSTANCE_ID}</instanceId>
                    <currentState><code>32</code><name>shutting-down</name></currentState>
                    <previousState><code>16</code><name>running</name></previousState>
                </item>
            </instancesSet>
        </TerminateInstancesResponse>"#
    )
}

fn describe_instances_response(code: u32, name: &str) -> String {
    format!(
        r#"<?xml version="1.0"?><DescribeInstancesResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">
            {REQUEST_ID}
            <reservationSet>
                <item>
                    <reservationId>r-1234567890abcdef0</reservationId>
                    <ownerId>123456789012</ownerId>
                    <instancesSet>
                        <item>
                            <instanceId>{INSTANCE_ID}</instanceId>
                            <imageId>ami-0aff18ec83b712f05</imageId>
                            <instanceState><code>{code}</code><name>{name}</name></instanceState>
                            <instanceType>t1.micro</instanceType>
                            <launchTime>2026-01-01T00:00:00.000Z</launchTime>
                            <placement><availabilityZone>us-east-1a</availabilityZone></placement>
                        </item>
                    </instancesSet>
                </item>
            </reservationSet>
        </DescribeInstancesResponse>"#
    )
}

fn monitor_instances_response() -> String {
    format!(
        r#"<?xml version="1.0"?><MonitorInstancesResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">
            {REQUEST_ID}
            <instancesSet>
                <item><instanceId>{INSTANCE_ID}</instanceId><monitoring><state>pending</state></monitoring></item>
            </instancesSet>
        </MonitorInstancesResponse>"#
    )
}

fn reboot_instances_response() -> String {
    format!(
        r#"<?xml version="1.0"?><RebootInstancesResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">{REQUEST_ID}<return>true</return></RebootInstancesResponse>"#
    )
}

fn stop_instances_response() -> String {
    format!(
        r#"<?xml version="1.0"?><StopInstancesResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">
            {REQUEST_ID}
            <instancesSet>
                <item>
                    <instanceId>{INSTANCE_ID}</instanceId>
                    <currentState><code>64</code><name>stopping</name></currentState>
                    <previousState><code>16</code><name>running</name></previousState>
                </item>
            </instancesSet>
        </StopInstancesResponse>"#
    )
}

fn start_instances_response() -> String {
    format!(
        r#"<?xml version="1.0"?><StartInstancesResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">
            {REQUEST_ID}
            <instancesSet>
                <item>
                    <instanceId>{INSTANCE_ID}</instanceId>
                    <currentState><code>0</code><name>pending</name></currentState>
                    <previousState><code>80</code><name>stopped</name></previousState>
                </item>
            </instancesSet>
        </StartInstancesResponse>"#
    )
}

// Single assertion, not the original's poll loop — the mock has nothing to converge on.
async fn assert_instance_state(client: &Client, instance_id: &str, target_state: &str) {
    let resp = client
        .describe_instances()
        .instance_ids(instance_id)
        .send()
        .await
        .expect("describe_instances failed");

    let state = resp
        .reservations()
        .first()
        .and_then(|r| r.instances().first())
        .and_then(|i| i.state())
        .and_then(|s| s.name())
        .map(|n| n.as_str())
        .unwrap_or("");

    assert_eq!(state, target_state, "mock did not return the expected state");
}

#[tokio::test]
async fn test_create_instance() {
    let client = mock_client(&[
        &run_instances_response(),
        &create_tags_response(),
        &terminate_instances_response(),
    ]);

    let result = create_instance(&client, "ami-0aff18ec83b712f05").await;
    if let Ok(ref id) = result {
        terminate_instance(&client, id).await.ok();
    }

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_show_state() {
    let client = mock_client(&[
        &run_instances_response(),
        &create_tags_response(),
        &describe_instances_response(16, "running"),
        &terminate_instances_response(),
    ]);

    let instance_id = create_instance(&client, "ami-0aff18ec83b712f05")
        .await
        .expect("Failed to create test instance");
    let result = show_state(&client, Some(vec![instance_id.clone()])).await;
    terminate_instance(&client, &instance_id).await.ok();

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_show_all_events() {
    // Empty region list: show_all_events builds its own client per region with no injection
    // seam, so that branch can't be mocked here.
    let body = format!(
        r#"<?xml version="1.0"?><DescribeRegionsResponse xmlns="http://ec2.amazonaws.com/doc/2016-11-15/">{REQUEST_ID}<regionInfo/></DescribeRegionsResponse>"#
    );
    let client = mock_client(&[&body]);

    let result = show_all_events(&client).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_enable_monitoring() {
    let client = mock_client(&[
        &run_instances_response(),
        &create_tags_response(),
        &monitor_instances_response(),
        &terminate_instances_response(),
    ]);

    let instance_id = create_instance(&client, "ami-0aff18ec83b712f05")
        .await
        .expect("Failed to create test instance");
    let result = enable_monitoring(&client, &instance_id).await;
    terminate_instance(&client, &instance_id).await.ok();

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_reboot_instance() {
    let client = mock_client(&[
        &run_instances_response(),
        &create_tags_response(),
        &describe_instances_response(16, "running"),
        &reboot_instances_response(),
        &terminate_instances_response(),
    ]);

    let instance_id = create_instance(&client, "ami-0aff18ec83b712f05")
        .await
        .expect("Failed to create test instance");
    assert_instance_state(&client, &instance_id, "running").await;
    let result = reboot_instance(&client, &instance_id).await;
    terminate_instance(&client, &instance_id).await.ok();

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_start_instance() {
    let client = mock_client(&[
        &run_instances_response(),
        &create_tags_response(),
        &describe_instances_response(16, "running"),
        &stop_instances_response(),
        &describe_instances_response(80, "stopped"),
        &start_instances_response(),
        &terminate_instances_response(),
    ]);

    let instance_id = create_instance(&client, "ami-0aff18ec83b712f05")
        .await
        .expect("Failed to create test instance");
    assert_instance_state(&client, &instance_id, "running").await;
    stop_instance(&client, &instance_id)
        .await
        .expect("stop_instance failed");
    assert_instance_state(&client, &instance_id, "stopped").await;
    let result = start_instance(&client, &instance_id).await;
    terminate_instance(&client, &instance_id).await.ok();

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_stop_instance() {
    let client = mock_client(&[
        &run_instances_response(),
        &create_tags_response(),
        &describe_instances_response(16, "running"),
        &stop_instances_response(),
        &terminate_instances_response(),
    ]);

    let instance_id = create_instance(&client, "ami-0aff18ec83b712f05")
        .await
        .expect("Failed to create test instance");
    assert_instance_state(&client, &instance_id, "running").await;
    let result = stop_instance(&client, &instance_id).await;
    terminate_instance(&client, &instance_id).await.ok();

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_create_instance_live() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);
    let ami_id = std::env::var("TEST_AMI_ID").unwrap_or_else(|_| "ami-0aff18ec83b712f05".to_string());

    let result = create_instance(&client, &ami_id).await;
    if let Ok(ref id) = result {
        terminate_instance(&client, id).await.ok();
    }

    assert!(result.is_ok(), "{:?}", result);
}
