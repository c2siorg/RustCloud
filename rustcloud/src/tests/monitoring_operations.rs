use crate::aws::aws_apis::management::aws_monitoring::*;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_cloudwatch::Client;
use aws_sdk_cloudwatch::types::{ComparisonOperator, Statistic, MetricDataQuery, MetricStat, Metric, ScanBy};
use aws_sdk_ec2::primitives::DateTime;
use std::collections::HashMap;

async fn get_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&config)
}

#[tokio::test]
async fn test_delete_alarm() {
    let client = get_client().await;
    let alarm_name = "test-alarm".to_string();
    let result = delete_alarm(&client, &alarm_name).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_get_metric_data() {
    let client = get_client().await;
    
    let metric_data_queries = Some(vec![
        MetricDataQuery::builder()
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
            .build(),
    ]);
    let start_time = Some(DateTime::from_secs(1625155200)); // Example timestamp
    let end_time = Some(DateTime::from_secs(1625241600)); // Example timestamp
    let next_token = None;
    let scan_by = Some(ScanBy::TimestampDescending);
    let _max_datapoints = None;
    let label_options = None;

    let result = get_metric_data(
        &client,
        metric_data_queries,
        start_time,
        end_time,
        next_token,
        scan_by,
        _max_datapoints,
        label_options,
    ).await;

    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_alarms() {
    let client = get_client().await;
    let result = list_alarms(&client).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_put_metric_alarm() {
    let client = get_client().await;

    let alarm_name = "test-alarm".to_string();
    let alarm_description = Some("This is a test alarm.".to_string());
    let alarm_actions = Some(vec!["arn:aws:sns:us-east-1:123456789012:my-sns-topic".to_string()]);
    let comparison_operator = Some(ComparisonOperator::GreaterThanThreshold);
    let evaluation_periods = Some(1);
    let metric_name = Some("CPUUtilization".to_string());
    let namespace = Some("AWS/EC2".to_string());
    let period = Some(60);
    let statistic = Some(Statistic::Average);
    let threshold = Some(80.0);
    let treat_missing_data = Some("missing".to_string());

    let result = put_metric_alarm(
        &client,
        &alarm_name,
        alarm_description,
        alarm_actions,
        comparison_operator,
        evaluation_periods,
        metric_name,
        namespace,
        period,
        statistic,
        threshold,
        treat_missing_data,
    ).await;

    assert!(result.is_ok());
}