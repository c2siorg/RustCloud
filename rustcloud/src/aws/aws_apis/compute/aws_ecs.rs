#![allow(clippy::result_large_err)]

use aws_sdk_ecs::types::{ClusterConfiguration, ClusterField, ClusterSetting};
use aws_sdk_ecs::{types::Tag, Client, Error};

pub async fn create_cluster(
    client: &Client,
    name: &String,
    tags: Option<Vec<Tag>>,
    settings: Option<Vec<ClusterSetting>>,
    configuration: ClusterConfiguration,
    capacity_providers: Option<Vec<String>>,
) -> Result<(), Error> {
    println!("{name}");
    let res = client
        .create_cluster()
        .cluster_name(name)
        .set_tags(tags)
        .set_settings(settings)
        .configuration(configuration)
        .set_capacity_providers(capacity_providers)
        .send()
        .await;
    match res {
        Ok(result) => {
            println!("Created: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn delete_cluster(client: &Client, name: &String) -> Result<(), Error> {
    let res = client.delete_cluster().cluster(name).send().await;
    match res {
        Ok(result) => {
            println!("Deleted : {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn describe_cluster(
    client: &Client,
    clusters: Option<Vec<String>>,
    include: Option<Vec<ClusterField>>,
) -> Result<(), Error> {
    let res = client
        .describe_clusters()
        .set_clusters(clusters)
        .set_include(include)
        .send()
        .await;
    match res {
        Ok(result) => {
            let clusters = result.clusters();
            println!("Found {} clusters:", clusters.len());
            for cluster in clusters {
                println!("  {}", cluster.cluster_name().unwrap());
            }
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn show_clusters(
    client: &aws_sdk_ecs::Client,
    max_results: Option<i32>,
) -> Result<(), Error> {
    let res = client
        .list_clusters()
        .set_max_results(max_results)
        .send()
        .await;
    match res {
        Ok(result) => {
            let cluster_arns = result.cluster_arns();
            println!("Found {} clusters:", cluster_arns.len());

            let clusters = client
                .describe_clusters()
                .set_clusters(Some(cluster_arns.into()))
                .send()
                .await;

            if clusters.is_err() {
                println!("Error: {:?}", clusters.err());
                return Ok(());
            }
            let describe_cluster_result = clusters?;
            for cluster in describe_cluster_result.clusters() {
                println!("  ARN:  {}", cluster.cluster_arn().unwrap());
                println!("  Name: {}", cluster.cluster_name().unwrap());
            }
            Ok(())
        }
        Err(e) => {
            println!("Error: {:?}", e);
            Err(e.into())
        }
    }
}
