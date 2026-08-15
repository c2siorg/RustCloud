use aws_sdk_route53::config::{BehaviorVersion, Credentials, Region, SharedCredentialsProvider};
use aws_sdk_route53::types::{
    Change, ChangeAction, ChangeBatch, HostedZoneConfig, HostedZoneType, ResourceRecord,
    ResourceRecordSet, RrType, Vpc, VpcRegion,
};
use aws_sdk_route53::{Client, Config};
use aws_smithy_http_client::test_util::{ReplayEvent, StaticReplayClient};
use aws_smithy_types::body::SdkBody;

use crate::aws::aws_apis::network::aws_dns::*;

fn mock_client(response_body: &str) -> Client {
    let request = http::Request::builder()
        .method("POST")
        .uri("https://route53.amazonaws.com/2013-04-01/hostedzone")
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

const CHANGE_INFO: &str = "\
    <ChangeInfo>\
        <Id>/change/C1PA6795UKMFR9</Id>\
        <Status>PENDING</Status>\
        <SubmittedAt>2026-01-01T00:00:00.000Z</SubmittedAt>\
    </ChangeInfo>";

#[tokio::test]
async fn test_change_record_sets() {
    let body = format!(
        r#"<?xml version="1.0"?><ChangeResourceRecordSetsResponse xmlns="https://route53.amazonaws.com/doc/2013-04-01/">{CHANGE_INFO}</ChangeResourceRecordSetsResponse>"#
    );
    let client = mock_client(&body);

    let hosted_zone_id = "Z1PA6795UKMFR9".to_string();
    let resource_record = ResourceRecord::builder()
        .value("192.0.2.44")
        .build()
        .unwrap();
    let resource_record_set = ResourceRecordSet::builder()
        .name("test.example.com.")
        .r#type(RrType::A)
        .ttl(60)
        .resource_records(resource_record)
        .build()
        .unwrap();
    let change = Change::builder()
        .action(ChangeAction::Upsert)
        .resource_record_set(resource_record_set)
        .build()
        .unwrap();
    let change_batch = ChangeBatch::builder().changes(change).build().unwrap();

    let result = change_record_sets(&client, hosted_zone_id, change_batch).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_create_zone() {
    let body = format!(
        r#"<?xml version="1.0"?><CreateHostedZoneResponse xmlns="https://route53.amazonaws.com/doc/2013-04-01/">
            <HostedZone>
                <Id>/hostedzone/Z1PA6795UKMFR9</Id>
                <Name>example.com.</Name>
                <CallerReference>unique-string</CallerReference>
            </HostedZone>
            {CHANGE_INFO}
            <DelegationSet>
                <NameServers>
                    <NameServer>ns-1.awsdns-01.com</NameServer>
                </NameServers>
            </DelegationSet>
        </CreateHostedZoneResponse>"#
    );
    let client = mock_client(&body);

    let vpc = Vpc::builder()
        .vpc_region(VpcRegion::UsEast1)
        .vpc_id("vpc-1a2b3c4d")
        .build();
    let hosted_zone_config = Some(
        HostedZoneConfig::builder()
            .comment("Test hosted zone")
            .build(),
    );

    let result = create_zone(
        &client,
        "example.com".to_string(),
        vpc,
        "unique-string".to_string(),
        hosted_zone_config,
        None,
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_delete_zone() {
    let body = format!(
        r#"<?xml version="1.0"?><DeleteHostedZoneResponse xmlns="https://route53.amazonaws.com/doc/2013-04-01/">{CHANGE_INFO}</DeleteHostedZoneResponse>"#
    );
    let client = mock_client(&body);

    let result = delete_zone(&client, "Z1PA6795UKMFR9".to_string()).await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
async fn test_list_zones() {
    let body = r#"<?xml version="1.0"?><ListHostedZonesResponse xmlns="https://route53.amazonaws.com/doc/2013-04-01/">
        <HostedZones>
            <HostedZone>
                <Id>/hostedzone/Z1PA6795UKMFR9</Id>
                <Name>example.com.</Name>
                <CallerReference>unique-string</CallerReference>
            </HostedZone>
        </HostedZones>
        <IsTruncated>false</IsTruncated>
        <MaxItems>10</MaxItems>
    </ListHostedZonesResponse>"#;
    let client = mock_client(body);

    let result = list_zones(
        &client,
        None,
        Some(10),
        None,
        Some(HostedZoneType::PrivateHostedZone),
    )
    .await;

    assert!(result.is_ok(), "{:?}", result);
}

#[tokio::test]
#[ignore]
async fn test_list_zones_live() {
    let config = aws_config::load_from_env().await;
    let client = Client::new(&config);

    let result = list_zones(&client, None, Some(10), None, None).await;

    assert!(result.is_ok(), "{:?}", result);
}
