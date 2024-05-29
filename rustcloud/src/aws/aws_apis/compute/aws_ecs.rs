#![allow(clippy::result_large_err)]

use aws_sdk_ecs::types::{ClusterConfiguration, ClusterField, ClusterSetting};
use aws_sdk_ecs::{types::Tag, Client, Error};


async fn create_cluster(client: &Client, name: &String, tags: Option<Vec<Tag>>, settings : ClusterSetting,configuration : ClusterConfiguration, capacity_providers:  Option<Vec<String>>) -> Result<(), Error> {
    let cluster = client.create_cluster().cluster_name(name).set_tags(tags).settings(settings).configuration(configuration).set_capacity_providers(capacity_providers).send().await?;
    println!("cluster created: {:?}", cluster);

    Ok(())
}



async fn delete_cluster(
    client: &Client,
    name: &String,
) -> Result<(), Error> {
    let cluster_deleted = client.delete_cluster().cluster(name).send().await?;
    println!("cluster deleted: {:?}", cluster_deleted);
    
    Ok(())
}


async fn describe_cluster(client: &Client, clusters: Option<Vec<String>>, include:Option<Vec<ClusterField>>) -> Result<(), Error> {

    let res = client.describe_clusters().set_clusters(clusters).set_include(include).send().await?;

    let clusters = res.clusters();
    println!("Found {} clusters:", clusters.len());
    for cluster in clusters {
        println!("  {}", cluster.cluster_name().unwrap());
    }
    Ok(())
}



async fn show_clusters(client: &aws_sdk_ecs::Client, max_results: Option<i32>) -> Result<(), Error> {
    let resp = client.list_clusters().set_max_results(max_results).send().await?;

    let cluster_arns = resp.cluster_arns();
    println!("Found {} clusters:", cluster_arns.len());

    let clusters = client
        .describe_clusters()
        .set_clusters(Some(cluster_arns.into()))
        .send()
        .await?;

    for cluster in clusters.clusters() {
        println!("  ARN:  {}", cluster.cluster_arn().unwrap());
        println!("  Name: {}", cluster.cluster_name().unwrap());
    }

    Ok(())
}



