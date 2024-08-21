#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{operation::run_instances::RunInstancesError, types::Tag, Client, Error};

// use clap::Parser;

pub async fn create_instance(client: &Client, ami_id: &str) -> Result<(), Error> {
    let run_instances = client
        .run_instances()
        .image_id(ami_id)
        .instance_type(aws_sdk_ec2::types::InstanceType::T1Micro)
        .min_count(1)
        .max_count(1)
        .send()
        .await;

    match run_instances {
        Ok(run_instances) => {
            if run_instances.instances().is_empty() {
                panic!("No instances created.");
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
                    .await
                    .unwrap();

                println!("Created {instance_id} and applied tags.",);
            }
            Ok(())
        }
        Err(err) => {
            println!("Error: {:?}", err);
            Err(err.into())
        }
    }
}
pub async fn show_state(client: &Client, ids: Option<Vec<String>>) -> Result<(), Error> {
    let resp = client
        .describe_instances()
        .set_instance_ids(ids)
        .send()
        .await;

    match resp {
        Ok(result) => {
            for reservation in result.reservations() {
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
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn show_all_events(client: &Client) -> Result<(), Error> {
    let resp = client.describe_regions().send().await;
    match resp {
        Ok(result) => {
            for region in result.regions.unwrap_or_default() {
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
        Err(err) => {
            println!("Error: {:?}", err);
            Err(err.into())
        }
    }

    // let result = resp?;
}

pub async fn enable_monitoring(client: &Client, id: &str) -> Result<(), Error> {
    let res = client.monitor_instances().instance_ids(id).send().await;
    match res {
        Ok(result) => {
            println!("Enabled monitoring: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Monitoring failed: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn reboot_instance(client: &Client, id: &str) -> Result<(), Error> {
    let res = client.reboot_instances().instance_ids(id).send().await;
    match res {
        Ok(result) => {
            println!("Enabled monitoring: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error rebooting instance: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn start_instance(client: &Client, id: &str) -> Result<(), Error> {
    let res = client.start_instances().instance_ids(id).send().await;
    match res {
        Ok(result) => {
            println!("Enabled monitoring: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error rebooting instance: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn stop_instance(client: &Client, id: &str) -> Result<(), Error> {
    let res = client.stop_instances().instance_ids(id).send().await;
    match res {
        Ok(result) => {
            println!("Enabled monitoring: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error rebooting instance: {:?}", e);
            Err(e.into())
        }
    }
}
