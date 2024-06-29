use crate::aws::aws_apis::network::aws_dns::*;
use aws_config::meta::region::RegionProviderChain;
use aws_sdk_route53::{config::Config, types::{builders::ResourceRecordSetBuilder, Change, ChangeAction, ResourceRecord, ResourceRecordSet, RrType, Vpc, VpcRegion}, Client};
use aws_sdk_route53::types::{ChangeBatch, HostedZoneConfig, HostedZoneType};

async fn get_client() -> Client {
    let region_provider = RegionProviderChain::default_provider().or_else("us-east-1");
    let config = aws_config::from_env().region(region_provider).load().await;
    Client::new(&config)
}

#[tokio::test]
    async fn test_change_record_sets() {
        let client = get_client().await;

        let hosted_zone_id = "your_hosted_zone_id".to_string(); // Replace with your hosted zone ID

        // Build ResourceRecord
        let resource_record = ResourceRecord::builder().value("192.0.2.44".to_string()).build().unwrap();
        // Build ResourceRecordSet
        let resource_record_set = ResourceRecordSetBuilder::default()
            .name("test.example.com.".to_string())
            .r#type(RrType::A)
            .ttl(60)
            .resource_records(resource_record)
            .build()
            .expect("Failed to build ResourceRecordSet");

        // Build Change
        let change = Change::builder()
            .action(ChangeAction::Upsert)
            .resource_record_set(resource_record_set.clone()) // Use clone() if needed
            .build().unwrap();

        // Build ChangeBatch
        let change_batch = ChangeBatch::builder()
            .changes(change)
            .build().unwrap();

        let result = change_record_sets(&client, hosted_zone_id, change_batch).await;
        assert!(result.is_ok());
    }

#[tokio::test]
async fn test_create_zone() {
    let client = get_client().await;

    let name = "example.com".to_string(); // Replace with your desired domain name
    let vpc = Vpc::builder()
        .vpc_region(VpcRegion::UsEast1)
        .vpc_id("vpc-1a2b3c4d") // Replace with your VPC ID
        .build();
    let caller_reference = "unique-string".to_string();
    let hosted_zone_config = Some(
        HostedZoneConfig::builder()
            .comment("Test hosted zone")
            .build(),
    );
    let delegation_set_id = None;

    let result = create_zone(&client, name, vpc, caller_reference, hosted_zone_config, delegation_set_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_zone() {
    let client = get_client().await;

    let hosted_zone_id = "Z3AADJGX6KTTL2".to_string(); // Replace with your hosted zone ID

    let result = delete_zone(&client, hosted_zone_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_zones() {
    let client = get_client().await;

    let marker = None;
    let max_items = Some(10);
    let delegation_set_id = None;
    let hosted_zone_type = Some(HostedZoneType::PrivateHostedZone);

    let result = list_zones(&client, marker, max_items, delegation_set_id, hosted_zone_type).await;
    assert!(result.is_ok());
}