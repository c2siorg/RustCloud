#![allow(clippy::result_large_err)]

use aws_config::meta::region::RegionProviderChain;
use aws_sdk_ec2::{types::Tag, Client, Error};
// use clap::Parser;

pub async fn create_instance(client: &Client, ami_id: &str) -> Result<(), Error> {
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

pub async fn show_state(client: &Client, ids: Option<Vec<String>>) -> Result<(), Error> {
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

pub async fn show_all_events(client: &Client) -> Result<(), Error> {
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

pub async fn enable_monitoring(client: &Client, id: &str) -> Result<(), Error> {
    client.monitor_instances().instance_ids(id).send().await?;

    println!("Enabled monitoring");

    Ok(())
}

pub async fn reboot_instance(client: &Client, id: &str) -> Result<(), Error> {
    client.reboot_instances().instance_ids(id).send().await?;

    println!("Rebooted instance.");
    Ok(())
}

pub async fn start_instance(client: &Client, id: &str) -> Result<(), Error> {
    client.start_instances().instance_ids(id).send().await?;

    println!("Started instance.");

    Ok(())
}

pub async fn stop_instance(client: &Client, id: &str) -> Result<(), Error> {
    client.stop_instances().instance_ids(id).send().await?;

    println!("Stopped instance.");

    Ok(())
}
// fn mock_client() -> Client {
//     // Create a mock client for testing purposes
//     Client::new(&aws_config::load_from_env().await)
// }

// fn mock_run_instances_output() -> RunInstancesOutput {
//     // Create a mock RunInstancesOutput for testing purposes
//     RunInstancesOutput::builder()
//         .instances(
//             std::iter::once(aws_sdk_ec2::output::Instance::builder()
//                 .instance_id("mock-instance-id")
//                 .build())
//             .collect(),
//         )
//         .build()
// }

// fn mock_create_tags_output() -> CreateTagsOutput {
//     // Create a mock CreateTagsOutput for testing purposes
//     CreateTagsOutput::builder().build()
// }


// #[cfg(test)]


// #[test]
// fn test_create_instance_success() {
//     let client = mock_client();
//     let ami_id = "mock-ami-id";

//     // Mock the run_instances method
//     let mut run_instances_mock = client.run_instances().image_id(ami_id);
//     run_instances_mock = run_instances_mock
//         .instance_type(InstanceType::T1Micro)
//         .min_count(1)
//         .max_count(1)
//         .send()
//         .mocked(|_, _| Ok(mock_run_instances_output()));

//     // Mock the create_tags method
//     let mut create_tags_mock = client.create_tags();
//     create_tags_mock = create_tags_mock
//         .resources("mock-instance-id")
//         .tags(
//             Tag::builder()
//                 .key("Name")
//                 .value("From SDK Examples")
//                 .build(),
//         )
//         .send()
//         .mocked(|_, _| Ok(mock_create_tags_output()));

//     let result = create_instance(&client, ami_id).unwrap();
//     assert!(result.is_ok());
// }

// #[test]
// #[should_panic(expected = "No instances created.")]
// fn test_create_instance_no_instances_created() {
//     let client = mock_client();
//     let ami_id = "mock-ami-id";

//     // Mock the run_instances method to return an empty list of instances
//     let mut run_instances_mock = client.run_instances().image_id(ami_id);
//     run_instances_mock = run_instances_mock
//         .instance_type(InstanceType::T1Micro)
//         .min_count(1)
//         .max_count(1)
//         .send()
//         .mocked(|_, _| Ok(RunInstancesOutput::builder().build()));

//     let _result = create_instance(&client, ami_id).unwrap();
// }