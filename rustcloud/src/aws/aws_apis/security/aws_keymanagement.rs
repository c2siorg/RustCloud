#![allow(clippy::result_large_err)]

use aws_sdk_kms::{types::{KeySpec, KeyUsageType, OriginType, Tag}, Client, Error};


pub async fn create_key(client: &Client, policy: String,description: Option<String>,key_usage: Option<KeyUsageType>,key_spec: Option<KeySpec>,origin: Option<OriginType>,custom_key_store_id: Option<String>,bypass_policy_lockout_safety_check: Option<bool>,tags: Option<Vec<Tag>>,multi_region: Option<bool>,xks_key_id: Option<String>) -> Result<(), Error> {
    let createkey = client.create_key().policy(policy).set_description(description).set_key_usage(key_usage).set_origin(origin).set_key_spec(key_spec).set_custom_key_store_id(custom_key_store_id).set_bypass_policy_lockout_safety_check(bypass_policy_lockout_safety_check).set_tags(tags).set_multi_region(multi_region).set_xks_key_id(xks_key_id).send().await?;
    println!("createkey: {:?}", createkey);
    Ok(())
}

pub async fn delete_key(client: &Client, custom_key_store_id: String) -> Result<(), Error> {
    let deletekey = client.delete_custom_key_store().custom_key_store_id(custom_key_store_id).send().await?;
    println!("deletekey: {:?}", deletekey);
    Ok(())
}

pub async fn describe_key(client: &Client, key_id: String, grant_tokens: Option<Vec<String>>) -> Result<(), Error> {
    let describekey = client.describe_key().key_id(key_id).set_grant_tokens(grant_tokens).send().await?;
    println!("describekey: {:?}", describekey);
    Ok(())
}

pub async fn put_key_policy(client: &Client, key_id: String, policy_name: String, policy: String, bypass_policy_lockout_safety_check: Option<bool>) -> Result<(), Error> {
    let putkeypolicy = client.put_key_policy().key_id(key_id).policy(policy).policy_name(policy_name).set_bypass_policy_lockout_safety_check(bypass_policy_lockout_safety_check).send().await?;
    println!("putkeypolicy: {:?}", putkeypolicy);
    Ok(())
}

pub async fn update(client: &Client, key_id: String, description: Option<String>) -> Result<(), Error> {
    let update = client.update_key_description().key_id(key_id).set_description(description).send().await?;
    println!("update: {:?}", update);
    Ok(())
}