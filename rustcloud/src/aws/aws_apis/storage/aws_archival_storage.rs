#![allow(clippy::result_large_err)]

use aws_sdk_glacier::{Client, Error};

pub async fn create_vault(
    client: &Client,
    vault_name: String,
    account_id: String,
) -> Result<(), Error> {
    let resp = client
        .create_vault()
        .vault_name(vault_name)
        .account_id(account_id)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("createvault: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn delete_archive(
    client: &Client,
    account_id: String,
    vault_name: String,
    archive_id: String,
) -> Result<(), Error> {
    let resp = client
        .delete_archive()
        .account_id(account_id)
        .vault_name(vault_name)
        .archive_id(archive_id)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("deletearchive: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn delete_vault(
    client: &Client,
    account_id: String,
    vault_name: String,
) -> Result<(), Error> {
    let resp = client
        .delete_vault()
        .account_id(account_id)
        .vault_name(vault_name)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("deletevault: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn upload(
    client: &Client,
    account_id: String,
    vault_name: String,
    archive_description: Option<String>,
    part_size: Option<String>,
) -> Result<(), Error> {
    let resp = client
        .initiate_multipart_upload()
        .account_id(account_id)
        .vault_name(vault_name)
        .set_archive_description(archive_description)
        .set_part_size(part_size)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("upload: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn list(
    client: &Client,
    account_id: String,
    marker: Option<String>,
    limit: Option<i32>,
) -> Result<(), Error> {
    let resp = client
        .list_vaults()
        .account_id(account_id)
        .set_marker(marker)
        .set_limit(limit)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("list: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}
