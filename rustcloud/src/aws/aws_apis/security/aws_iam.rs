#![allow(clippy::result_large_err)]

use aws_sdk_iam::{Client, Error};

pub async fn attach_group_policy(
    client: &Client,
    group_name: String,
    policy_arn: String,
) -> Result<(), Error> {
    let resp = client
        .attach_group_policy()
        .group_name(group_name)
        .policy_arn(policy_arn)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("attachgrouppolicy: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn create_group(client: &Client, path: String, group_name: String) -> Result<(), Error> {
    let resp = client
        .create_group()
        .group_name(path)
        .group_name(group_name)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("creategroup: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn delete_group(client: &Client, group_name: String) -> Result<(), Error> {
    let resp = client.delete_group().group_name(group_name).send().await;
    match resp {
        Ok(result) => {
            println!("deletegroup: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn detach_group_policy(
    client: &Client,
    group_name: String,
    policy_arn: String,
) -> Result<(), Error> {
    let resp = client
        .detach_group_policy()
        .group_name(group_name)
        .policy_arn(policy_arn)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("detachgrouppolicy: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn describe(
    client: &Client,
    group_name: String,
    marker: Option<String>,
    max_items: Option<i32>,
) -> Result<(), Error> {
    let resp = client
        .get_group()
        .group_name(group_name)
        .set_marker(marker)
        .set_max_items(max_items)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("describe: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}
