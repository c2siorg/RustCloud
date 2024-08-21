use aws_sdk_ec2::types::{VolumeAttributeName, VolumeType};
use aws_sdk_ec2::{Client, Error};

pub async fn create(
    client: &Client,
    availability_zone: String,
    size: Option<i32>,
    volume_type: Option<VolumeType>,
    iops: Option<i32>,
    encrypted: Option<bool>,
    kms_key_id: Option<String>,
) -> Result<(), Error> {
    let resp = client
        .create_volume()
        .availability_zone(availability_zone)
        .set_size(size)
        .set_volume_type(volume_type)
        .set_iops(iops)
        .set_encrypted(encrypted)
        .set_kms_key_id(kms_key_id)
        .send()
        .await;
    match resp {
        Ok(result) => {
            println!("create: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}

pub async fn delete(client: &Client, volume_id: String) -> Result<(), Error> {
    let resp = client.delete_volume().volume_id(volume_id).send().await;
    match resp {
        Ok(result) => {
            println!("delete: {:?}", result);
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
    volume_id: String,
    attribute: VolumeAttributeName,
) -> Result<(), Error> {
    let resp = client
        .describe_volume_attribute()
        .volume_id(volume_id)
        .attribute(attribute)
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

pub async fn list(
    client: &Client,
    volume_ids: Option<Vec<String>>,
    filters: Option<Vec<aws_sdk_ec2::types::Filter>>,
    max_results: Option<i32>,
    next_token: Option<String>,
) -> Result<(), Error> {
    let mut request = client.describe_volumes();

    if let Some(ids) = volume_ids {
        request = request.set_volume_ids(Some(ids));
    }
    if let Some(f) = filters {
        request = request.set_filters(Some(f));
    }
    if let Some(max) = max_results {
        request = request.max_results(max);
    }
    if let Some(token) = next_token {
        request = request.next_token(token);
    }

    let resp = request.send().await;
    match resp {
        Ok(result) => {
            println!("descibe: {:?}", result);
            Ok(())
        }
        Err(e) => {
            println!("Error : {:?}", e);
            Err(e.into())
        }
    }
}
