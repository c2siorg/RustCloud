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
        .path(path)
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

pub async fn create_user(
    client: &Client,
    user_name: String,
    path: Option<String>,
    tags: Option<Vec<aws_sdk_iam::types::Tag>>,
) -> Result<(), Error> {
    let mut req = client.create_user().user_name(user_name);
    if let Some(p) = path {
        req = req.path(p);
    }
    if let Some(t) = tags {
        req = req.set_tags(Some(t));
    }
    let resp = req.send().await;
    match resp {
        Ok(result) => {
            println!("createuser: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn delete_user(client: &Client, user_name: String) -> Result<(), Error> {
    let resp = client.delete_user().user_name(user_name).send().await;
    match resp {
        Ok(result) => {
            println!("deleteuser: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn list_users(
    client: &Client,
    path_prefix: Option<String>,
    marker: Option<String>,
    max_items: Option<i32>,
) -> Result<(), Error> {
    let resp = client
        .list_users()
        .set_path_prefix(path_prefix)
        .set_marker(marker)
        .set_max_items(max_items)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("listusers: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn attach_user_policy(
    client: &Client,
    user_name: String,
    policy_arn: String,
) -> Result<(), Error> {
    let resp = client
        .attach_user_policy()
        .user_name(user_name)
        .policy_arn(policy_arn)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("attachuserpolicy: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn detach_user_policy(
    client: &Client,
    user_name: String,
    policy_arn: String,
) -> Result<(), Error> {
    let resp = client
        .detach_user_policy()
        .user_name(user_name)
        .policy_arn(policy_arn)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("detachuserpolicy: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn create_role(
    client: &Client,
    role_name: String,
    assume_role_policy_document: String,
    path: Option<String>,
    description: Option<String>,
) -> Result<(), Error> {
    let mut req = client
        .create_role()
        .role_name(role_name)
        .assume_role_policy_document(assume_role_policy_document);
    if let Some(p) = path {
        req = req.path(p);
    }
    if let Some(d) = description {
        req = req.description(d);
    }
    let resp = req.send().await;
    match resp {
        Ok(result) => {
            println!("createrole: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn delete_role(client: &Client, role_name: String) -> Result<(), Error> {
    let resp = client.delete_role().role_name(role_name).send().await;
    match resp {
        Ok(result) => {
            println!("deleterole: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn list_roles(
    client: &Client,
    path_prefix: Option<String>,
    marker: Option<String>,
    max_items: Option<i32>,
) -> Result<(), Error> {
    let resp = client
        .list_roles()
        .set_path_prefix(path_prefix)
        .set_marker(marker)
        .set_max_items(max_items)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("listroles: {:?}", result);
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
