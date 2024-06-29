#![allow(clippy::result_large_err)]

use aws_sdk_ec2::primitives::DateTime;
use aws_sdk_cloudwatch::{types::{ComparisonOperator, LabelOptions, MetricDataQuery, ScanBy, Statistic}, Client, Error};



pub async fn delete_alarm(client: &Client, alarm_name: &String) -> Result<(), Error> {
    let delete_alarm = client.delete_alarms().alarm_names(alarm_name).send().await?;
    println!("{:?}", delete_alarm);
    Ok(())
}


pub async fn get_metric_data(client: &Client, metric_data_queries:Option<Vec<MetricDataQuery>>,start_time: Option<DateTime>,end_time: Option<DateTime>,next_token: Option<String>,scan_by: Option<ScanBy>,  _max_datapoints: Option<i32>,label_options: Option<LabelOptions>) -> Result<(), Error> {
    
    let get_metric_data = client.get_metric_data().set_metric_data_queries(metric_data_queries).set_start_time(start_time).set_end_time(end_time).set_next_token(next_token).set_scan_by(scan_by).set_label_options(label_options).send().await?;
    println!("{:?}", get_metric_data);

    Ok(())
}

pub async fn list_alarms(client: &Client) -> Result<(),Error> {
    let list_alarms = client.list_metrics().send().await?;
    println!("{:?}", list_alarms);
    Ok(())
}

pub async fn put_metric_alarm(client: &Client, alarm_name: &String, alarm_description: Option<String>, alarm_actions: Option<Vec<String>>, comparison_operator: Option<ComparisonOperator>, evaluation_periods: Option<i32>, metric_name: Option<String>, namespace: Option<String>, period: Option<i32>, statistic: Option<Statistic>, threshold: Option<f64>, treat_missing_data: Option<String>) -> Result<(), Error> {
    let put_metric_alarm = client.put_metric_alarm().alarm_name(alarm_name).set_alarm_description(alarm_description).set_alarm_actions(alarm_actions).set_comparison_operator(comparison_operator).set_evaluation_periods(evaluation_periods).set_metric_name(metric_name).set_namespace(namespace).set_period(period).set_statistic(statistic).set_threshold(threshold).set_treat_missing_data(treat_missing_data).send().await?;

    println!("{:?}", put_metric_alarm);


    Ok(())
}

