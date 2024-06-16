#![allow(clippy::result_large_err)]

use aws_sdk_s3::{types::{BucketCannedAcl, CreateBucketConfiguration, ObjectOwnership, RequestPayer, ChecksumAlgorithm}, Client, Error};

async fn create_bucket(client: &Client, acl: BucketCannedAcl,bucket: String,create_bucket_configuration: CreateBucketConfiguration,grant_full_control: Option<String>,grant_read: Option<String>,grant_read_acp: Option<String>,grant_write: Option<String>,grant_write_acp: Option<String>,object_lock_enabled_for_bucket: Option<bool>,object_ownership: Option<ObjectOwnership>) -> Result<(), Error> {
    let createbucket = client.create_bucket().acl(acl).bucket(bucket).create_bucket_configuration(create_bucket_configuration).set_grant_full_control(grant_full_control).set_grant_read(grant_read).set_grant_read_acp(grant_read_acp).set_grant_write(grant_write).set_grant_write_acp(grant_write_acp).set_object_lock_enabled_for_bucket(object_lock_enabled_for_bucket).set_object_ownership(object_ownership).send().await?;
    println!("createbucket: {:?}", createbucket);
    Ok(())
}

async fn delete(client: &Client, bucket: String, expected_bucket_owner: Option<String>) -> Result<(), Error> {
    let delete = client.delete_bucket().bucket(bucket).set_expected_bucket_owner(expected_bucket_owner).send().await?;
    println!("delete: {:?}", delete);
    Ok(())
}


async fn delete_object(client: &Client, bucket: String, key: String, expected_bucket_owner: Option<String>, mfa: Option<String>, version_id: Option<String>, request_payer: Option<RequestPayer>) -> Result<(), Error> {
    let deleteobject = client.delete_object().bucket(bucket).key(key).set_expected_bucket_owner(expected_bucket_owner).set_mfa(mfa).set_version_id(version_id).set_request_payer(request_payer).send().await?;
    println!("deleteobject: {:?}", deleteobject);
    Ok(())
}


async fn list(client: &Client) -> Result<(), Error> {
    let list = client.list_buckets().send().await?;
    println!("list: {:?}", list);
    Ok(())
}

async fn upload(client: &Client, body: ::aws_smithy_types::byte_stream::ByteStream,bucket: Option<String>,content_length: Option<i64>,content_md5: Option<String>,checksum_algorithm: Option<ChecksumAlgorithm>,checksum_crc32: Option<String>,checksum_crc32_c: Option<String>,checksum_sha1: Option<String>,checksum_sha256: Option<String>,key: Option<String>,part_number: Option<i32>,upload_id: Option<String>,sse_customer_algorithm: Option<String>,sse_customer_key: Option<String>,sse_customer_key_md5: Option<String>,request_payer: Option<RequestPayer>,expected_bucket_owner: Option<String>) -> Result<(), Error> {
    let upload = client.upload_part().send().await?;
    println!("upload: {:?}", upload);
    Ok(())
}

