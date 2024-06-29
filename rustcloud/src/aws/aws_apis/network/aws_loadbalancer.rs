#![allow(clippy::result_large_err)]

use aws_sdk_elasticloadbalancing::{types::{Listener, Tag}, Client, Error};


pub async fn add_tags(client: &Client, load_balancer_name: String, tags: Tag) -> Result<(), Error> {
    let add_tags = client.add_tags().load_balancer_names(load_balancer_name).tags(tags).send().await?;
    println!("{:?}", add_tags);
    Ok(())
}


pub async fn create(client: &Client, load_balancer_name: String,listeners: Option<Vec<Listener>>,availability_zones: Option<Vec<String>>,subnets: Option<Vec<String>>,security_groups: Option<Vec<String>>,scheme: Option<String>,tags: Option<Vec<Tag>>) -> Result<(), Error> {
    let create = client.create_load_balancer().load_balancer_name(load_balancer_name).set_listeners(listeners).set_availability_zones(availability_zones).set_security_groups(security_groups).set_scheme(scheme).set_tags(tags).set_subnets(subnets).send().await?;
    println!("{:?}", create);
    Ok(())
    
}

pub async fn delete(client: &Client, load_balancer_name: String) -> Result<(), Error> {
    let delete = client.delete_load_balancer().load_balancer_name(load_balancer_name).send().await?;
    println!("{:?}", delete);
    Ok(())
}

pub async fn describe(client: &Client, load_balancer_name: String) -> Result<(), Error> {
    let describe = client.describe_load_balancer_attributes().load_balancer_name(load_balancer_name).send().await?;
    println!("{:?}", describe);
    Ok(())
}
pub async fn list_load_balancers(client: &Client, load_balancer_names: String,marker: String,page_size: i32) -> Result<(), Error> {
    let list_load_balancers = client.describe_load_balancers().load_balancer_names(load_balancer_names).marker(marker).page_size(page_size).send().await?;
    println!("{:?}", list_load_balancers);
    Ok(())
}

