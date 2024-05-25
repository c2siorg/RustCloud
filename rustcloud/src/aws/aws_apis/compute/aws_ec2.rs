#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{config::Region, meta::PKG_VERSION, types::Tag, Client, Error};
// use clap::Parser;

async fn create_instance(client: &Client, ami_id: &str) -> Result<(), Error> {
    let run_instances = client
        .run_instances()
        .image_id(ami_id)
        .instance_type(aws_sdk_ec2::types::InstanceType::T1Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await?;

    if run_instances.instances().is_empty() {
        panic!("No instances created.");
    }

    println!("Created instance.");

    let instance_id = run_instances.instances()[0].instance_id().unwrap();
    client
        .create_tags()
        .resources(instance_id)
        .tags(
            Tag::builder()
                .key("Name")
                .value("From SDK Examples")
                .build(),
        )
        .send()
        .await?;

    println!("Created {instance_id} and applied tags.",);

    Ok(())
}

async fn show_state(client: &Client, ids: Option<Vec<String>>) -> Result<(), Error> {
    let resp = client
        .describe_instances()
        .set_instance_ids(ids)
        .send()
        .await?;

    for reservation in resp.reservations() {
        for instance in reservation.instances() {
            println!("Instance ID: {}", instance.instance_id().unwrap());
            println!(
                "State:       {:?}",
                instance.state().unwrap().name().unwrap()
            );
            println!();
        }
    }

    Ok(())
}

async fn show_all_events(client: &Client) -> Result<(), Error> {
    let resp = client.describe_regions().send().await.unwrap();

    for region in resp.regions.unwrap_or_default() {
        let reg: &'static str = Box::leak(Box::from(region.region_name().unwrap()));
        let region_provider = RegionProviderChain::default_provider().or_else(reg);
        let config = aws_config::from_env().region(region_provider).load().await;
        let new_client = Client::new(&config);

        let resp = new_client.describe_instance_status().send().await;

        println!("Instances in region {}:", reg);
        println!();

        for status in resp.unwrap().instance_statuses() {
            println!(
                "  Events scheduled for instance ID: {}",
                status.instance_id().unwrap_or_default()
            );
            for event in status.events() {
                println!("    Event ID:     {}", event.instance_event_id().unwrap());
                println!("    Description:  {}", event.description().unwrap());
                println!("    Event code:   {}", event.code().unwrap().as_ref());
                println!();
            }
        }
    }

    Ok(())
}

async fn enable_monitoring(client: &Client, id: &str) -> Result<(), Error> {
    client.monitor_instances().instance_ids(id).send().await?;

    println!("Enabled monitoring");

    Ok(())
}

async fn reboot_instance(client: &Client, id: &str) -> Result<(), Error> {
    client.reboot_instances().instance_ids(id).send().await?;

    println!("Rebooted instance.");
    Ok(())
}

async fn start_instance(client: &Client, id: &str) -> Result<(), Error> {
    client.start_instances().instance_ids(id).send().await?;

    println!("Started instance.");

    Ok(())
}

async fn stop_instance(client: &Client, id: &str) -> Result<(), Error> {
    client.stop_instances().instance_ids(id).send().await?;

    println!("Stopped instance.");

    Ok(())
}


#[cfg(test)]
#[test]
fn test_create_instance() {
    let config = aws_config::from_env().load().await.unwrap();
    let client = Client::new(&config);
    create_instance(&client, "ami-00000000000000000").await.unwrap();
}