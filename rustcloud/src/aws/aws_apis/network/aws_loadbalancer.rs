#![allow(clippy::result_large_err)]

use aws_sdk_elasticloadbalancing::{
    types::{Listener, Tag},
    Client, Error,
};

pub async fn add_tags(client: &Client, load_balancer_name: String, tags: Tag) -> Result<(), Error> {
    let resp = client
        .add_tags()
        .load_balancer_names(load_balancer_name)
        .tags(tags)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("{:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn create(
    client: &Client,
    load_balancer_name: String,
    listeners: Option<Vec<Listener>>,
    availability_zones: Option<Vec<String>>,
    subnets: Option<Vec<String>>,
    security_groups: Option<Vec<String>>,
    scheme: Option<String>,
    tags: Option<Vec<Tag>>,
) -> Result<(), Error> {
    let resp = client
        .create_load_balancer()
        .load_balancer_name(load_balancer_name)
        .set_listeners(listeners)
        .set_availability_zones(availability_zones)
        .set_security_groups(security_groups)
        .set_scheme(scheme)
        .set_tags(tags)
        .set_subnets(subnets)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("{:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn delete(client: &Client, load_balancer_name: String) -> Result<(), Error> {
    let resp = client
        .delete_load_balancer()
        .load_balancer_name(load_balancer_name)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("{:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn describe(client: &Client, load_balancer_name: String) -> Result<(), Error> {
    let resp = client
        .describe_load_balancer_attributes()
        .load_balancer_name(load_balancer_name)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("{:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}
pub async fn list_load_balancers(
    client: &Client,
    load_balancer_names: Option<Vec<String>>,
    marker: Option<String>,
    page_size: Option<i32>,
) -> Result<(), Error> {
    let resp = client
        .describe_load_balancers()
        .set_load_balancer_names(load_balancer_names)
        .set_marker(marker)
        .set_page_size(page_size)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("{:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}
