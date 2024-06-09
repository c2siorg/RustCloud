#![allow(clippy::result_large_err)]

use aws_sdk_iam::{Client, Error};


async fn attach_group_policy(client: &Client, group_name: String, policy_arn: String) -> Result<(), Error> {
    let attachgrouppolicy = client.attach_group_policy().group_name(group_name).policy_arn(policy_arn).send().await?;
    println!("attachgrouppolicy: {:?}", attachgrouppolicy);
    Ok(())

}

pub struct CreateGroupInput {
    pub path: Option<String>,
    pub group_name: Option<String>,
}

async fn create_group(client: &Client, path: String, group_name: String) -> Result<(), Error> {
    let creategroup = client.create_group().group_name(path).group_name(group_name).send().await?;
    println!("creategroup: {:?}", creategroup);
    Ok(())
}

async fn delete_group(client: &Client, group_name: String) -> Result<(), Error> {
    let deletegroup = client.delete_group().group_name(group_name).send().await?;
    println!("deletegroup: {:?}", deletegroup);
    Ok(())
}

async fn detach_group_policy(client: &Client, group_name: String, policy_arn: String) -> Result<(), Error> {
    let detachgrouppolicy = client.detach_group_policy().group_name(group_name).policy_arn(policy_arn).send().await?;
    println!("detachgrouppolicy: {:?}", detachgrouppolicy);
    Ok(())
}

async fn describe(client: &Client, group_name: String, marker: String, max_items: i32) -> Result<(), Error> {
    let describe = client.get_group().group_name(group_name).marker(marker).max_items(max_items).send().await?;
    println!("describe: {:?}", describe);
    Ok(())
}
