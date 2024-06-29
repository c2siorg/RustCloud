#![allow(clippy::result_large_err)]

use std::collections::HashMap;
use aws_sdk_eks::operation::create_cluster::{CreateClusterError};
use aws_sdk_eks::operation::delete_cluster::DeleteClusterError;
use aws_sdk_eks::{ Client, Error};
use aws_sdk_eks::types::{AmiTypes, CapacityTypes, KubernetesNetworkConfigRequest, LaunchTemplateSpecification, Logging, NodegroupScalingConfig, NodegroupUpdateConfig, RemoteAccessConfig, Taint, UpdateAccessConfigRequest, VpcConfigRequest};



#[tokio::main]
#[allow(clippy::result_large_err)]
pub async fn create_cluster(client: &Client, cluster_name: String,_subnet_id: Vec<String>, version: Option<String>, role_arn: Option<String>, resources_vpc_config: Option<VpcConfigRequest>, kubernetes_network_config: Option<KubernetesNetworkConfigRequest> ) -> Result<(), Error> {
    
    let cluster = client
        .create_cluster()
        .name(cluster_name)
        .set_role_arn(role_arn)
        .set_version(version)
        .set_resources_vpc_config(resources_vpc_config)
        .set_kubernetes_network_config(kubernetes_network_config)
        .send()
        .await?;
        println!("cluster created: {:?}", cluster);
        Ok(())
    // return cluster;
}



pub async fn create_node_group(client: &Client, cluster_name: String, nodegroup_name: String, disk_size: Option<i32>, scaling_config: Option<NodegroupScalingConfig>, subnets: Option<Vec<String>>, instance_types: Option<Vec<String>>, ami_type: Option<AmiTypes>, remote_access: Option<RemoteAccessConfig>,node_role: Option<String>, labels: Option<HashMap<String,String>>,taints: Option<Vec<Taint>>,tags: Option<HashMap<String, String>>, client_request_token: Option<String>, launch_template: Option<LaunchTemplateSpecification>,update_config: Option<NodegroupUpdateConfig>,capacity_type: Option<CapacityTypes>,version: Option<String>,release_version: Option<String> ) -> Result<(), Error> {

    let node_group = client.create_nodegroup()
    .cluster_name(cluster_name)
    .nodegroup_name(nodegroup_name)
    .set_disk_size(disk_size)
    .set_scaling_config(scaling_config)
    .set_subnets(subnets)
    .set_instance_types(instance_types)
    .set_ami_type(ami_type)
    .set_remote_access(remote_access)
    .set_node_role(node_role)
    .set_labels(labels)
    .set_tags(tags)
    .set_client_request_token(client_request_token)
    .set_taints(taints)
    .set_update_config(update_config)
    .set_capacity_type(capacity_type)
    .set_version(version)
    .set_launch_template(launch_template)
    .set_release_version(release_version)
    .send()
    .await?;

    println!("nodegroup created successfully: {:?}", node_group);
    Ok(())

}



pub async fn delete_nodegroup(client: &Client, cluster_name: String, nodegroup_name: String) -> Result<(), Error> {
    let nodegroup_deleted = client.delete_nodegroup().cluster_name(cluster_name).nodegroup_name(nodegroup_name).send().await?;
    println!("nodegroup deleted: {:?}", nodegroup_deleted);
 
    Ok(())
    
}

pub async fn describe_cluster(client: &Client, name: String) -> Result<(), Error> {
    let cluster = client.describe_cluster().name(name).send().await?;
    println!("cluster: {:?}", cluster);
    Ok(())
}

pub async fn describe_nodegroup(client: &Client, cluster_name: String, nodegroup_name: String) -> Result<(), Error> {
    let cluster = client.describe_nodegroup().cluster_name(cluster_name).nodegroup_name(nodegroup_name).send().await?;
    println!("nodegroup: {:?}", cluster);
    Ok(())
    
}


pub async fn delete_cluster(client: &Client, cluster_name: &str) -> Result<(), DeleteClusterError> {
    let cluster_deleted = client.delete_cluster().name(cluster_name).send().await;
    println!("cluster deleted: {:?}", cluster_deleted);
    Ok(())
}

pub async fn list_clusters(client: &Client, max_results: Option<i32>, include: Option<Vec<String>>) -> Result<(), Error> {

    let resp = client.list_clusters().set_max_results(max_results).set_include(include).send().await?;
    let clusters = resp.clusters();
    // ListClustersOutput
    println!("Found {} clusters:", clusters.len());

    for cluster in clusters {
        println!("  {}", cluster);
    }

    Ok(())
}

pub async fn list_nodegroups(client: &Client, cluster_name: String, max_results: Option<i32>) -> Result<(), Error> {
    let resp = client.list_nodegroups().cluster_name(cluster_name).set_max_results(max_results).send().await?;
    let nodegroups = resp.nodegroups();
    // ListClustersOutput
    println!("Found {} nodegroups:", nodegroups.len());

    for nodegroup in nodegroups {
        println!("  {}", nodegroup);
    }

    Ok(())
}

pub async fn update_tags(client: &Client, resource_arn: String, tags: Option<HashMap<String, String>>) -> Result<(), Error> {
    let update_tags = client.tag_resource().resource_arn(resource_arn).set_tags(tags).send().await?;
    println!("tags updated: {:?}", update_tags);
    Ok(())
}


pub async fn update_config(client: &Client, name: String, resources_vpc_config: Option<VpcConfigRequest>, logging: Option<Logging>, client_request_token: Option<String>, access_config: Option<UpdateAccessConfigRequest>) -> Result<(), Error> {
    let update_config = 
    client
    .update_cluster_config()
    .name(name)
    .set_resources_vpc_config(resources_vpc_config)
    .set_logging(logging)
    .set_client_request_token(client_request_token)
    .set_access_config(access_config)
    .send().await?;

    println!("config updated: {:?}", update_config);

    Ok(())
}




