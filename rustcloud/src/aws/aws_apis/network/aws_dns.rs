#![allow(clippy::result_large_err)]

use std::collections::HashMap;
use tokio;
use aws_sdk_route53::{operation::{change_resource_record_sets::ChangeResourceRecordSetsInput, create_hosted_zone::CreateHostedZoneInput, delete_hosted_zone::DeleteHostedZoneInput, list_hosted_zones::ListHostedZonesInput}, types::{ChangeBatch, HostedZoneConfig, HostedZoneType, Vpc}, Client, Error};


async fn change_record_sets(client: &Client, hosted_zone_id: String, change_batch: ChangeBatch) -> Result<(), Error> {
    let change_record_sets = client.change_resource_record_sets().hosted_zone_id(hosted_zone_id).change_batch(change_batch).send().await?;
    println!("{:?}", change_record_sets);
    Ok(())
    
}

async fn create_zone(client: &Client, name: String, vpc: Vpc, caller_reference: String,hosted_zone_config: Option<HostedZoneConfig>,delegation_set_id: Option<String> ) -> Result<(), Error> {
    let create_zone = client.create_hosted_zone().name(name).vpc(vpc).caller_reference(caller_reference).set_hosted_zone_config(hosted_zone_config).set_delegation_set_id(delegation_set_id).send().await?;
    println!("{:?}", create_zone);
    Ok(())
    
}
async fn delete_zone(client: &Client, id: String) -> Result<(), Error> {
    let delete_zone = client.delete_hosted_zone().id(id).send().await?;
    println!("{:?}", delete_zone);
    Ok(())
}

async fn list_zones(client: &Client, marker: Option<String>,max_items: Option<i32>,delegation_set_id: Option<String>,hosted_zone_type:Option<HostedZoneType>) -> Result<(), Error> {
    let list_zones = client.list_hosted_zones().set_marker(marker).set_max_items(max_items).set_hosted_zone_type(hosted_zone_type).set_delegation_set_id(delegation_set_id).send().await?;
    println!("{:?}", list_zones);
    Ok(())
}