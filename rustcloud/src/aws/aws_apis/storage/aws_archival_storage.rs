#![allow(clippy::result_large_err)]

use aws_sdk_glacier::{Client, Error};


async fn create_vault(client: &Client, vault_name: String, account_id: String) -> Result<(), Error> {
    let createvault = client.create_vault().vault_name(vault_name).account_id(account_id).send().await?;
    println!("createvault: {:?}", createvault);
    Ok(())
}


async fn delete_archive(client: &Client, account_id: String, vault_name: String, archive_id: String) -> Result<(), Error> {
    let deletearchive = client.delete_archive().account_id(account_id).vault_name(vault_name).archive_id(archive_id).send().await?;
    println!("deletearchive: {:?}", deletearchive);
    Ok(())
}

async fn delete_vault(client: &Client, account_id: String, vault_name: String) -> Result<(), Error> {
    let deletevault = client.delete_vault().account_id(account_id).vault_name(vault_name).send().await?;
    println!("deletevault: {:?}", deletevault);
    Ok(())
}


async fn upload(client: &Client, account_id: String, vault_name: String, archive_description: Option<String>, part_size: Option<String>) -> Result<(), Error> {
    let upload = client.initiate_multipart_upload().account_id(account_id).vault_name(vault_name).set_archive_description(archive_description).set_part_size(part_size).send().await?;
    println!("upload: {:?}", upload);
    Ok(())
}

async fn list(client: &Client,account_id: String, marker: Option<String>, limit: Option<i32>) -> Result<(), Error> {
    let list = client.list_vaults().account_id(account_id).set_marker(marker).set_limit(limit).send().await?;
    println!("list: {:?}", list);
    Ok(())
}





