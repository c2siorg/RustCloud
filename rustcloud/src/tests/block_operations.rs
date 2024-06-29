use aws_sdk_ec2::{Client, Config};
use aws_sdk_ec2::config::Region;
use aws_sdk_ec2::types::{VolumeType, VolumeAttributeName};
use crate::aws::aws_apis::storage::aws_block_storage::*;

#[tokio::test]
async fn test_create_volume() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let availability_zone = "us-east-1a".to_string();
    let size = Some(8); // 8 GiB
    let volume_type = Some(VolumeType::Gp2);
    let iops = Some(100);
    let encrypted = Some(false);
    let kms_key_id = None;

    let result = create(&client, availability_zone, size, volume_type, iops, encrypted, kms_key_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_delete_volume() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let volume_id = "vol-1234567890abcdef0".to_string(); // Replace with a valid volume ID

    let result = delete(&client, volume_id).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_describe_volume() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let volume_id = "vol-1234567890abcdef0".to_string(); // Replace with a valid volume ID
    let attribute = VolumeAttributeName::AutoEnableIo;

    let result = describe(&client, volume_id, attribute).await;
    assert!(result.is_ok());
}

#[tokio::test]
async fn test_list_volumes() {
    let config = Config::builder().region(Region::new("us-east-1")).build();
    let client = Client::from_conf(config);

    let volume_ids = None; // Optionally provide a list of volume IDs
    let filters = None; // Optionally provide filters
    let max_results = Some(10); // Limit the number of results
    let next_token = None; // Optionally provide a next token for pagination

    let result = list(&client, volume_ids, filters, max_results, next_token).await;
    assert!(result.is_ok());
}
