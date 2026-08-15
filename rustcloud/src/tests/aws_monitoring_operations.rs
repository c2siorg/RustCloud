use aws_sdk_cloudwatch::config::{BehaviorVersion, Credentials, Region, SharedCredentialsProvider};
use aws_sdk_cloudwatch::types::{
    ComparisonOperator, Metric, MetricDataQuery, MetricStat, ScanBy, Statistic,
};
use aws_sdk_cloudwatch::{Client, Config};
use aws_sdk_ec2::primitives::DateTime;
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;

use crate::aws::aws_apis::management::aws_monitoring::*;

// CloudWatch responses are RPCv2 CBOR here, not JSON/XML like the other legacy services.
// 0xA0 is an empty CBOR map, valid for every op below since their output fields are optional.
const EMPTY_CBOR_MAP: &[u8] = &[0xA0];

fn mock_client(response_body: &[u8]) -> Client {
    let request = http::Request::builder()
        .method("POST")
        .uri("https://monitoring.us-east-1.amazonaws.com/")
        .body(SdkBody::empty())
        .unwrap();
    let response = http::Response::builder()
        .status(200)
        .header("content-type", "application/cbor")
        .header("smithy-protocol", "rpc-v2-cbor")
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

#[tokio::test]
async fn test_delete_alarm() {
    let client = mock_client(EMPTY_CBOR_MAP);

    let result = delete_alarm(&client, &"test-alarm".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_get_metric_data() {
    let client = mock_client(EMPTY_CBOR_MAP);

    let metric_data_queries = Some(vec![MetricDataQuery::builder()
        .id("test-query".to_string())
        .metric_stat(
            MetricStat::builder()
                .metric(
                    Metric::builder()
                        .namespace("AWS/EC2")
                        .metric_name("CPUUtilization")
                        .build(),
                )
                .period(60)
                .build(),
        )
        .return_data(true)
        .build()]);

    let result = get_metric_data(
        &client,
        metric_data_queries,
        Some(DateTime::from_secs(1625155200)),
        Some(DateTime::from_secs(1625241600)),
        None,
        Some(ScanBy::TimestampDescending),
        None,
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_list_alarms() {
    let client = mock_client(EMPTY_CBOR_MAP);

    let result = list_alarms(&client).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_put_metric_alarm() {
    let client = mock_client(EMPTY_CBOR_MAP);

    let result = put_metric_alarm(
        &client,
        &"test-alarm".to_string(),
        Some("This is a test alarm.".to_string()),
        None,
        Some(ComparisonOperator::GreaterThanThreshold),
        Some(1),
        Some("CPUUtilization".to_string()),
        Some("AWS/EC2".to_string()),
        Some(60),
        Some(Statistic::Average),
        Some(80.0),
        Some("missing".to_string()),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_list_alarms_live() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let result = list_alarms(&client).await;

    assert!(result.is_ok(), "{:?}", result);    
}
