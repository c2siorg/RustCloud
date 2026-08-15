use aws_sdk_elasticloadbalancing::config::{
    BehaviorVersion, Credentials, Region, SharedCredentialsProvider,
};
use aws_sdk_elasticloadbalancing::types::Tag;
use aws_sdk_elasticloadbalancing::{Client, Config};
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;

use crate::aws::aws_apis::network::aws_loadbalancer::*;

fn mock_client(response_body: &str) -> Client {
    let request = http::Request::builder()
        .method("POST")
        .uri("https://elasticloadbalancing.us-east-1.amazonaws.com/")
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
async fn test_add_tags_to_loadbalancer() {
    let body = format!(
        r#"<?xml version="1.0"?><AddTagsResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/"><AddTagsResult/>{REQUEST_ID}</AddTagsResponse>"#
    );
    let client = mock_client(&body);
    let tag = Tag::builder()
        .key("Environment".to_string())
        .value("Production".to_string())
        .build()
        .unwrap();

    let result = add_tags(&client, "my-load-balancer".to_string(), tag).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_create_loadbalancer() {
    let body = format!(
        r#"<?xml version="1.0"?><CreateLoadBalancerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
            <CreateLoadBalancerResult>
                <DNSName>my-load-balancer-1234567890.us-east-1.elb.amazonaws.com</DNSName>
            </CreateLoadBalancerResult>
            {REQUEST_ID}
        </CreateLoadBalancerResponse>"#
    );
    let client = mock_client(&body);

    let result = create(
        &client,
        "my-load-balancer".to_string(),
        None,
        None,
        None,
        None,
        None,
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_loadbalancer() {
    let body = format!(
        r#"<?xml version="1.0"?><DeleteLoadBalancerResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/"><DeleteLoadBalancerResult/>{REQUEST_ID}</DeleteLoadBalancerResponse>"#
    );
    let client = mock_client(&body);

    let result = delete(&client, "my-load-balancer".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_describe_loadbalancer() {
    let body = format!(
        r#"<?xml version="1.0"?><DescribeLoadBalancerAttributesResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
            <DescribeLoadBalancerAttributesResult>
                <LoadBalancerAttributes>
                    <CrossZoneLoadBalancing><Enabled>true</Enabled></CrossZoneLoadBalancing>
                </LoadBalancerAttributes>
            </DescribeLoadBalancerAttributesResult>
            {REQUEST_ID}
        </DescribeLoadBalancerAttributesResponse>"#
    );
    let client = mock_client(&body);

    let result = describe(&client, "my-load-balancer".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_list_load_balancers() {
    let body = format!(
        r#"<?xml version="1.0"?><DescribeLoadBalancersResponse xmlns="http://elasticloadbalancing.amazonaws.com/doc/2012-06-01/">
            <DescribeLoadBalancersResult>
                <LoadBalancerDescriptions>
                    <member>
                        <LoadBalancerName>my-load-balancer</LoadBalancerName>
                    </member>
                </LoadBalancerDescriptions>
            </DescribeLoadBalancersResult>
            {REQUEST_ID}
        </DescribeLoadBalancersResponse>"#
    );
    let client = mock_client(&body);

    let result = list_load_balancers(&client, None, None, Some(10)).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_list_load_balancers_live() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let result = list_load_balancers(&client, None, None, Some(10)).await;

    assert!(result.is_ok(), "{:?}", result);
}
